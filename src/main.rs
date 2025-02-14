use chrono::{DateTime, Local, TimeZone, Utc};
use clap::Parser;
use directories::ProjectDirs;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "ishango")]
enum Opt {
    #[command(name = "init")]
    Init { bucket: String },

    #[command(name = "add", allow_hyphen_values = true)]
    Add { bucket: String, value: f64 },

    #[command(name = "balance")]
    Balance { bucket: String },

    #[command(name = "transactions")]
    Transactions { bucket: String },

    #[command(name = "list")]
    List,
}

#[derive(Serialize, Deserialize)]
struct Transaction {
    time: i64,
    value: f64,
}

fn get_data_dir() -> PathBuf {
    ProjectDirs::from("", "", "ishango")
        .expect("Failed to determine project directory")
        .data_dir()
        .to_owned()
}

fn is_valid_bucket_name(name: &str) -> bool {
    let re = Regex::new(r"^[a-zA-Z0-9_-]+$").unwrap();
    re.is_match(name)
}

fn get_bucket_path(bucket: &str) -> PathBuf {
    get_data_dir().join(format!("{}.jsonl", bucket))
}

fn ensure_bucket_exists(bucket: &str) -> Result<(), String> {
    if !get_bucket_path(bucket).exists() {
        return Err(format!("Bucket '{}' does not exist", bucket));
    }
    Ok(())
}

fn init(bucket: &str) -> Result<(), String> {
    if !is_valid_bucket_name(bucket) {
        return Err("Bucket name must only contain digits, letters, - or _".to_string());
    }

    let data_dir = get_data_dir();
    fs::create_dir_all(&data_dir).map_err(|e| e.to_string())?;

    let bucket_path = get_bucket_path(bucket);
    if bucket_path.exists() {
        return Err(format!("Bucket '{}' already exists", bucket));
    }

    File::create(bucket_path).map_err(|e| e.to_string())?;
    Ok(())
}

fn add(bucket: &str, value: f64) -> Result<(), String> {
    ensure_bucket_exists(bucket)?;

    let transaction = Transaction {
        time: Utc::now().timestamp(),
        value,
    };

    let mut file = OpenOptions::new()
        .append(true)
        .open(get_bucket_path(bucket))
        .map_err(|e| e.to_string())?;

    let line = serde_json::to_string(&transaction).map_err(|e| e.to_string())?;
    writeln!(file, "{}", line).map_err(|e| e.to_string())?;
    Ok(())
}

fn balance(bucket: &str) -> Result<(), String> {
    ensure_bucket_exists(bucket)?;

    let file = File::open(get_bucket_path(bucket)).map_err(|e| e.to_string())?;
    let reader = BufReader::new(file);

    let sum: f64 = reader
        .lines()
        .map_while(Result::ok)
        .filter_map(|line| serde_json::from_str::<Transaction>(&line).ok())
        .map(|transaction| transaction.value)
        .sum();

    println!("{:.2}", sum);
    Ok(())
}

fn transactions(bucket: &str) -> Result<(), String> {
    ensure_bucket_exists(bucket)?;

    let file = File::open(get_bucket_path(bucket)).map_err(|e| e.to_string())?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.map_err(|e| e.to_string())?;
        let transaction: Transaction = serde_json::from_str(&line).map_err(|e| e.to_string())?;

        let datetime: DateTime<Local> = DateTime::from(
            Utc.timestamp_opt(transaction.time, 0)
                .single()
                .ok_or("Invalid timestamp")?,
        );
        println!(
            "{} {:.2}",
            datetime.format("%Y-%m-%d %H:%M:%S"),
            transaction.value
        );
    }
    Ok(())
}

fn list() -> Result<(), String> {
    let data_dir = get_data_dir();
    if !data_dir.exists() {
        return Ok(());
    }

    for entry in fs::read_dir(data_dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("jsonl") {
            if let Some(name) = path.file_stem().and_then(|s| s.to_str()) {
                println!("{}", name);
            }
        }
    }
    Ok(())
}

fn main() {
    let opt = Opt::parse();
    let result = match opt {
        Opt::Init { bucket } => init(&bucket),
        Opt::Add { bucket, value } => add(&bucket, value),
        Opt::Balance { bucket } => balance(&bucket),
        Opt::Transactions { bucket } => transactions(&bucket),
        Opt::List => list(),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
