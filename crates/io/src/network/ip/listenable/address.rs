use std::net::*;

//
// ListenableAddress
//

/// Listenable [IpAddr].
pub struct ListenableAddress {
    /// Address.
    pub address: IpAddr,

    /// Optional flowinfo for IPv6 address.
    pub flowinfo: Option<u32>,

    /// Optional scope ID for IPv6 address.
    pub scope: Option<u32>,
}

impl ListenableAddress {
    /// Constructor.
    pub fn new(address: IpAddr) -> Self {
        Self { address, flowinfo: None, scope: None }
    }

    /// Constructor.
    pub fn new_with_ipv6(address: IpAddr, flowinfo: Option<u32>, scope: Option<u32>) -> Self {
        Self { address, flowinfo, scope }
    }

    /// To [SocketAddr].
    pub fn to_socket_address(&self, port: u16) -> SocketAddr {
        match self.address {
            IpAddr::V6(address) => {
                // BROKEN: Scope is ignored when binding?
                // See: https://github.com/rust-lang/libs-team/issues/476#issuecomment-2825453898
                SocketAddrV6::new(address, port, self.flowinfo.unwrap_or_default(), self.scope.unwrap_or_default())
                    .into()
            }

            IpAddr::V4(address) => SocketAddrV4::new(address, port).into(),
        }
    }
}
