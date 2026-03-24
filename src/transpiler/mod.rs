use std::fs;
use std::path::Path;
use tree_sitter::{Parser, Node};

pub fn transpile_file(input_path: &Path, output_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let source_code = fs::read_to_string(input_path)?;
    let rust_code = transpile_string(&source_code)?;
    let mut final_code = String::new();
    final_code.push_str("#[allow(unused_imports)]\n");
    final_code.push_str("use rusty_framework::prelude::*;\n\n");
    final_code.push_str(&rust_code);
    fs::write(output_path, final_code)?;
    Ok(())
}

pub fn transpile_file_to_string(input_path: &Path) -> Result<String, Box<dyn std::error::Error>> {
    let source_code = fs::read_to_string(input_path)?;
    transpile_string(&source_code)
}

pub fn transpile_string(source: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut parser = Parser::new();
    parser.set_language(tree_sitter_c_sharp::language())?;
    let tree = parser.parse(source, None).ok_or("Failed to parse")?;
    
    let mut out = String::new();
    let mut found_classes = Vec::new();
    transpile_node_recursive(tree.root_node(), source, &mut out, &mut found_classes)?;
    
    Ok(out)
}

fn transpile_node_recursive(node: Node, source: &str, out: &mut String, found_classes: &mut Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let kind = node.kind();
    
    if kind == "class_declaration" {
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
            if found_classes.contains(&class_name.to_string()) { return Ok(()); }
            found_classes.push(class_name.to_string());
            
            let mut signals = Vec::new();
            let mut members_out = String::new();
            find_members(node, source, &mut signals, &mut members_out);

            // Wrap in struct/impl
            out.push_str(&format!("#[derive(Debug)]\npub struct {} {{\n", class_name));
            for sig in &signals {
                out.push_str(&format!("    pub {}: Signal<{}>,\n", sig.name, sig.ty));
            }
            out.push_str("}\n\n");

            out.push_str(&format!("impl {} {{\n", class_name));
            out.push_str("    pub fn new() -> Self {\n");
            out.push_str("        Self {\n");
            for sig in &signals {
                let mut init = sig.init.clone();
                if sig.ty == "String" && (init.starts_with('"') || init == "String::new()") {
                    if !init.ends_with(".to_string()") && init != "String::new()" {
                        init = format!("{}.to_string()", init);
                    }
                }
                out.push_str(&format!("            {}: Signal::new({}),\n", sig.name, init));
            }
            out.push_str("        }\n");
            out.push_str("    }\n");
            out.push_str("}\n\n");

            out.push_str(&format!("impl Default for {} {{\n", class_name));
            out.push_str("    fn default() -> Self { Self::new() }\n");
            out.push_str("}\n\n");

            out.push_str(&format!("impl IvyApp for {} {{\n", class_name));
            out.push_str("    fn build(&self) -> Box<dyn Widget> {\n");
            out.push_str("        let app = self;\n");
            if members_out.is_empty() {
                out.push_str("        Box::new(Separator::new())\n");
            } else {
                out.push_str(&members_out);
                if !members_out.contains("return ") {
                    out.push_str("        Box::new(Separator::new())\n");
                }
            }
            out.push_str("    }\n\n");

            out.push_str("    fn update_state(&self, signal_id: &str, value: &str) {\n");
            for sig in &signals {
                out.push_str(&format!("        if signal_id == self.{}.id() {{\n", sig.name));
                if sig.ty == "String" {
                    out.push_str(&format!("            self.{}.set(value.to_string());\n", sig.name));
                } else if sig.ty == "bool" {
                    out.push_str(&format!("            if let Ok(val) = value.parse::<bool>() {{ self.{}.set(val); }}\n", sig.name));
                }
                out.push_str("        }\n");
            }
            out.push_str("    }\n");
            out.push_str("}\n\n");
            out.push_str(&format!("impl Widget for {} {{\n", class_name));
            out.push_str("    fn serialize(&self) -> serde_json::Value { self.build().serialize() }\n");
            out.push_str("}\n\n");
        }
    } else {
        for i in 0..node.child_count() {
            transpile_node_recursive(node.child(i).unwrap(), source, out, found_classes)?;
        }
    }
    Ok(())
}

struct SignalMember {
    name: String,
    ty: String,
    init: String,
}

fn find_members(node: Node, source: &str, signals: &mut Vec<SignalMember>, members_out: &mut String) {
    let kind = node.kind();
    if kind == "method_declaration" {
        let name_node = node.child_by_field_name("name").unwrap();
        let name = &source[name_node.start_byte()..name_node.end_byte()];
        if name == "Build" || name == "BuildSample" {
            // First pass: find signals in the method
            let mut temp_signals = Vec::new();
            collect_signals_in_node(node, source, &mut temp_signals);
            for s in temp_signals {
                if !signals.iter().any(|existing| existing.name == s.name) {
                    signals.push(s);
                }
            }

            if let Some(body) = node.child_by_field_name("body") {
                transpile_body(body, source, members_out, 2, signals);
            }
        }
    } else if kind == "field_declaration" {
        let decl_node = node.child_by_field_name("declaration").or_else(|| {
            for i in 0..node.child_count() {
                let c = node.child(i).unwrap();
                if c.kind() == "variable_declaration" { return Some(c); }
            }
            None
        }).unwrap_or(node.child(0).unwrap());
        if decl_node.kind() == "variable_declaration" {
            let mut temp_signals = Vec::new();
            collect_signals_in_node(decl_node, source, &mut temp_signals);
            for s in temp_signals {
                if !signals.iter().any(|existing| existing.name == s.name) {
                    signals.push(s);
                }
            }
        }
    } else {
        for i in 0..node.child_count() {
            find_members(node.child(i).unwrap(), source, signals, members_out);
        }
    }
}

fn collect_signals_in_node(node: Node, source: &str, signals: &mut Vec<SignalMember>) {
    let kind = node.kind();
    if kind == "variable_declaration" {
        let text = &source[node.start_byte()..node.end_byte()];
        if text.contains("UseState") || text.contains("Signal") {
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                if child.kind() == "variable_declarator" {
                    let id_node = child.child_by_field_name("name")
                        .or(child.child_by_field_name("identifier"))
                        .or(child.child(0))
                        .unwrap();
                    let name = &source[id_node.start_byte()..id_node.end_byte()];
                    
                    let mut init_text = String::new();
                    for j in 0..child.child_count() {
                        let grand = child.child(j).unwrap();
                        if grand.kind() == "equals_value_clause" {
                            init_text = source[grand.start_byte()..grand.end_byte()].to_string();
                        }
                    }

                    if !init_text.is_empty() {
                         let inner = if let Some(start) = init_text.find('(') {
                            let end = init_text.rfind(')').unwrap_or(init_text.len());
                            let content = &init_text[start+1..end];
                            if content.is_empty() || content == "(string)null" { "String::new()".to_string() }
                            else if content == "null" { "String::new()".to_string() }
                            else { content.to_string() }
                        } else { "String::new()".to_string() };
                        
                        let ty = if inner == "false" || inner == "true" { "bool" } else { "String" };
                        signals.push(SignalMember { name: name.to_string(), ty: ty.to_string(), init: inner });
                    }
                }
            }
        }
    } else {
        for i in 0..node.child_count() {
            collect_signals_in_node(node.child(i).unwrap(), source, signals);
        }
    }
}

fn transpile_body(node: Node, source: &str, out: &mut String, indent: usize, signals: &[SignalMember]) {
    if node.kind() == "arrow_expression_clause" {
        let pad = "    ".repeat(indent);
        out.push_str(&pad);
        out.push_str("return Box::new(");
        transpile_expr(node.child(1).unwrap(), source, out, indent, signals);
        out.push_str(");\n");
        return;
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        let kind = child.kind();
        let pad = "    ".repeat(indent);
        match kind {
            "return_statement" => {
                out.push_str(&pad);
                let expr = child.child_by_field_name("expression").or_else(|| {
                    if child.child_count() > 1 { Some(child.child(1).unwrap()) } else { None }
                });
                if let Some(expr) = expr {
                    out.push_str("return Box::new(");
                    transpile_expr(expr, source, out, indent, signals);
                    out.push_str(");\n");
                } else {
                    out.push_str("return Box::new(Separator::new());\n");
                }
            }
            "expression_statement" => { 
                let expr = child.child(0).unwrap();
                let text = &source[expr.start_byte()..expr.end_byte()];
                if !text.contains("UseEffect") && !text.contains("UseCollection") {
                    out.push_str(&pad);
                    transpile_expr(expr, source, out, indent, signals);
                    out.push_str(";\n"); 
                }
            }
            "local_declaration_statement" => {
                let text = &source[child.start_byte()..child.end_byte()];
                if !text.contains("UseState") {
                    let decl = child.child(0).unwrap(); // variable_declaration
                    if decl.kind() == "variable_declaration" {
                        for i in 0..decl.child_count() {
                            let declarator = decl.child(i).unwrap();
                            if declarator.kind() == "variable_declarator" {
                                out.push_str(&pad);
                                out.push_str("let ");
                                let name = declarator.child_by_field_name("name")
                                    .or(declarator.child_by_field_name("identifier"))
                                    .or(declarator.child(0))
                                    .unwrap();
                                out.push_str(&source[name.start_byte()..name.end_byte()]);
                                out.push_str(" = ");
                                
                                let mut found_value = false;
                                for j in 0..declarator.child_count() {
                                    let grand = declarator.child(j).unwrap();
                                    if grand.kind() == "equals_value_clause" {
                                        transpile_expr(grand.child(1).unwrap(), source, out, indent, signals);
                                        found_value = true;
                                    }
                                }
                                if !found_value {
                                    out.push_str("Default::default()");
                                }
                                out.push_str(";\n");
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

fn transpile_expr(node: Node, source: &str, out: &mut String, indent: usize, signals: &[SignalMember]) {
    let kind = node.kind();
    let pad = "    ".repeat(indent);
    match kind {
        "binary_expression" => {
            let left = node.child_by_field_name("left").unwrap();
            let op = &source[node.child_by_field_name("operator").unwrap().start_byte()..node.child_by_field_name("operator").unwrap().end_byte()];
            let right = node.child_by_field_name("right").unwrap();
            if op == "|" {
                transpile_expr(left, source, out, indent, signals);
                out.push_str(&format!("\n{}| ", pad));
                
                // If right side is a Signal.get() result (String), wrap it in Text::block
                let mut right_out = String::new();
                transpile_expr(right, source, &mut right_out, indent, signals);
                
                let is_widget = right_out.contains("::new") || right_out.contains("::center") || 
                               right_out.contains("::vertical") || right_out.contains("::horizontal") ||
                               right_out.contains("::grid") || right_out.contains("Text::") ||
                               right_out.contains("Layout::") || right_out.contains("Card::") ||
                               right_out.contains("TextInput::") || right_out.contains("BoolInput::") ||
                               right_out.contains("Separator::") || right_out.contains("Logo::") ||
                               right_out.contains("Badge::") || right_out.contains("Box::") ||
                               right_out.contains("Spacer::") || right_out.contains("Confetti::") ||
                               right_out.starts_with("(") || right_out.contains("app.build()");
                
                if !is_widget && (right_out.contains(".get()") || right_out.contains(".to_string()") || right_out.starts_with("format!")) {
                    out.push_str(&format!("Text::block({})", right_out));
                } else {
                    out.push_str(&right_out);
                }
            } else if op == "+" {
                out.push_str("format!(\"{}{}\", ");
                transpile_expr(left, source, out, indent, signals);
                out.push_str(", ");
                transpile_expr(right, source, out, indent, signals);
                out.push_str(")");
            } else {
                transpile_expr(left, source, out, indent, signals);
                out.push_str(&format!(" {} ", op));
                transpile_expr(right, source, out, indent, signals);
            }
        }
        "invocation_expression" => {
            let func_node = node.child_by_field_name("function").unwrap();
            let func = &source[func_node.start_byte()..func_node.end_byte()];
            let args = node.child_by_field_name("arguments").unwrap();
            
            if func == "Layout.Center" { out.push_str("Layout::center()"); }
            else if func == "Layout.Vertical" { out.push_str("Layout::vertical()"); }
            else if func == "Layout.Horizontal" { out.push_str("Layout::horizontal()"); }
            else if func == "Layout.Grid" { out.push_str("Layout::grid()"); }
            else if func.starts_with("Text.") {
                 let method = func.split('.').last().unwrap().to_lowercase();
                 out.push_str(&format!("Text::{}(", method));
                 transpile_args(args, source, out, false, signals);
                 out.push_str(")");
            }
            else if func.ends_with(".Gap") || func.ends_with(".Padding") || func.ends_with(".Width") || func.ends_with(".ToInput") ||
                     func.ends_with(".Placeholder") || func.ends_with(".Label") || func.ends_with(".Description") ||
                     func.ends_with(".Disabled") || func.ends_with(".Invalid") || func.ends_with(".Small") || func.ends_with(".Large") ||
                     func.ends_with(".Prefix") || func.ends_with(".Suffix") || func.ends_with(".Variant") || func.ends_with(".Icon") ||
                     func.ends_with(".MinLength") || func.ends_with(".MaxLength") || func.ends_with(".OnBlur") || func.ends_with(".OnSubmit") ||
                     func.ends_with(".ShortcutKey") || func.ends_with(".Columns") || func.ends_with(".ToTextInput") || func.ends_with(".ToPasswordInput") ||
                     func.ends_with(".ToTextareaInput") || func.ends_with(".ToSearchInput") || func.ends_with(".ToBoolInput") ||
                     func.ends_with(".ToSwitchInput") || func.ends_with(".ToToggleInput") {
                 let method = func.split('.').last().unwrap();
                 if method == "ToInput" || method == "ToTextInput" || method == "ToPasswordInput" || method == "ToTextareaInput" || method == "ToSearchInput" {
                     out.push_str("TextInput::new(");
                     transpile_expr(func_node.child(0).unwrap(), source, out, indent, signals);
                     out.push_str(".clone())");
                     if method == "ToPasswordInput" { out.push_str(".variant(\"Password\".to_string())"); }
                     else if method == "ToTextareaInput" { out.push_str(".variant(\"Textarea\".to_string())"); }
                     else if method == "ToSearchInput" { out.push_str(".variant(\"Search\".to_string())"); }
                     
                     if args.child_count() > 2 {
                          out.push_str(".placeholder(");
                          transpile_args(args, source, out, false, signals);
                          out.push_str(")");
                     }
                 } else if method == "ToBoolInput" || method == "ToSwitchInput" || method == "ToToggleInput" {
                     out.push_str("BoolInput::new(");
                     transpile_expr(func_node.child(0).unwrap(), source, out, indent, signals);
                     out.push_str(".clone())");
                     if method == "ToSwitchInput" { out.push_str(".variant(BoolInputVariant::Switch)"); }
                     else if method == "ToToggleInput" { 
                         out.push_str(".variant(BoolInputVariant::Toggle)"); 
                         if args.child_count() > 2 {
                             out.push_str(".icon(");
                             transpile_args(args, source, out, false, signals);
                             out.push_str(")");
                         }
                     }
                 } else {
                     let m = method.to_lowercase();
                     let base = func_node.child(0).unwrap();
                     transpile_expr(base, source, out, indent, signals);
                     out.push_str(&format!(".{}(", m));
                     if m == "placeholder" || m == "label" || m == "description" || m == "invalid" || m == "variant" || m == "prefix" || m == "suffix" || m == "density" {
                         transpile_args(args, source, out, false, signals);
                     } else if m == "columns" || m == "min_length" || m == "max_length" {
                         transpile_args(args, source, out, false, signals); // Ints
                     } else {
                         transpile_args(args, source, out, true, signals); // Floats
                     }
                     out.push_str(")");
                 }
            }
            else if func == "Size.Units" { transpile_args(args, source, out, true, signals); }
            else if func == "string.IsNullOrEmpty" {
                 out.push_str("(");
                 transpile_args(args, source, out, false, signals);
                 out.push_str(").is_empty()");
            }
            else if func == "string.IsNullOrWhiteSpace" {
                 out.push_str("(");
                 transpile_args(args, source, out, false, signals);
                 out.push_str(").trim().is_empty()");
            }
            else if func == "Card" || func == "Box" {
                out.push_str(&format!("{}::new(Box::new(", func));
                transpile_args(args, source, out, false, signals);
                out.push_str("))");
            }
            else if func == "Confetti" || func == "Badge" || func == "Spacer" || func == "Separator" {
                out.push_str(&format!("{}::new(", func));
                transpile_args(args, source, out, false, signals);
                out.push_str(")");
            }
            else if func == "Logo" {
                out.push_str("Logo::new(");
                if args.child_count() > 2 {
                    transpile_args(args, source, out, false, signals);
                } else {
                    out.push_str("LogoType::Ivy");
                }
                out.push_str(")");
            }
            else { 
                if func_node.child_count() > 0 && func.contains('.') {
                    // Method call on object
                    transpile_expr(func_node.child(0).unwrap(), source, out, indent, signals);
                    out.push_str(&format!(".{}(", func.split('.').last().unwrap().to_lowercase()));
                    transpile_args(args, source, out, false, signals);
                    out.push_str(")");
                } else {
                    let text = func;
                    if signals.iter().any(|s| s.name == text) {
                        out.push_str(&format!("app.{}", text));
                    } else {
                        out.push_str(&format!("/* Unknown: {} */ Separator::new()", func)); 
                    }
                }
            }
        }
        "object_creation_expression" => {
            let type_node = node.child_by_field_name("type").unwrap();
            let res = &source[type_node.start_byte()..type_node.end_byte()];
            if res.starts_with("List<") {
                out.push_str("Vec::new()");
            } else if res == "Separator" || res == "Spacer" || res == "Logo" { 
                out.push_str(&format!("{}::new(", res));
                if res == "Logo" { out.push_str("LogoType::Ivy"); }
                out.push_str(")");
            }
            else if res == "Badge" || res == "Box" || res == "TextBlock" || res == "Card" {
                out.push_str(&format!("{}::new(", res));
                if let Some(args) = node.child_by_field_name("arguments") {
                    if res == "Card" || res == "Box" { out.push_str("Box::new("); }
                    transpile_args(args, source, out, false, signals);
                    if res == "Card" || res == "Box" { out.push_str(")"); }
                }
                out.push_str(")");
            } else if res.starts_with("Signal<") {
                out.push_str("Signal::new(");
                if let Some(args) = node.child_by_field_name("arguments") {
                    transpile_args(args, source, out, false, signals);
                }
                out.push_str(")");
            }
            else { 
                out.push_str(&format!("{}::new()", res));
            }
        }
        "conditional_expression" => {
            let condition = node.child_by_field_name("condition").unwrap();
            let consequence = node.child_by_field_name("consequence").unwrap();
            let alternative = node.child_by_field_name("alternative").unwrap();
            out.push_str("if ");
            transpile_expr(condition, source, out, indent, signals);
            out.push_str(" { ");
            transpile_expr(consequence, source, out, indent, signals);
            out.push_str(" } else { ");
            transpile_expr(alternative, source, out, indent, signals);
            out.push_str(" }");
        }
        "cast_expression" => {
            let expr = node.child_by_field_name("expression").unwrap();
            transpile_expr(expr, source, out, indent, signals);
        }
        "member_access_expression" => {
            let name_node = node.child_by_field_name("name").unwrap();
            let name = &source[name_node.start_byte()..name_node.end_byte()];
            let expr_node = node.child_by_field_name("expression").unwrap();
            let expr = &source[expr_node.start_byte()..expr_node.end_byte()];
            
            if expr == "app" || expr == "this" {
                out.push_str(&format!("app.{}.get()", name));
            } else if expr == "Icons" {
                out.push_str(&format!("\"{}\".to_string()", name));
            } else if expr == "LogoType" {
                out.push_str(&format!("LogoType::{}", name));
            } else if expr == "BoolInputVariant" {
                out.push_str(&format!("BoolInputVariant::{}", name));
            } else if name == "Value" {
                transpile_expr(expr_node, source, out, indent, signals);
                out.push_str(".get()");
            } else {
                transpile_expr(expr_node, source, out, indent, signals);
                out.push_str(&format!(".{}", name));
            }
        }
        "identifier" | "identifier_name" => {
            let text = &source[node.start_byte()..node.end_byte()];
            if text == "null" { out.push_str("String::new()"); } // Default for strings
            else if signals.iter().any(|s| s.name == text) {
                out.push_str(&format!("app.{}", text));
            }
            else { out.push_str(text); }
        }
        "string_literal" => {
            out.push_str(&source[node.start_byte()..node.end_byte()]);
            out.push_str(".to_string()");
        }
        "boolean_literal" => {
            out.push_str(&source[node.start_byte()..node.end_byte()]);
        }
        "integer_literal" | "decimal_integer_literal" | "number_literal" => {
            out.push_str(&source[node.start_byte()..node.end_byte()]);
        }
        "parenthesized_expression" => {
            out.push_str("(");
            transpile_expr(node.child(1).unwrap(), source, out, indent, signals);
            out.push_str(")");
        }
        "interpolated_string_expression" => {
            out.push_str("format!(");
            let mut parts = Vec::new();
            let mut args = Vec::new();
            for i in 0..node.child_count() {
                let child = node.child(i).unwrap();
                if child.kind() == "interpolated_string_text" {
                    parts.push(source[child.start_byte()..child.end_byte()].to_string());
                } else if child.kind() == "interpolation" {
                    parts.push("{}".to_string());
                    let mut arg = String::new();
                    transpile_expr(child.child(1).unwrap(), source, &mut arg, indent, signals);
                    args.push(arg);
                }
            }
            out.push_str("\"");
            out.push_str(&parts.join(""));
            out.push_str("\", ");
            out.push_str(&args.join(", "));
            out.push_str(")");
        }
        _ => { 
            let text = &source[node.start_byte()..node.end_byte()];
            if text.chars().all(|c| c.is_numeric() || c == '.' || c == 'x' || c == 'X' || c == 'b' || c == 'B' || c == '_') {
                out.push_str(text);
            } else if text.starts_with('"') && text.ends_with('"') {
                out.push_str(&format!("{}.to_string()", text));
            }
            else {
                out.push_str(&format!("/* unknown {}: {} */", kind, text)); 
            }
        }
    }
}

fn transpile_args(node: Node, source: &str, out: &mut String, floats: bool, signals: &[SignalMember]) {
    let mut cursor = node.walk();
    let mut first = true;
    for child in node.children(&mut cursor) {
        if child.kind() == "argument" {
            if !first { out.push_str(", "); }
            let mut expr_idx = 0;
            if child.child(0).unwrap().kind() == "name_colon" {
                expr_idx = 1;
            }
            let expr = child.child(expr_idx).unwrap();
            let text = &source[expr.start_byte()..expr.end_byte()];
            if floats && text.parse::<f32>().is_ok() {
                out.push_str(&format!("{}.0", text));
            } else {
                transpile_expr(expr, source, out, 0, signals);
            }
            first = false;
        }
    }
}
