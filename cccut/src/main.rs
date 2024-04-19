
use std::io::{stdin, Read};
use std::io::{BufRead, BufReader};
use std::fs::File;
use core::str;
use argh::FromArgs;

#[derive(FromArgs)]
/// Reach new heights.
struct Args {
    /// fields to cut
    #[argh(option, short = 'f', long = "fields")]
    fields: String,

    /// delimiter, default tab
    #[argh(option, short = 'd', default = "String::from(\"\t\")")]
    delimiter: String,

    /// filename
    #[argh(positional)]
    filename: Option<String>,
}

fn parse_buffer<T: Read>(reader: BufReader<T>, columns: Vec<usize>, delimiter: String) {
    for line_result in reader.lines() {
        match line_result {
            Ok(line) => {
                let mut words = line.split(&delimiter);
                let mut l = String::from("");
                for col in &columns {
                    l += words.nth(*col).unwrap_or_default();
                    l += &String::from("\t");
                }
                println!("{}", l);
            },
            Err(error) => {
                println!("error occurred")
            }
        };
    }
}

fn get_fields_needed(fields_needed: String) -> Result<Vec<usize>, Box<dyn std::error::Error>> {
    let results = if fields_needed.contains(',') {
        fields_needed
            .split(',')
            .map(|field| field.parse::<usize>())
            .collect::<Result<Vec<_>, _>>()?
    } else if fields_needed.contains(' ') {
        fields_needed
            .split(' ')
            .map(|field| field.parse::<usize>())
            .collect::<Result<Vec<_>, _>>()?
    } else {
        vec![fields_needed.parse::<usize>()?]
    };

    // we are subtracting 1 because the command is 1 based and the field is 0 based
    Ok(results.into_iter().map(|field| field - 1).collect())
}

fn main() {
    let up: Args = argh::from_env();
    let columns = get_fields_needed(up.fields);

    // Handling the case where filename is not provided
    match up.filename {
        Some(filename) => {
            match File::open(filename) {
                Ok(t) => {
                    parse_buffer(BufReader::new(t), columns.unwrap(), up.delimiter)
                },
                Err(error) => {
                    println!("file error")
                }
            };
        },
        None => {
            parse_buffer(BufReader::new(stdin()), columns.unwrap(), up.delimiter)
        },
    }
}
