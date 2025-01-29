use super::{bind::*, tls::*};

use {
    axum_server::{bind_rustls, service::*, tls_rustls::*, *},
    http::Request,
    hyper::body::Incoming,
    std::{io, net::*},
};

impl Bind {
    /// Serve an await a service with Axum.
    pub async fn serve<MakeServiceT>(self, make_service: MakeServiceT) -> io::Result<()>
    where
        MakeServiceT: MakeService<SocketAddr, Request<Incoming>>,
    {
        let socket_address = self.socket_address()?;

        match self.tls {
            None => {
                tracing::info!("binding {:?} to {}", self.name, socket_address);
                bind(socket_address).serve(make_service).await
            }

            Some(tls) => {
                let rustls_config = tls.to_rustls_config().await?;
                tracing::info!("binding {:?} to {} (TLS)", self.name, socket_address);
                bind_rustls(socket_address, rustls_config).serve(make_service).await
            }
        }
    }
}

impl BindTLS {
    /// To Rustls configuration.
    pub async fn to_rustls_config(self) -> io::Result<RustlsConfig> {
        let certificate = self.certificate.get()?;
        let key = self.key.get()?;
        RustlsConfig::from_pem(certificate, key).await
    }
}
