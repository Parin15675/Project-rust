// src/templating.rs

use std::collections::HashMap;

pub struct Template {
    pub visualization_type: String,
    pub columns: Vec<String>,
}

pub fn parse_template(template_str: &str) -> Template {
    // For simplicity, let's assume the template string is "type:table,columns:name,age"
    let parts: HashMap<_, _> = template_str
        .split(',')
        .map(|part| {
            let kv: Vec<_> = part.split(':').collect();
            (kv[0].to_string(), kv[1].to_string())
        })
        .collect();

    Template {
        visualization_type: parts["type"].clone(),
        columns: parts["columns"].split('|').map(|s| s.to_string()).collect(),
    }
}
