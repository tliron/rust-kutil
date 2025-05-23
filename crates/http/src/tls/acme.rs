use super::{error::*, provider::*};

use {
    bytestring::*,
    rustls_acme::{caches::*, *},
    std::{io, path::*},
};

pub use acme::{LETS_ENCRYPT_PRODUCTION_DIRECTORY, LETS_ENCRYPT_STAGING_DIRECTORY};

impl TlsProvider {
    /// Add [ResolvesServerCertAcme] for all hosts.
    pub fn add_resolver_from_acme(&mut self, acme: ACME) -> Result<(), TlsProviderError> {
        let hosts = acme.hosts.clone();
        let resolver = acme.into_config().state().resolver();
        for host in hosts {
            self.add_delegate(host.clone(), resolver.clone())?;
        }
        Ok(())
    }
}

//
// ACME
//

/// ACME.
#[derive(Debug, Default)]
pub struct ACME {
    /// Hosts.
    pub hosts: Vec<ByteString>,

    /// Directory URL.
    pub directory: ByteString,

    /// Contacts (usually email addresses).
    pub contacts: Vec<ByteString>,

    /// Cache path.
    pub cache: PathBuf,
}

impl ACME {
    /// Into [AcmeConfig].
    pub fn into_config(self) -> AcmeConfig<io::Error> {
        let mut acme_config = AcmeConfig::new(self.hosts).directory(self.directory).cache(DirCache::new(self.cache));
        for contact in self.contacts {
            acme_config = acme_config.contact_push(String::from("mailto:") + &contact);
        }
        acme_config
    }
}
