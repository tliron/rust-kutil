use super::{error::*, provider::*};

use {axum_server::tls_rustls::*, std::sync::*};

impl TlsProvider {
    /// Creates an axum [RustlsAcceptor].
    pub fn axum_acceptor(&self) -> Result<RustlsAcceptor, TlsProviderError> {
        Ok(RustlsAcceptor::new(self.axum_config()?))
    }

    /// Creates an axum [RustlsConfig].
    pub fn axum_config(&self) -> Result<RustlsConfig, TlsProviderError> {
        Ok(RustlsConfig::from_config(Arc::new(self.http_server_config()?)))
    }
}
