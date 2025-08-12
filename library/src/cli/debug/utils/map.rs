use super::super::{context::*, debuggable::*, format::*};

use std::io::*;

/// Debug into map key.
pub const DEBUG_INTO_MAP_KEY: &str = "?";

/// Debug into map value.
pub const DEBUG_INTO_MAP_VALUE: &str = ":";

/// Debug into map entry.
pub const DEBUG_INTO_MAP_ENTRY: &str = "⚬"; // U+26AC

/// Debug map entry separator.
pub const DEBUG_MAP_ENTRY_SEPARATOR: &str = " ⇨"; // U+21E8

/// Write an [Iterator] of [Debuggable] as a map.
///
/// Supports [DebugFormat::Reduced] and [DebugFormat::Compact].
pub fn write_debug_as_map<'own, KeyT, ValueT, IteratorT, WriteT>(
    iterator: IteratorT,
    override_format: Option<DebugFormat>,
    writer: &mut WriteT,
    context: &DebugContext,
) -> Result<()>
where
    KeyT: 'own + Debuggable,
    ValueT: 'own + Debuggable,
    IteratorT: Iterator<Item = (&'own KeyT, &'own ValueT)>,
    WriteT: Write,
{
    let mut iterator = iterator.peekable();

    if iterator.peek().is_none() {
        context.separate(writer)?;
        return context.theme.write_delimiter(writer, "{}");
    }

    let format = match override_format {
        Some(format) => format,
        None => context.format.clone(),
    };

    match format {
        DebugFormat::Compact => {
            context.separate(writer)?;
            context.theme.write_delimiter(writer, "{")?;

            while let Some((key, value)) = iterator.next() {
                key.write_debug_for(writer, context)?;
                context.theme.write_delimiter(writer, ":")?;
                value.write_debug_for(writer, context)?;
                if iterator.peek().is_some() {
                    context.theme.write_delimiter(writer, ",")?;
                }
            }

            context.theme.write_delimiter(writer, "}")
        }

        DebugFormat::Reduced => {
            let key_context = context.child().with_separator(true).with_format(DebugFormat::Compact);
            let value_context = context.child().with_inline(true).with_separator(true).increase_indentation();

            let mut first = true;
            while let Some((key, value)) = iterator.next() {
                context.separate_or_indent_into(writer, DEBUG_INTO_MAP_ENTRY, first)?;
                key.write_debug_for(writer, &key_context)?;

                context.theme.write_delimiter(writer, DEBUG_MAP_ENTRY_SEPARATOR)?;
                value.write_debug_for(writer, &value_context)?;

                first = false;
            }

            Ok(())
        }

        DebugFormat::Verbose => {
            let child_context = context.child().with_separator(true).increase_indentation();

            let mut first = true;
            while let Some((key, value)) = iterator.next() {
                context.separate_or_indent_into(writer, DEBUG_INTO_MAP_KEY, first)?;
                key.write_debug_for(writer, &child_context)?;

                context.indent_into(writer, DEBUG_INTO_MAP_VALUE)?;
                value.write_debug_for(writer, &child_context)?;

                first = false;
            }

            Ok(())
        }
    }
}
