use super::super::context::*;

use std::io::*;

/// Write debug field.
pub fn write_debug_field<WriteT, WriteNestedT>(
    meta: &str,
    last: bool,
    writer: &mut WriteT,
    context: &DebugContext,
    write_nested: WriteNestedT,
) -> Result<()>
where
    WriteT: Write,
    WriteNestedT: Fn(&mut WriteT, &DebugContext) -> Result<()>,
{
    context.indent_into_branch(writer, last)?;
    context.theme.write_meta(writer, meta)?;
    context.theme.write_delimiter(writer, ":")?;
    write_nested(writer, &context.child().with_inline(true).with_separator(true).increase_indentation_branch(last))
}
