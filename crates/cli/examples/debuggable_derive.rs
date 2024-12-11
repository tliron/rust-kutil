use {kutil_cli::debug::*, std::collections::*};

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

    // "option" will show a bare "None" if the option is None
    // Also let's use std::fmt::Display here
    #[debuggable(option, as(display))]
    role: Option<String>,

    // We can skip fields
    #[debuggable(skip)]
    invisible: String,

    // Automagically iterate items with "-" prefix
    // (the annotations will be applied to the items)
    #[debuggable(iter(item), style(meta))]
    groups: Vec<String>,

    // Iterate as key-value pairs with "?" and ":" prefixes
    // Can use "key_as" and "key_style" for keys
    #[debuggable(iter(kv), as(display), key_as(display), key_style(bare))]
    meta: HashMap<String, String>,
}

#[derive(Debuggable, Default)]
#[allow(dead_code)]
// Branching style: thin (default), thick, or double
#[debuggable(branch(double))]
struct Credentials {
    #[debuggable(iter(kv))]
    meta: HashMap<String, String>,

    username: String,

    #[debuggable(style(error))]
    password: String,
}

pub fn main() {
    let user = User {
        name: "Tal".into(),
        age: 100,
        role: Some("admin".into()),
        credentials: Credentials {
            username: "root".into(),
            password: "12345".into(),
            meta: HashMap::from([("dangerous".into(), "very".into()), ("replace".into(), "asap".into())]),
        },
        groups: vec!["users".into(), "admins".into()],
        meta: HashMap::from([("personality".into(), "awesome".into()), ("athletic".into(), "kinda".into())]),
        ..Default::default()
    };

    user.print_debug();
}
