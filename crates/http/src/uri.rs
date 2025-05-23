use http::*;

//
// UriUtilities
//

/// [Uri] utilities.
pub trait UriUtilities {
    /// With new path.
    fn with_path(&self, path: &str) -> Result<Uri>;
}

impl UriUtilities for Uri {
    fn with_path(&self, path: &str) -> Result<Uri> {
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
    fn set_uri_path(&mut self, path: &str) -> Result<()>;
}

impl<BodyT> SetUri for Request<BodyT> {
    fn set_uri(&mut self, uri: Uri) {
        *self.uri_mut() = uri;
    }

    fn set_uri_path(&mut self, path: &str) -> Result<()> {
        let uri = self.uri().with_path(path)?;
        self.set_uri(uri);
        Ok(())
    }
}
