use html2md::parse_html_extended;
use std::{collections::HashMap, path::PathBuf};
use tera::{Result, Tera, Value, to_value, try_get_value};

/// Convert HTML to Markdown.
fn markdown(value: &Value, _: &HashMap<String, Value>) -> Result<Value> {
    let s = try_get_value!("markdown", "value", String, value);

    Ok(to_value(parse_html_extended(&s)).unwrap())
}

/// Read the template, and add any additional filters.
pub fn initialize_template(path: &PathBuf) -> Result<Tera> {
    let mut tera = Tera::default();
    tera.add_template_file(path, Some("template"))?;
    tera.register_filter("markdown", markdown);

    Ok(tera)
}
