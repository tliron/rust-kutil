use super::super::{context::*, debuggable::*, format::*};

use std::io::*;

/// Debug into list item.
pub const DEBUG_INTO_LIST_ITEM: &str = "⦁︎"; // U+2981

/// Write an [Iterator] of [Debuggable] as a list.
///
/// Supports [DebugFormat::Compact].
pub fn write_debug_as_list<'own, ItemT, IteratorT, WriteT>(
    iterator: IteratorT,
    override_format: Option<DebugFormat>,
    writer: &mut WriteT,
    context: &DebugContext,
) -> Result<()>
where
    ItemT: Debuggable + 'own,
    IteratorT: Iterator<Item = &'own ItemT>,
    WriteT: Write,
{
    let mut iterator = iterator.peekable();

    if iterator.peek().is_none() {
        context.separate(writer)?;
        return context.theme.write_delimiter(writer, "[]");
    }

    let format = match override_format {
        Some(format) => format,
        None => context.format.clone(),
    };

    match format {
        DebugFormat::Compact => {
            context.separate(writer)?;
            context.theme.write_delimiter(writer, "[")?;

            let child_context = context.child().with_separator(false);

            while let Some(item) = iterator.next() {
                item.write_debug_for(writer, &child_context)?;
                if iterator.peek().is_some() {
                    context.theme.write_delimiter(writer, ",")?;
                }
            }

            context.theme.write_delimiter(writer, "]")
        }

        DebugFormat::Reduced | DebugFormat::Verbose => {
            let child_context = context.child().with_separator(true).increase_indentation();

            let mut first = true;
            for item in iterator {
                context.separate_or_indent_into(writer, DEBUG_INTO_LIST_ITEM, first)?;
                item.write_debug_for(writer, &child_context)?;

                first = false;
            }

            Ok(())
        }
    }
}
