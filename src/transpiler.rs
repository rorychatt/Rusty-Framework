use std::fs;
use std::path::Path;
use regex::Regex;

pub fn transpile_file(input_path: &Path, output_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string(input_path)?;
    let mut rust_code = String::new();

    rust_code.push_str("use crate::prelude::*;\n\n");

    // Extract class name
    let class_re = Regex::new(r"public class (\w+) : ViewBase")?;
    let class_name = class_re.captures(&content)
        .map(|c| c[1].to_string())
        .unwrap_or_else(|| "App".to_string());

    rust_code.push_str(&format!("pub struct {};\n\n", class_name));
    rust_code.push_str(&format!("impl {} {{\n", class_name));
    rust_code.push_str("    pub fn build() -> Box<dyn Widget> {\n");

    // Extract Build() body (simplified)
    let build_re = Regex::new(r"public override object\? Build\(\)\s*\{([\s\S]*?)\}")?;
    if let Some(caps) = build_re.captures(&content) {
        let body = &caps[1];
        let transpiled_body = transpile_body(body);
        rust_code.push_str(&transpiled_body);
    }

    rust_code.push_str("    }\n");
    rust_code.push_str("}\n");

    fs::write(output_path, rust_code)?;
    Ok(())
}

fn transpile_body(body: &str) -> String {
    let mut result = String::new();
    
    for line in body.lines() {
        let line = line.trim();
        if line.is_empty() { continue; }

        // State: var nameState = this.UseState<string>(); -> let name_state = Signal::use_state("".to_string());
        let state_re = Regex::new(r"var (\w+) = this\.UseState<(\w+)>\(\);").unwrap();
        let line = state_re.replace_all(line, "let $1 = Signal::use_state(\"\".to_string());");

        // Layout: Layout.Center() -> Box::new(Layout::center())
        let line = line.replace("Layout.Center()", "Box::new(Layout::center())");
        let line = line.replace("Layout.Vertical()", "Layout::vertical()");
        
        // Gap/Padding: .Gap(6) -> .gap(6.0)
        let line = line.replace(".Gap(", ".gap(");
        let line = line.replace(".Padding(", ".padding(");

        // Card: new Card(...) -> Box::new(Card { content: ..., width: None })
        let line = line.replace("new Card(", "Box::new(Card { content: ");
        
        // Text: Text.H2(...) -> Box::new(Text { content: ..., style: Some("H2".to_string()) })
        let text_re = Regex::new(r"Text\.(\w+)\((.*?)\)").unwrap();
        let line = text_re.replace_all(&line, "Box::new(Text { content: $2.to_string(), style: Some(\"$1\".to_string()) })");

        // Separator: new Separator() -> Box::new(Separator)
        let line = line.replace("new Separator()", "Box::new(Separator)");

        // Size: Size.Units(120).Max(500) -> Some(120.0) (simplified)
        let size_re = Regex::new(r"Size\.Units\((\d+)\)(\.Max\(\d+\))?").unwrap();
        let line = size_re.replace_all(&line, "Some($1.0)");

        // Width: .Width(...) -> .width(...)
        let line = line.replace(".Width(", ".width(");

        // Handle C# ternary in strings (very basic)
        let ternary_re = Regex::new(r"\(string\.IsNullOrEmpty\((.*?)\) \? (.*?) : (.*?)\)").unwrap();
        let line = ternary_re.replace_all(&line, "if $1.get().is_empty() { $2 } else { $3 }");

        result.push_str(&format!("        {}\n", line));
    }

    result
}
