use std::error::Error;
use std::{fs, usize};
use colored::Colorize;

pub mod config;
use crate::config::Config;

// Now our abstraction
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filename)?;
    let ll = config.linelength;

    /* For debugging
        for line in content.lines() {
            for char in line.chars() {
                if char == ' ' {
                    print!("_");
                } else if char == '\t' {
                    print!("\\t");
                } else {
                    print!("{}", &char);
                }
            }
            println!("");
        }
    */

    let results: Vec<(usize, &str)> = content
        .lines()
        .map(|line| {
            // Alternative to (line.chars().count(), line) to account for tabs
            (find_len(line, config.tab_size), line)
        })
        .collect();

    // A header
    println!("?|Line Num |Count|Text");

    for (line_num, (length, line)) in results.iter().enumerate() {
        if length > &ll {
            let temp = format!("X|Line:{:04}|{:4}|{}", line_num, length, &line);
            println!("{}", &temp.red());
            // println!("❌ Line {}, Len: {}, Text:{}", line_num + 1, length, line)
        } else {
            // A valid length, print our valid message
            let temp = format!(" |Line:{:04}|{:4}|{}", line_num, length, &line);
            println!("{}", &temp.green());
            // println!("✅ Line {}, Len: {}, Text:{}", line_num + 1, length, line)
        }
    }

    Ok(())
}

// Takes into account the true length of a tab character,
// based on how long a tabstop is in given application
fn find_len(line: &str, tab_size: usize) -> usize {
    let mut ind = 0;
    for char in line.chars() {
        if char == '\t' {
            ind += tab_size - (ind % tab_size)
        } else {
            ind += 1;
        }
    }
    ind
}
