use test_rust_tree_siter::{
    model::node_detail::NodeDetail,
    util::{action_with_all_nodes, count_nodes, dump_ast, find_main_node},
};

use std::collections::LinkedList;

use tree_sitter::{Parser, Node};

#[derive(Debug)]
struct AstNode {
    node_type: String,
    value: String,
    children: Vec<AstNode>,
}


impl AstNode {
    // เพิ่มเมธอดสร้างโครงสร้างข้อมูลในรูปแบบของ AstNode
    fn new(node_type: &str, value: &str) -> AstNode {
        AstNode {
            node_type: node_type.to_string(),
            value: value.to_string(),
            children: Vec::new(),
        }
    }
}


fn main() {
    let source_code = r#"
    class Lab {
       public static void main(String[] args) {
           int x = 1;
           int y = foo(x);
           System.out.println(x + y);
       }

       int foo(){
           int x = 1;
           int y = foo2();
           return x + y;
       }

       int foo2(){
           int x = 1;
           int y = 0;
           return x + y;
        }
   }   
    "#;

    let mut parser = Parser::new();
    parser
        .set_language(tree_sitter_java::language())
        .expect("Error loading Java grammar");
    let tree = parser.parse(source_code, None).unwrap();
    let mut cursor = tree.walk();
    let root_node = &tree.root_node();

    let count = count_nodes(&root_node);
    println!("Number of nodes: {}", count);
    // dump_ast(&tree, &source_code.as_bytes());

    // let node_detail = NodeDetail::new(&root_node, &mut cursor, &mut source_code.to_string());
    // node_detail.dump_ast(0);

    let main_node = find_main_node(&mut cursor, &source_code.as_bytes());
    let main_node_detail = NodeDetail::new(&main_node, &mut cursor, &source_code.to_string());
    main_node_detail.dump_ast(0);

    println!("=====================================================");


    // let mut filtered_text_stack: Vec<String> = Vec::new();
    // filter_out_unwanted_chars(&main_node, &source_code, &mut filtered_text_stack);

    // // for item in &filtered_text_stack {
    // //     print!("------------------\n");
    // //     print!("\n{}", item);
    // // }
    
    // println!("{:?}", filtered_text_stack);


    // fn filter_out_unwanted_chars(node: &Node, source_code: &str, result_stack: &mut Vec<String>) {
    //     let mut cursor = node.walk();
    //     while cursor.goto_next_sibling() {
    //         let child_node = cursor.node();
    //         let child_text = child_node.utf8_text(source_code.as_bytes()).unwrap();
    //         if !["{", "}", "[", "]", "(", ")", ";", ":"].contains(&child_text) {
    //             result_stack.push(child_text.to_string());
    //         }
    //     }
    //     result_stack.push("".to_string());
        
    // }
    let mut filtered_text_list: LinkedList<String> = LinkedList::new();
    filter_out_unwanted_chars(&main_node, &source_code, &mut filtered_text_list);
    for item in &filtered_text_list {
        println!("------------------");
        println!("{}", item);
    }

    fn filter_out_unwanted_chars(node: &Node, source_code: &str, result_list: &mut LinkedList<String>) {
        let mut cursor = node.walk();
        let mut index = 0;
        
        loop {
    
            let child_node = cursor.node();
            let child_kind = child_node.kind();
            let child_text = child_node.utf8_text(source_code.as_bytes()).unwrap();
    
            if !["{", "}", "[", "]", "(", ")", ";", ":" , "." , "System" , "out" , "System.out"].contains(&child_text) {
                result_list.push_back(format!("---- Type {}: \n | Value : {}", child_kind, child_text));
            }
    
            if cursor.goto_first_child() {
                continue;
            }
    
            while !cursor.goto_next_sibling() {
                if !cursor.goto_parent() {
                    break;
                }
            }
    
            if cursor.node().kind().contains("method_declaration") {
                break;
            }
        }
    }

    for item in &filtered_text_list {
        println!("==================");
        println!("{}", item);
    }


    let mut filtered_text_list: LinkedList<String> = LinkedList::new();
    let ast_nodes = create_ast_from_filtered_list(&filtered_text_list);
    for node in ast_nodes {
        println!("{:#?}", node);
    }




}

fn create_ast_from_filtered_list(filtered_text_list: &LinkedList<String>) -> Vec<AstNode> {
    let mut ast_nodes: Vec<AstNode> = Vec::new();
    let mut stack: Vec<&mut AstNode> = Vec::new();

    for line in filtered_text_list {
        let (depth, content) = parse_line(line);

        let node_type = "custom_type"; // ปรับเป็นประเภทที่เหมาะสม

        let mut new_node = AstNode::new(node_type, content);

        while !stack.is_empty() && stack.len() >= depth {
            stack.pop();
        }

        if let Some(parent) = stack.last_mut() {
            parent.children.push(new_node);
        } else {
            ast_nodes.push(new_node);
        }

        stack.push(ast_nodes.last_mut().unwrap());
    }

    ast_nodes
}

    
fn parse_line(line: &str) -> (usize, String) {
    let depth = line.chars().take_while(|&c| c == '-').count();
    let content = line.chars().skip(depth * 2).collect::<String>().trim().to_string();
    (depth, content)
}


    

   




    // 1) แปลง source code เป็น AST
    // 2) แปลง AST เป็น NodeDetail (NodeDetail คือ struct ที่เก็บข้อมูลของ node และ children ของ node นั้น ๆ)
    // 3) ตัด Node ที่ไม่จำเป็นออกไป เช่น () , {} , ; และอื่น ๆ
    // 4) หา Node ที่เป็น main method
    // 5) แสดงผลลัพธ์ออกมา
    

    // let mut stack: Vec<&Node> = vec![];
    // stack.push(&main_node );
    // stack.push(&main_node );
    // stack.push(&main_node );
    // stack.push(&main_node );

    // println!("Stack: {:?}", stack.pop());

    // action_with_all_nodes(
    //     &root_node,
    //     &mut cursor,
    //     &source_code,
    //     &mut |node, cursor, source_code| {
    //         let node_detail = NodeDetail::new(&node, cursor, source_code);
    //         println!("\n\nNode kind: {}\nNode text: {}", node_detail.get_kind(),node_detail.get_text());
    //     },
    // );

