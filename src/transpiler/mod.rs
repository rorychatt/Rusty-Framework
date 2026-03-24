use std::fs;
use std::path::Path;
use tree_sitter::{Parser, Node};

pub fn transpile_file(input_path: &Path, output_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let source_code = fs::read_to_string(input_path)?;
    let rust_code = transpile_string(&source_code)?;
    fs::write(output_path, rust_code)?;
    Ok(())
}

pub fn transpile_string(source: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut parser = Parser::new();
    parser.set_language(tree_sitter_c_sharp::language())?;
    let tree = parser.parse(source, None).ok_or("Failed to parse")?;
    
    let mut out = String::new();
    out.push_str("#![allow(non_snake_case)]\n");
    out.push_str("use rusty_framework::prelude::*;\n\n");
    
    let mut found = false;
    transpile_node_recursive(tree.root_node(), source, &mut out, &mut found)?;
    
    Ok(out)
}

fn transpile_node_recursive(node: Node, source: &str, out: &mut String, found_class: &mut bool) -> Result<(), Box<dyn std::error::Error>> {
    let kind = node.kind();
    if kind == "class_declaration" && !*found_class {
        let mut name_node = None;
        for i in 0..node.child_count() {
            let child = node.child(i).unwrap();
            let k = child.kind();
            if k == "identifier" || k == "identifier_name" { name_node = Some(child); break; }
        }
        if let Some(n) = name_node {
            let class_name = &source[n.start_byte()..n.end_byte()];
            *found_class = true;
            out.push_str(&format!("pub struct {};\n\n", class_name));
            out.push_str(&format!("impl {} {{\n", class_name));
            for i in 0..node.child_count() {
                let child = node.child(i).unwrap();
                if child.kind() == "declaration_list" || child.kind() == "class_body" {
                    for j in 0..child.child_count() {
                        let member = child.child(j).unwrap();
                        if member.kind() == "method_declaration" {
                            process_method(member, source, out);
                        }
                    }
                }
            }
            out.push_str("}\n");
        }
    } else {
        for i in 0..node.child_count() {
            transpile_node_recursive(node.child(i).unwrap(), source, out, found_class)?;
            if *found_class { break; }
        }
    }
    Ok(())
}

fn process_method(member: Node, source: &str, out: &mut String) {
    let mut m_name_node = None;
    for k in 0..member.child_count() {
        let sub = member.child(k).unwrap();
        let sk = sub.kind();
        if sk == "identifier" || sk == "identifier_name" { m_name_node = Some(sub); break; }
    }
    if let Some(m_node) = m_name_node {
        let method_name = &source[m_node.start_byte()..m_node.end_byte()];
        if method_name == "Build" {
            out.push_str("    pub fn build() -> Box<dyn Widget> {\n");
            for k in 0..member.child_count() {
                let sub = member.child(k).unwrap();
                if sub.kind() == "block" || sub.kind() == "arrow_expression_clause" {
                    transpile_body(sub, source, out, 2);
                    break;
                }
            }
            out.push_str("    }\n");
        }
    }
}

fn transpile_body(node: Node, source: &str, out: &mut String, indent: usize) {
    for i in 0..node.child_count() {
        let child = node.child(i).unwrap();
        let kind = child.kind();
        let pad = "    ".repeat(indent);
        match kind {
            "block" => transpile_body(child, source, out, indent),
            "local_declaration_statement" => {
                let decl = child.child(0).unwrap();
                for j in 0..decl.child_count() {
                    let sub = decl.child(j).unwrap();
                    if sub.kind() == "variable_declarator" {
                        let n_node = sub.child_by_field_name("name").or_else(|| {
                             for k in 0..sub.child_count() {
                                 let c = sub.child(k).unwrap();
                                 if c.kind() == "identifier" { return Some(c); }
                             }
                             None
                        });
                        if let Some(name_node) = n_node {
                            let name = &source[name_node.start_byte()..name_node.end_byte()];
                            let init_node = sub.child_by_field_name("value").or_else(|| {
                                 for k in 0..sub.child_count() {
                                     let c = sub.child(k).unwrap();
                                     if c.kind() == "equals_value_clause" { return Some(c.child(1).unwrap()); }
                                 }
                                 None
                            });
                            if let Some(init) = init_node {
                                let init_text = &source[init.start_byte()..init.end_byte()];
                                if init_text.contains("UseState") {
                                    out.push_str(&format!("{}let {} = Signal::use_state(\"\".to_string());\n", pad, name));
                                }
                            }
                        }
                    }
                }
            }
            "return_statement" => {
                out.push_str(&format!("{}let result = ", pad));
                if let Some(expr) = child.child_by_field_name("expression").or_else(|| {
                      for k in 0..child.child_count() {
                          let c = child.child(k).unwrap();
                          if c.kind() != "return" && c.kind() != ";" { return Some(c); }
                      }
                      None
                }) {
                    transpile_expr(expr, source, out, indent + 1);
                }
                out.push_str(&format!(";\n{}Box::new(result)\n", pad));
            }
            "expression_statement" => { 
                transpile_body(child, source, out, indent); 
                out.push_str(";\n"); 
            }
            _ => {}
        }
    }
}

fn transpile_expr(node: Node, source: &str, out: &mut String, indent: usize) {
    let kind = node.kind();
    let pad = "    ".repeat(indent);
    match kind {
        "binary_expression" => {
            let left = node.child_by_field_name("left").unwrap();
            let op = &source[node.child_by_field_name("operator").unwrap().start_byte()..node.child_by_field_name("operator").unwrap().end_byte()];
            let right = node.child_by_field_name("right").unwrap();
            if op == "|" {
                transpile_expr(left, source, out, indent);
                out.push_str(&format!("\n{}| ", pad));
                transpile_expr(right, source, out, indent);
            } else if op == "+" {
                out.push_str("format!(\"{}{}\", ");
                transpile_expr(left, source, out, indent);
                out.push_str(", ");
                transpile_expr(right, source, out, indent);
                out.push_str(")");
            } else {
                transpile_expr(left, source, out, indent);
                out.push_str(&format!(" {} ", op));
                transpile_expr(right, source, out, indent);
            }
        }
        "invocation_expression" => {
            let func_node = node.child_by_field_name("function").unwrap();
            let func = &source[func_node.start_byte()..func_node.end_byte()];
            let args = node.child_by_field_name("arguments").unwrap();
            if func == "Layout.Center" { out.push_str("Layout::center()"); }
            else if func == "Layout.Vertical" { out.push_str("Layout::vertical()"); }
            else if func.starts_with("Text.") {
                 let method = func.split('.').last().unwrap().to_lowercase();
                 out.push_str(&format!("Text::{}(", method));
                 transpile_args(args, source, out, false);
                 out.push_str(")");
            }
            else if func.ends_with(".Gap") || func.ends_with(".Padding") || func.ends_with(".Width") {
                 let method = func.split('.').last().unwrap().to_lowercase();
                 transpile_expr(func_node.child(0).unwrap(), source, out, indent);
                 out.push_str(&format!(".{}(", method));
                 transpile_args(args, source, out, true);
                 out.push_str(")");
            }
            else if func == "Size.Units" { transpile_args(args, source, out, true); }
            else if func == "string.IsNullOrEmpty" {
                 out.push_str("(");
                 transpile_args(args, source, out, false);
                 out.push_str(").is_empty()");
            }
            else if func.ends_with(".ToInput") {
                out.push_str("Separator"); 
            }
            else { out.push_str("Separator"); }
        }
        "object_creation_expression" => {
            let type_node = node.child_by_field_name("type").unwrap();
            let name = &source[type_node.start_byte()..type_node.end_byte()];
            if name == "Card" {
                out.push_str("Card::new(Box::new(");
                if let Some(a) = node.child_by_field_name("arguments") { transpile_args(a, source, out, false); }
                out.push_str("))");
            } else if name == "Confetti" {
                out.push_str("Confetti::new(Box::new(");
                if let Some(a) = node.child_by_field_name("arguments") { transpile_args(a, source, out, false); }
                out.push_str("))");
            } else if name == "Logo" || name == "Separator" {
                out.push_str(name);
            } else { out.push_str("Separator"); }
        }
        "string_literal" | "verbatim_string_literal" | "integer_literal" | "identifier_name" | "identifier" | "predefined_type" => {
            let text = &source[node.start_byte()..node.end_byte()];
            out.push_str(text);
            if kind.contains("string") { out.push_str(".to_string()"); }
        }
        "parenthesized_expression" => {
            out.push_str("(");
            if let Some(expr) = node.child(1) { 
                if expr.kind() != "(" && expr.kind() != ")" {
                    transpile_expr(expr, source, out, indent); 
                }
            }
            out.push_str(")");
        }
        "conditional_expression" => {
            out.push_str("({ if ");
            transpile_expr(node.child_by_field_name("condition").unwrap(), source, out, indent);
            out.push_str(" { ");
            transpile_expr(node.child_by_field_name("consequence").unwrap(), source, out, indent);
            out.push_str(" } else { ");
            transpile_expr(node.child_by_field_name("alternative").unwrap(), source, out, indent);
            out.push_str(" } })");
        }
        "member_access_expression" => {
            transpile_expr(node.child_by_field_name("expression").unwrap(), source, out, indent);
            let name = &source[node.child_by_field_name("name").unwrap().start_byte()..node.child_by_field_name("name").unwrap().end_byte()];
            if name == "Value" { out.push_str(".get()"); } else { out.push_str(&format!(".{}", name)); }
        }
        _ => { out.push_str("Separator"); }
    }
}

fn transpile_args(args: Node, source: &str, out: &mut String, as_float: bool) {
    let mut first = true;
    for i in 0..args.child_count() {
        let child = args.child(i).unwrap();
        if child.kind() == "argument" {
            if !first { out.push_str(", "); }
            let mut expr = child.child(0).unwrap();
            if expr.kind() == "name_colon" {
                 expr = child.child(1).unwrap();
            }
            let text = &source[expr.start_byte()..expr.end_byte()];
            if as_float && text.chars().all(|c| c.is_digit(10)) {
                 out.push_str(&format!("{}.0", text));
            } else {
                 transpile_expr(expr, source, out, 0);
            }
            first = false;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_minimal_app() {
        let code = "public class MinApp : ViewBase { public override object Build() => Text.H2(\"Hi\"); }";
        let rust = transpile_string(code).unwrap();
        assert!(rust.contains("struct MinApp"));
    }
}
