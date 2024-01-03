/// This module contains the logic for the cat command
pub mod logic {

    /// This function prints the content of the file with line numbers
    pub fn numbered_lines(lines: std::str::Lines<'_>) -> String {
        let mut for_test: String = Default::default(); // for testing
        for (i, line) in lines.enumerate() {
            let value = format!("\t\x1b[34m{}\x1b[0m {}", i + 1, line);
            for_test.push_str(&value);
            println!("{}",value);
        }
        for_test
    }

    /// This function prints the content of the file with nonblank line numbers
    pub fn nonblank_numbered_lines(lines: std::str::Lines<'_>) -> String {
        let mut for_test: String = Default::default(); // for testing
        let mut count: i128 = 1;
        for (_, line) in lines.enumerate() {
            if line != "" {
                let value = format!("\t\x1b[34m{}\x1b[0m {}", count, line);
                for_test.push_str(&value);
                println!("{}",value);
                count += 1;
            } else {
                for_test.push_str("\t");
                println!("\t");
            }
        }
        for_test
    }

    /// This function prints the content of the file with tabs replaced with ^I
    pub fn show_tabs(lines: std::str::Lines<'_>) {
        for (_, line) in lines.enumerate() {
            for c in line.chars() {
                if c == '\t' {
                    print!("^I");
                } else {
                    print!("{}", c);
                }
            }
            println!();
        }
    }
    /// This function prints the content of the file with repeated empty lines suppressed
    pub fn squeeze_blank(lines: std::str::Lines<'_>) {
        let mut prev_line = "".to_string();
        for (_, line) in lines.enumerate() {
            if prev_line == line {
                prev_line = line.to_string();
                continue;
            } else {
                prev_line = line.to_string();
                println!("{}", line);
            }
        }
    }
}
