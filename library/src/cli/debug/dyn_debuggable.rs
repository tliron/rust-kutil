use super::{context::*, debuggable::*};

use std::io::*;

//
// DynDebuggable
//

/// A reduced `dyn`-compatible version of [Debuggable].
pub trait DynDebuggable {
    /// See [Debuggable::write_debug_for].
    fn dyn_write_debug_for(&self, writer: Box<&mut dyn Write>, context: &DebugContext) -> Result<()>;
}

impl<DebuggableT> DynDebuggable for DebuggableT
where
    DebuggableT: Debuggable,
{
    fn dyn_write_debug_for(&self, mut writer: Box<&mut dyn Write>, context: &DebugContext) -> Result<()> {
        self.write_debug_for(writer.as_mut(), context)
    }
}
