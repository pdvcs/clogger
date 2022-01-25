#[macro_use]
extern crate quick_error;

use chrono::Local;
use std::env;
use std::fs;
use std::io::{stdin, BufRead, BufReader, Write};
use std::process::exit;

quick_error! {
    #[derive(Debug)]
    pub enum CloggerError {
        Custom(s: String) {
            display("{}", s)
            from()
        }
    }
}

const ENV_VAR_BASE: &str = "CLOG_PREFIX";
const TIMESTAMP_FORMAT: &str = "%Y%m%d-%H%M";

fn die(msg: &str, e: CloggerError) {
    println!("{}: {}", msg, e);
    exit(1);
}

fn write_line(line: &str) -> Result<(), CloggerError> {
    let output_file_name =
        env::var(ENV_VAR_BASE).map_err(|e| format!("could not read ${}: {}", ENV_VAR_BASE, e))?;
    let date_format = Local::now().format(TIMESTAMP_FORMAT).to_string();

    let mut output = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(format!("{}.{}", output_file_name, date_format))
        .map_err(|e| format!("error opening file for appending: {}", e))
        .unwrap();

    output
        .write_all(format!("{}\n", line).as_bytes())
        .map_err(|e| format!("error appending to file: {}", e))?;

    Ok(())
}

fn main() {
    let standard_in = BufReader::new(stdin());

    // read stdin line by line
    for r_line in standard_in.lines() {
        let line = r_line
            .map_err(|e| format!("error reading stdin: {}", e))
            .unwrap();
        let r = write_line(&line);
        match r {
            Err(e) => die("write_line", e),
            _ => (),
        }
    }
}
