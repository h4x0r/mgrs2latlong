use anyhow::{Context, Result};
use clap::Parser;
use csv::{Reader, Writer};
use geoconvert::Mgrs;
use regex::Regex;
use std::fs::File;
use std::io::{self, BufReader, BufWriter};

#[derive(Parser)]
#[command(name = "mgrs2latlong")]
#[command(about = "Convert MGRS coordinates to latitude/longitude in CSV files")]
#[command(author = "Albert Hui <albert@securityronin.com>")]
struct Cli {
    #[arg(help = "Input CSV file path")]
    input: String,
    
    #[arg(short, long, help = "Output CSV file path (defaults to stdout)")]
    output: Option<String>,
}

fn is_likely_mgrs(value: &str) -> bool {
    let mgrs_pattern = Regex::new(r"(?i)\b\d{1,2}\s*[C-X]\s*[A-Z]{2}\s*\d{2,10}\b").unwrap();
    mgrs_pattern.is_match(value) && value.trim().len() >= 7
}

fn convert_mgrs_to_latlon(mgrs_str: &str) -> Result<(f64, f64)> {
    let normalized_mgrs = mgrs_str.replace(" ", "");
    let mgrs = Mgrs::parse_str(&normalized_mgrs)
        .with_context(|| format!("Failed to parse MGRS coordinate: {}", mgrs_str))?;
    
    let latlon = mgrs.to_latlon();
    Ok((latlon.latitude(), latlon.longitude()))
}

fn detect_mgrs_column(records: &[csv::StringRecord]) -> Option<usize> {
    if records.is_empty() {
        return None;
    }
    
    let num_columns = records[0].len();
    let mut column_scores = vec![0; num_columns];
    
    for record in records.iter().take(100) {
        for (col_idx, field) in record.iter().enumerate() {
            if is_likely_mgrs(field.trim()) {
                column_scores[col_idx] += 1;
            }
        }
    }
    
    column_scores
        .iter()
        .enumerate()
        .max_by_key(|&(_, score)| score)
        .filter(|&(_, score)| *score > 0)
        .map(|(idx, _)| idx)
}

fn process_csv(input_path: &str, output_path: Option<&str>) -> Result<()> {
    let file = File::open(input_path)
        .with_context(|| format!("Failed to open input file: {}", input_path))?;
    let mut reader = Reader::from_reader(BufReader::new(file));
    
    let mut records = Vec::new();
    let headers = reader.headers()?.clone();
    
    for result in reader.records() {
        let record = result.with_context(|| "Failed to read CSV record")?;
        records.push(record);
    }
    
    let mgrs_column = detect_mgrs_column(&records)
        .with_context(|| "No MGRS-like column detected in the CSV file")?;
    
    let output: Box<dyn io::Write> = match output_path {
        Some(path) => {
            let file = File::create(path)
                .with_context(|| format!("Failed to create output file: {}", path))?;
            Box::new(BufWriter::new(file))
        }
        None => Box::new(io::stdout()),
    };
    
    let mut writer = Writer::from_writer(output);
    
    let mut new_headers = headers.iter().collect::<Vec<_>>();
    new_headers.push("Latitude");
    new_headers.push("Longitude");
    writer.write_record(&new_headers)
        .with_context(|| "Failed to write headers")?;
    
    for record in &records {
        let mut new_record = record.iter().collect::<Vec<_>>();
        
        let mgrs_value = record.get(mgrs_column).unwrap_or("").trim();
        
        let (lat_str, lon_str) = if !mgrs_value.is_empty() && is_likely_mgrs(mgrs_value) {
            match convert_mgrs_to_latlon(mgrs_value) {
                Ok((lat, lon)) => (lat.to_string(), lon.to_string()),
                Err(_) => (String::new(), String::new())
            }
        } else {
            (String::new(), String::new())
        };
        
        new_record.push(&lat_str);
        new_record.push(&lon_str);
        
        writer.write_record(&new_record)
            .with_context(|| "Failed to write record")?;
    }
    
    writer.flush()
        .with_context(|| "Failed to flush output")?;
    
    println!("Processed {} records. MGRS column detected at index {}.", 
             records.len(), mgrs_column);
    
    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    process_csv(&cli.input, cli.output.as_deref())?;
    
    Ok(())
}