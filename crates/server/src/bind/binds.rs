use super::bind::*;

use std::io;

//
// Binds
//

/// Binds.
pub trait Binds {
    /// Parse and resolve.
    fn parse_and_resolve(&self) -> io::Result<Vec<Bind>>;
}

impl Binds for Vec<String> {
    fn parse_and_resolve(&self) -> io::Result<Vec<Bind>> {
        let mut binds = Vec::new();

        for bind in self {
            let bind = Bind::parse(bind)?;
            binds.extend(bind.resolve()?);
        }

        Ok(binds)
    }
}
