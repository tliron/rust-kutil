use {
    bytestring::*,
    kutil_std::collections::*,
    rustls::{server::*, sign::*},
    std::sync::*,
};

//
// SniResolverTarget
//

/// Target for [SniResolver].
#[derive(Clone, Debug)]
pub enum SniResolverTarget {
    /// Key.
    Key(Arc<CertifiedKey>),

    /// Delegate.
    Delegate(Arc<dyn ResolvesServerCert>),
}

//
// SniResolver
//

/// [ResolvesServerCert] that can select the target by client SNI (Server Name Indication).
///
/// Also has an optimized modes for when there is only one target.
#[derive(Clone, Debug)]
pub enum SniResolver {
    /// Select [SniResolverTarget] by SNI.
    BySNI(FastHashMap<ByteString, SniResolverTarget>),

    /// Single [SniResolverTarget].
    Single(SniResolverTarget),
}

impl ResolvesServerCert for SniResolver {
    fn resolve(&self, client_hello: ClientHello<'_>) -> Option<Arc<CertifiedKey>> {
        match self {
            Self::BySNI(targets) => match client_hello.server_name() {
                Some(sni) => match targets.get(sni) {
                    Some(target) => match target {
                        SniResolverTarget::Key(certified_key) => Some(certified_key.clone()),
                        SniResolverTarget::Delegate(resolver) => resolver.resolve(client_hello),
                    },
                    None => None,
                },
                None => None,
            },

            Self::Single(target) => match target {
                SniResolverTarget::Key(certified_key) => Some(certified_key.clone()),
                SniResolverTarget::Delegate(resolver) => resolver.resolve(client_hello),
            },
        }
    }
}
