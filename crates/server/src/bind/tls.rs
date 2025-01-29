use compris::resolve::*;

//
// BindTLS
//

/// Bind TLS.
#[derive(Clone, Debug, Default, Resolve)]
pub struct BindTLS {
    /// Certificate PEM.
    #[resolve]
    pub certificate: LoadableBlob,

    /// Key PEM.
    #[resolve]
    pub key: LoadableBlob,
}
