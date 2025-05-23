use {
    bytestring::*,
    http::{uri::*, *},
    std::{borrow::*, collections::*, result::Result, string::*},
    url::form_urlencoded,
    urlencoding::*,
};

//
// PathAndQueryUtilities
//

/// [PathAndQuery] utilities.
pub trait PathAndQueryUtilities {
    /// Decoded path.
    fn decoded_path(&self) -> Option<Cow<'_, str>>;

    /// Decoded query.
    fn decoded_query(&self) -> Option<Vec<(Cow<'_, str>, Cow<'_, str>)>>;

    /// Decoded path.
    fn decoded_query_map(&self) -> Option<BTreeMap<ByteString, ByteString>> {
        self.decoded_query()
            .map(|query| query.iter().map(|(name, value)| (name.as_ref().into(), value.as_ref().into())).collect())
    }
}

impl PathAndQueryUtilities for PathAndQuery {
    fn decoded_path(&self) -> Option<Cow<'_, str>> {
        decode(self.path()).ok()
    }

    fn decoded_query(&self) -> Option<Vec<(Cow<'_, str>, Cow<'_, str>)>> {
        self.query().map(|query| form_urlencoded::parse(query.as_bytes()).collect())
    }
}

impl PathAndQueryUtilities for Uri {
    fn decoded_path(&self) -> Option<Cow<'_, str>> {
        self.path_and_query().and_then(|path_and_query| path_and_query.decoded_path())
    }

    fn decoded_query(&self) -> Option<Vec<(Cow<'_, str>, Cow<'_, str>)>> {
        self.path_and_query().and_then(|path_and_query| path_and_query.decoded_query())
    }
}

//
// UriUtilities
//

/// [Uri] utilities.
pub trait UriUtilities: PathAndQueryUtilities {
    /// With new path.
    fn with_path(&self, path: &str) -> Result<Uri, Error>;
}

impl UriUtilities for Uri {
    fn with_path(&self, path: &str) -> Result<Uri, Error> {
        //let mut path_and_query = encode(path).into_owned();
        let mut path_and_query = String::from(path);
        if let Some(query) = self.query() {
            path_and_query = path_and_query + "?" + query;
        }

        let mut builder = Self::builder().path_and_query(path_and_query);

        if let Some(scheme) = self.scheme() {
            builder = builder.scheme(scheme.clone());
        }

        if let Some(authority) = self.authority() {
            builder = builder.authority(authority.clone());
        }

        builder.build()
    }
}

//
// SetUri
//

/// Set [Uri].
pub trait SetUri {
    /// Set [Uri].
    fn set_uri(&mut self, uri: Uri);

    /// Set [Uri] path.
    fn set_uri_path(&mut self, path: &str) -> Result<(), Error>;
}

impl<BodyT> SetUri for Request<BodyT> {
    fn set_uri(&mut self, uri: Uri) {
        *self.uri_mut() = uri;
    }

    fn set_uri_path(&mut self, path: &str) -> Result<(), Error> {
        let uri = self.uri().with_path(path)?;
        self.set_uri(uri);
        Ok(())
    }
}
