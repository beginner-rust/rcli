use crate::cli::OutputFormat;
use anyhow::Result;
use csv::{Reader, StringRecord};
use serde_json::Value;
use std::fs;

pub fn process_csv(input: &str, output: String, format: OutputFormat) -> Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut ret: Vec<Value> = Vec::with_capacity(128);
    let headers: StringRecord = reader.headers()?.clone();
    for result in reader.records() {
        let record: StringRecord = result?;
        let json_value: Value = headers
            .iter()
            .zip(record.iter())
            .collect::<serde_json::Value>();
        ret.push(json_value);
    }

    let content: String = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&ret)?,
        OutputFormat::Yaml => serde_yaml::to_string(&ret)?,
    };

    fs::write(output, content)?;
    Ok(())
}
