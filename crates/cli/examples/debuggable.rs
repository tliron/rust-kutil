use {kutil_cli::debug::*, kutil_std::iter::*, owo_colors::*, std::io::*};

// It's pretty easy to implement Debuggable manually

// See: examples/debuggable_derive.rs for how to automagically derive Debuggable

struct Node {
    name: String,
    children: Vec<Node>,
}

impl Node {
    fn new(name: &str) -> Self {
        Self::new_with(name, Vec::new())
    }

    fn new_with(name: &str, children: Vec<Node>) -> Self {
        Self { name: name.into(), children }
    }
}

impl Debuggable for Node {
    // You just need to implement this function

    fn write_debug_representation<WriteT>(&self, writer: &mut WriteT, prefix: &DebugPrefix, theme: &Theme) -> Result<()>
    where
        WriteT: Write,
    {
        // We'll use the provided theme
        write!(writer, "{}", self.name.style(theme.string))?;

        // DebugPrefix helps us follow the rules and build a recursive, nested horizontal tree
        for (child, last) in IterateWithLast::new(&self.children) {
            prefix.write_with_branch(writer, last)?;
            child.write_debug_representation(writer, &prefix.with_branch(last), theme)?;
        }

        Ok(())
    }
}

pub fn main() {
    let node = Node::new_with(
        "root",
        vec![
            Node::new_with(
                "first",
                vec![
                    Node::new_with("child1", vec![Node::new("grandchild1"), Node::new("grandchild2")]),
                    Node::new_with("child2", vec![Node::new("grandchild3"), Node::new("grandchild4")]),
                ],
            ),
            Node::new("second"),
            Node::new("third"),
        ],
    );

    node.print_debug();
}
