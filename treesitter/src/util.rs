use tree_sitter::{Node, Tree, TreeCursor};

// count the number of nodes in the tree
pub fn count_nodes(node: &Node) -> usize {
    let mut count = 1;
    let mut cursor = node.walk();
    loop {
        if cursor.goto_first_child() {
            count += count_nodes(&cursor.node());
            while cursor.goto_next_sibling() {
                count += count_nodes(&cursor.node());
            }
            break;
        }
        if !cursor.goto_next_sibling() {
            break;
        }
    }
    count
}

pub fn find_main_node<'a>(cursor: &mut TreeCursor<'a>, source_code: &'a [u8]) -> Node<'a> {
    loop {
        let node = cursor.node();
        let kind = node.kind();

        if kind == "method_declaration" {
            let identifier_node = node.child_by_field_name("name").unwrap();
            let identifier_text = identifier_node.utf8_text(source_code).unwrap();

            if identifier_text == "main" {
                println!("Found main node");
                return node;
            }
        }

        if cursor.goto_first_child() {
            continue;
        } else {
            while !cursor.goto_next_sibling() {
                if !cursor.goto_parent() {
                    return node;
                }
            }
        }
    }
}


// print the AST of the tree to stdout
pub fn dump_ast(tree: &Tree, source_code: &[u8]) {
    let mut cursor = tree.walk();
    let mut depth = 0;
    loop {
        let node = cursor.node();
        let kind = node.kind();
        let mut text = node.utf8_text(source_code).unwrap().to_string();
        if text.len() > 20 {
            text = format!("{}...", &text[..20]);
        }
        println!("{: <width$}{} {}", "", kind, text, width = depth * 2);
        if cursor.goto_first_child() {
            depth += 1;
        } else {
            while !cursor.goto_next_sibling() {
                if !cursor.goto_parent() {
                    return;
                }
                depth -= 1;
            }
        }
    }
}

// action with all the nodes by function parameter
pub fn action_with_all_nodes<F>(node: &Node, cursor: &mut TreeCursor, source_code: & str, f: &mut F)
where
    F: FnMut(&Node, &mut TreeCursor, & str),
{
    f(node, cursor, source_code);
    if cursor.goto_first_child() {
        loop {
            let child = cursor.node();
            action_with_all_nodes(&child, cursor, source_code, f);
            if !cursor.goto_next_sibling() {
                break;
            }
        }
        cursor.goto_parent();
    }
}
