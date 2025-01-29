use super::tls::*;

use {
    compris::{parse::*, resolve::*, *},
    kutil_io::network::ip::*,
    kutil_std::sync::*,
    std::{io, net::*},
};

//
// Bind
//

/// Bind.
#[derive(Clone, Debug, Resolve)]
pub struct Bind {
    /// Index.
    pub index: usize,

    /// Name. Will default to a string representation of the index.
    #[resolve]
    pub name: String,

    /// Address.
    ///
    /// * If address is None, will bind to all reachable addresses (both IPv6 and IPv4).
    /// * If address is unspecified IPv6 (::0), will bind to all reachable IPv6 addresses.
    /// * If address is unspecified IPv4 (0.0.0.0), will bind to all reachable IPv4 addresses.
    #[resolve]
    pub address: Option<IpAddr>,

    /// Optional scope for IPv6 address.
    #[resolve]
    pub scope: Option<u32>,

    /// Whether to include loopbacks in reachable addresses. Default is true.
    #[resolve]
    pub include_loopbacks: bool,

    /// Port. Will default to 8080.
    #[resolve]
    pub port: u16,

    /// Optional TLS.
    #[resolve]
    pub tls: Option<BindTLS>,
}

static COUNTER: Counter = Counter::new();

impl Default for Bind {
    fn default() -> Self {
        let index = COUNTER.next();
        Self {
            index,
            name: index.to_string(),
            address: None,
            scope: None,
            include_loopbacks: true,
            port: 8080,
            tls: None,
        }
    }
}

impl Bind {
    /// Parse.
    pub fn parse(content: &str) -> io::Result<Self> {
        let value = Parser::new(Format::JSON)
            .with_try_unsigned_integers(true)
            .parse_from_string(content)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        tracing::info!("bind: {}", value);

        Ok(<normal::Value as Resolve<Bind, CommonResolveContext, CommonResolveError>>::resolve(&value)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
            .unwrap())
    }

    /// Resolve.
    pub fn resolve(self) -> io::Result<Vec<Self>> {
        let mut resolved = Vec::new();

        match to_reachable_addresses(&self.address, self.include_loopbacks)? {
            None => resolved.push(self),

            Some(addresses) => {
                if !addresses.is_empty() {
                    for address in addresses {
                        let mut bind = self.clone();
                        bind.address = Some(address);
                        resolved.push(bind);
                    }
                } else {
                    tracing::warn!("no reachable addresses found");
                }
            }
        }

        Ok(resolved)
    }

    /// Socket address.
    pub fn optional_socket_address(&self) -> Option<SocketAddr> {
        match self.address {
            Some(address) => match address {
                // BROKEN: Scope is ignored when binding?
                // See: https://github.com/rust-lang/libs-team/issues/476#issuecomment-2825453898
                IpAddr::V6(address) => {
                    Some(SocketAddrV6::new(address, self.port, 0, self.scope.unwrap_or_default()).into())
                }

                IpAddr::V4(address) => Some(SocketAddrV4::new(address, self.port).into()),
            },

            None => None,
        }
    }

    /// Socket address.
    pub fn socket_address(&self) -> io::Result<SocketAddr> {
        self.optional_socket_address()
            .ok_or(io::Error::new(io::ErrorKind::AddrNotAvailable, "address not provided for bind"))
    }
}
