use super::context::*;

use std::io::*;

//
// DynDebuggable
//

/// A reduced `dyn`-compatible version of [Debuggable].
pub trait DynDebuggable {
    /// See [Debuggable::write_debug_for].
    fn write_debug_for(&self, writer: Box<&mut dyn Write>, context: &DebugContext) -> Result<()>;
}
