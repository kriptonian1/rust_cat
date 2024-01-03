use std::fs::{self};

use clap::Parser;
use logic::logic::{nonblank_numbered_lines, numbered_lines, show_tabs, squeeze_blank};

mod logic;

#[derive(Parser)]
#[command(
    author = "@kriptonian8",
    version,
    about = "Implementation of cat in rust",
    long_about = "This is a simple cat command"
)]
struct Args {
    #[arg(name = "FILE", help = "File to read")]
    file_name: String,

    #[arg(name = "number", short, long, help = "Print line numbers")]
    line_number: bool,

    #[arg(
        name = "nonblank",
        short = 'b',
        long = "number-nonblank",
        help = "Print nonblank line numbers"
    )]
    nonblank: bool,

    #[arg(
        name = "show tabs",
        short = 'T',
        long = "show-tabs",
        help = "Print TAB characters as ^I"
    )]
    show_tabs: bool,

    #[arg(
        name = "squeeze blank",
        short = 's',
        long = "squeeze-blank",
        help = "Suppress repeated empty output lines"
    )]
    squeeze_blank: bool,
}

fn main() {
    let cli: Args = Args::parse();
    let read_content: Result<String, std::io::Error> = fs::read_to_string(cli.file_name);
    match read_content {
        Ok(content) => {
            let lines: std::str::Lines<'_> = content.lines();
            if cli.line_number {
                let _ = numbered_lines(lines);
            } else if cli.nonblank {
                let _ = nonblank_numbered_lines(lines);
            } else if cli.show_tabs {
                show_tabs(lines)
            } else if cli.squeeze_blank {
                squeeze_blank(lines)
            } else {
                println!("{}", content);
            }
        }
        Err(err) => {
            println!("{}", err);
        }
    }
}
