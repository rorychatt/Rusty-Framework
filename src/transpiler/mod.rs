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
            if k == "identifier" || k == "identifier_name" { 
                if name_node.is_none() { name_node = Some(child); }
            }
        }
        
        if let Some(n) = name_node {
            let class_name = &source[n.start_byte()..n.end_byte()];
            *found_class = true;
            
            let mut signals = Vec::new();
            // Search deeply for fields and methods in this class
            find_members(node, source, &mut signals, out);

            // Wrap in struct/impl
            let members_code = out.clone();
            out.clear();
            out.push_str("#[allow(unused_imports)]\n");
            out.push_str("use rusty_framework::prelude::*;\n\n");
            out.push_str(&format!("pub struct {} {{\n", class_name));
            for s in signals.iter() {
                out.push_str(&format!("    pub {}: Signal<String>,\n", s));
            }
            out.push_str("}\n\n");
            out.push_str(&format!("impl {} {{\n", class_name));
            out.push_str("    pub fn new() -> Self {\n");
            out.push_str("        Self {\n");
            for s in signals.iter() {
                out.push_str(&format!("            {}: Signal::use_state(\"{}\".to_string(), \"\".to_string()),\n", s, s));
            }
            out.push_str("        }\n");
            out.push_str("    }\n");
            out.push_str("}\n\n");
            out.push_str(&format!("impl IvyApp for {} {{\n", class_name));
            out.push_str("    fn update_state(&self, signal_id: &str, value: &str) {\n");
            for s in signals.iter() {
                out.push_str(&format!("        if signal_id == \"{}\" {{ self.{}.set(value.to_string()); }}\n", s, s));
            }
            out.push_str("    }\n\n");
            out.push_str(&members_code);
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

fn find_members(node: Node, source: &str, signals: &mut Vec<String>, out: &mut String) {
    for i in 0..node.child_count() {
        let child = node.child(i).unwrap();
        let kind = child.kind();
        if kind == "field_declaration" {
            collect_signals(child, source, signals);
        } else if kind == "method_declaration" {
            process_method(child, source, out, signals);
        } else {
            find_members(child, source, signals, out);
        }
        // If we found a class nested inside, we don't want to transpile it as members of the outer class
        // but for this project we assume simple structures.
    }
}

fn collect_signals(member: Node, source: &str, signals: &mut Vec<String>) {
    // Search for variable_declarator anywhere inside field_declaration
    search_for_signals(member, source, signals);
}

fn search_for_signals(node: Node, source: &str, signals: &mut Vec<String>) {
    if node.kind() == "variable_declarator" {
        let mut name = None;
        let mut is_signal = false;
        for i in 0..node.child_count() {
            let c = node.child(i).unwrap();
            let ck = c.kind();
            if ck == "identifier" { name = Some(&source[c.start_byte()..c.end_byte()]); }
            else if ck == "equals_value_clause" {
                let text = &source[c.start_byte()..c.end_byte()];
                if text.contains("Signal") || text.contains("UseState") { is_signal = true; }
            }
        }
        if let (Some(n), true) = (name, is_signal) {
            signals.push(n.to_string());
        }
    } else {
        for i in 0..node.child_count() {
            search_for_signals(node.child(i).unwrap(), source, signals);
        }
    }
}

fn process_method(member: Node, source: &str, out: &mut String, signals: &[String]) {
    let mut m_name_node = None;
    for k in 0..member.child_count() {
        let sub = member.child(k).unwrap();
        let sk = sub.kind();
        if sk == "identifier" || sk == "identifier_name" { m_name_node = Some(sub); break; }
    }
    if let Some(m_node) = m_name_node {
        let method_name = &source[m_node.start_byte()..m_node.end_byte()];
        if method_name == "Build" {
            out.push_str("    #[allow(non_snake_case)]\n");
            out.push_str("    fn build(&self) -> Box<dyn Widget> {\n");
            for s in signals {
                out.push_str(&format!("        let {} = &self.{};\n", s, s));
            }
            for k in 0..member.child_count() {
                let sub = member.child(k).unwrap();
                if sub.kind() == "block" {
                    transpile_body(sub, source, out, 2);
                    break;
                } else if sub.kind() == "arrow_expression_clause" {
                    out.push_str("        let result = ");
                    transpile_expr(sub.child(1).unwrap(), source, out, 2);
                    out.push_str(";\n        Box::new(result)\n");
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
                         let mut name = None;
                         let mut is_signal = false;
                         for k in 0..sub.child_count() {
                             let c = sub.child(k).unwrap();
                             let ck = c.kind();
                             if ck == "identifier" { name = Some(&source[c.start_byte()..c.end_byte()]); }
                             else if ck == "equals_value_clause" {
                                 let text = &source[c.start_byte()..c.end_byte()];
                                 if text.contains("UseState") || text.contains("Signal") { is_signal = true; }
                             }
                         }
                         if let (Some(n), true) = (name, is_signal) {
                             out.push_str(&format!("{}let {} = Signal::use_state(\"\".to_string());\n", pad, n));
                         }
                    }
                }
            }
            "return_statement" => {
                out.push_str(&format!("\n{}let result = ", pad));
                if let Some(expr) = child.child_by_field_name("expression").or_else(|| {
                      for k in 0..child.child_count() {
                          let c = child.child(k).unwrap();
                          if c.kind() != "return" && c.kind() != ";" { return Some(c); }
                      }
                      None
                }) {
                    transpile_expr(expr, source, out, indent + 1);
                }
                out.push_str(&format!("\n        Box::new(result)\n"));
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
            else if func.ends_with(".Gap") || func.ends_with(".Padding") || func.ends_with(".Width") || func.ends_with(".ToInput") {
                 let method = func.split('.').last().unwrap();
                 if method == "ToInput" {
                     out.push_str("TextInput::new(");
                     transpile_expr(func_node.child(0).unwrap(), source, out, indent);
                     out.push_str(".clone())");
                     if args.child_count() > 2 {
                          out.push_str(".placeholder(");
                          transpile_args(args, source, out, false);
                          out.push_str(")");
                     }
                 } else {
                     let m = method.to_lowercase();
                     transpile_expr(func_node.child(0).unwrap(), source, out, indent);
                     out.push_str(&format!(".{}(", m));
                     transpile_args(args, source, out, true);
                     out.push_str(")");
                 }
            }
            else if func == "Size.Units" { transpile_args(args, source, out, true); }
            else if func == "string.IsNullOrEmpty" {
                 out.push_str("(");
                 transpile_args(args, source, out, false);
                 out.push_str(").is_empty()");
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
            } else if name.contains("Signal") {
                 out.push_str("Signal::use_state(\"\".to_string())");
            } else if name == "Logo" || name == "Separator" {
                out.push_str(name);
            } else { out.push_str("Separator"); }
        }
        "string_literal" | "verbatim_string_literal" | "integer_literal" | "identifier_name" | "identifier" | "predefined_type" => {
            let text = &source[node.start_byte()..node.end_byte()];
            out.push_str(text);
            if kind.contains("string") { out.push_str(".to_string()"); }
        }
        "interpolated_string_expression" => {
             out.push_str("format!(\"");
             let mut fmts = Vec::new();
             let mut arg_nodes = Vec::new();
             for i in 0..node.child_count() {
                let child = node.child(i).unwrap();
                let ck = child.kind();
                if ck == "interpolated_string_text" {
                    fmts.push(source[child.start_byte()..child.end_byte()].to_string());
                } else if ck == "interpolation" {
                    fmts.push("{}".to_string());
                    arg_nodes.push(child.child(1).unwrap());
                }
             }
             for f in fmts { out.push_str(&f); }
             out.push_str("\", ");
             for (i, a) in arg_nodes.iter().enumerate() {
                 if i > 0 { out.push_str(", "); }
                 transpile_expr(*a, source, out, 0);
             }
             out.push_str(")");
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
    fn test_field_collection() {
        let code = r#"public class App { 
            protected Signal<string> nameState = new Signal<string>(""); 
            public override object Build() => Text.H2("Hi");
        }"#;
        let rust = transpile_string(code).expect("Failed to transpile");
        assert!(rust.contains("let nameState = Signal::use_state"));
    }
}
