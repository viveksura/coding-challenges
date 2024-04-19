extern crate getopts;

use std::io::{stdin, Read};
use std::io::{BufRead, BufReader, Error};
use std::fs::File;
use std::env;
use getopts::Options;

fn print_usage(program: &str, opts: &Options) {
    let brief = format!("Usage: {} [options (Optional)] [filepath]", program);
    print!("{}", opts.usage(&brief));
}

fn parse_file(filepath: &str) -> Result<[i32; 3], Error> {
    match File::open(filepath) {
        Ok(t) => {
            return parse_buffer(BufReader::new(t))
        },
        Err(error) => {
            return Err(error);
        }
    };
}

fn parse_buffer<T: Read>(reader: BufReader<T>) -> Result<[i32; 3], Error> {
    let mut word_count = 0;
    let mut char_count = 0;
    let mut line_count: i32 = 0;
    
    for line_result in reader.lines() {
        match line_result {
            Ok(l) => {
                char_count += l.bytes().count();
                word_count += l.split_whitespace().count();
                line_count += 1;
            },
            Err(error) => {
                return Err(error);
            }
        };
    }

    return Ok([char_count as i32, word_count as i32, line_count]);
}

fn main() -> std::io::Result<()> {

    let args: Vec<String> = env::args().collect();
    let program: String = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("c", "char",  "Get char/byte count");
    opts.optflag("l", "line", "Get line count");
    opts.optflag("w", "word", "Get word count");
    opts.optflag("h", "help", "Help");
 
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m },
        Err(f) => {
            println!("{}", f.to_string());
            print_usage(&program, &opts);
            return Ok(());
        }
    };

    if matches.opt_present("h") {
        print_usage(&program, &opts);
        return Ok(());
    }

    let opt_found = matches.opt_present("c") ||  matches.opt_present("l") || matches.opt_present("w");

    let mut filepath = String::new();
    let result; 
    
    if (opt_found && args.len() == 3) || (!opt_found && args.len() == 2) {
        filepath = args[args.len() - 1].clone();
        result = parse_file(&filepath);
    } else {
        result = parse_buffer(BufReader::new(stdin()))
    }

    match result {
        Ok(count) => {
            if matches.opt_present("c") {
                println!("{} {}", count[0], filepath);
            } else if matches.opt_present("l") {
                println!("{} {}", count[1], filepath);
            } else if matches.opt_present("w") {
                println!("{} {}", count[2], filepath);
            } else {
                println!("{} {} {} {}", count[0], count[1], count[2], filepath);
            }
        },
        Err(e) => {
            println!("{}", e.to_string());
            print_usage(&program, &opts);
        }
    };

    Ok(())
}
