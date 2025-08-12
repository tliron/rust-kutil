use super::super::context::*;

use std::io::*;

/// Write debug heading.
pub fn write_debug_heading<WriteT, WriteNestedT>(
    heading: &str,
    writer: &mut WriteT,
    context: &DepictionContext,
    write_nested: WriteNestedT,
) -> Result<()>
where
    WriteT: Write,
    WriteNestedT: Fn(&mut WriteT, &DepictionContext) -> Result<()>,
{
    context.separate(writer)?;
    context.theme.write_heading(writer, heading)?;
    write_nested(writer, &context.child().with_inline(true).increase_indentation())
}
