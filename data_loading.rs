// src/data_loading.rs

use csv;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;

pub struct DataRow {
    pub headers: Vec<String>,
    pub values: HashMap<String, String>,
}

pub fn load_csv(file_path: &str) -> Result<Vec<DataRow>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(File::open(file_path)?);
    let headers = rdr
        .headers()?
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let mut data = Vec::new();
    for result in rdr.records() {
        let record = result?;
        let mut row_data = HashMap::new();
        for (header, value) in headers.iter().zip(record.iter()) {
            row_data.insert(header.clone(), value.to_string());
        }
        data.push(DataRow {
            headers: headers.clone(),
            values: row_data,
        });
    }
    Ok(data)
}
