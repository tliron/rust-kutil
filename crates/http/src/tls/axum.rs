use super::{container::*, error::*};

use {axum_server::tls_rustls::*, std::sync::*};

impl TlsContainer {
    /// Creates an axum [RustlsAcceptor].
    pub fn axum_acceptor(&self) -> Result<RustlsAcceptor, TlsContainerError> {
        Ok(RustlsAcceptor::new(self.axum_config()?))
    }

    /// Creates an axum [RustlsConfig].
    pub fn axum_config(&self) -> Result<RustlsConfig, TlsContainerError> {
        Ok(RustlsConfig::from_config(Arc::new(self.http_server_config()?)))
    }
}
