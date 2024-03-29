use std::fmt::Debug;
// use termcolor::{Color};

use tree_sitter::{Node, TreeCursor};
#[derive(Clone)]
pub struct NodeDetail<'a> {
    pub kind: &'a str,
    pub text: String,
    pub children: Vec<NodeDetail<'a>>,
}

impl<'a> NodeDetail<'a> {
    pub fn new(node: &Node, cursor: &mut TreeCursor, source_code: &str) -> Self {
        let kind = node.kind();
        let text = node.utf8_text(source_code.as_bytes()).unwrap().to_string();
        let mut children = Vec::new();
        if cursor.goto_first_child() {
            loop {
                let child = cursor.node();
                let child_node = NodeDetail::new(&child, cursor, source_code);
                children.push(child_node);
                if !cursor.goto_next_sibling() {
                    break;
                }
            }
            cursor.goto_parent();
        }
        Self {
            kind,
            text,
            children,
        }
    }

    pub fn get_children(&self) -> Vec<NodeDetail> {
        self.children.clone()
    }

    pub fn get_kind(&self) -> &str {
        self.kind.clone()
    }

    pub fn get_text(&self) -> String {
        self.text.clone()
    }

    pub fn dump_ast(&self, mut depth: usize) {
        let mut text = self.text.clone();
        if text.len() > 10 {
            text = format!("{}...", &text[..10]);
        }
        println!(
            "{} Type {}: | Value : {}",
            "-".repeat(depth * 2),
            self.kind,
            text
        );
        depth += 1;
        for child in &self.children {
            child.dump_ast(depth);
        }
    }
}

impl Debug for NodeDetail<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NodeDetail")
            .field("kind", &self.kind)
            .field("text", &self.text)
            .field("children", &self.children)
            .finish()
    }
}
