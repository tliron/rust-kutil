use {
    kutil_cli::debug::*,
    std::{collections::*, io},
};

// See: examples/debuggable.rs for how to manually implement Debuggable

// You need to enable the "derive" feature
#[derive(Debuggable, Default)]
#[allow(dead_code)]
struct User {
    // Without any annotation the field will be displayed using std::fmt::Debug
    name: String,

    // Add some style
    #[debuggable(style(number))]
    age: u64,

    // Delegate to a nested Debuggable (can be recursive)
    #[debuggable(as(debuggable))]
    credentials: Credentials,

    // "option" will show a "None" or "Some" symbol before the value
    // Also let's use std::fmt::Display here
    #[debuggable(option, as(display))]
    role: Option<String>,

    // We can skip fields
    #[debuggable(skip)]
    invisible: String,

    // Automagically iterate items with a pretty delimiter
    // (the annotations will be applied to the items)
    #[debuggable(iter(item), style(meta))]
    groups: Vec<String>,

    // "as(custom(...))" can be used for a custom stringifying function
    // ("uppercase" is defined below)
    #[debuggable(as(custom(uppercase)))]
    special: String,

    // Iterate as key-value pairs with pretty delimiters
    // Can use "key_as" and "key_style" for keys
    #[debuggable(iter(kv), as(display), key_as(display), key_style(symbol))]
    meta: HashMap<String, String>,
}

// Enums!
#[derive(Debuggable, Default)]
#[allow(dead_code)]
enum Credentials {
    #[default]
    Prompt,

    #[debuggable(option, as(display), style(string))]
    LoadFrom(Option<String>),

    #[debuggable(as(debuggable))]
    Provided(ProvidedCredentials),
}

#[derive(Debuggable, Default)]
#[allow(dead_code)]
// Branching style: thin (default), thick, or double
// We also show the use of "tag" here to add a custom tag (appears after the generated output)
// (Tags can be used on structs as well as individual fields or enum variants)
#[debuggable(branch(double), tag(safety))]
struct ProvidedCredentials {
    #[debuggable(iter(kv))]
    meta: HashMap<String, String>,

    username: String,

    #[debuggable(style(error), tag(safety))]
    password: String,
}

pub fn main() {
    let user = User {
        name: "Tal".into(),
        age: 100,
        role: Some("admin".into()),
        // credentials: Credentials::LoadFrom(Some("hi".into())),
        credentials: Credentials::Provided(ProvidedCredentials {
            username: "root".into(),
            password: "12345".into(),
            meta: HashMap::from([("dangerous".into(), "very".into()), ("replace".into(), "asap".into())]),
        }),
        groups: vec!["users".into(), "admins".into()],
        special: "this is special".into(),
        meta: HashMap::from([("personality".into(), "awesome".into()), ("athletic".into(), "kinda".into())]),
        ..Default::default()
    };

    user.print_debug();
}

fn uppercase(special: &str) -> io::Result<String> {
    Ok(special.to_uppercase())
}

// Custom "safety" tag
fn safety<WriteT>(
    provided_credentials: &ProvidedCredentials,
    _field_name: &str,
    writer: &mut WriteT,
    _context: &DebugContext,
) -> io::Result<()>
where
    WriteT: io::Write,
{
    if provided_credentials.password.is_empty() {
        write!(writer, " unsafe password!")
    } else {
        write!(writer, " safe password!")
    }
}
