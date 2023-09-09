use tree_sitter::{Node, Parser, Point, Query, QueryCursor};

pub enum Position {
    AttributeName(String),
    AttributeValue { name: String, value: String },
}

fn get_text(node: Node<'_>, source: &str) -> String {
    return node
        .utf8_text(source.as_bytes())
        .expect("getting text shouldn't fail")
        .to_string();
}
