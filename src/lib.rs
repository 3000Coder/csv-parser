use std::fs::read_to_string;

const QUOTE: char = '"';
const ESCAPE: char = '\\';

// ? May be a better idea to add interfaces instead of having everything public
pub struct CsvContent {
    pub header: Option<Vec<String>>,
    pub content: Vec<Vec<String>>,
}

// ? May be better to handle this dirrectly in handle function
fn read_lines(filename: &str) -> Result<Vec<String>, ()> {
    match read_to_string(filename) {
        Ok(f) => {
            let mut result: Vec<String> = Vec::new();

            for line in f.lines() {
                result.push(line.to_string());
            }

            return Ok(result);
        }
        Err(_) => return Err(()),
    }
}

// TODO: Add borrows
pub fn parse(filename: &str, enable_header: &bool, separator: &char) -> Result<CsvContent, ()> {
    let lines: Vec<String>;
    match read_lines(filename) {
        Err(_) => return Err(()),
        Ok(x) => {
            lines = x;
        }
    }

    let mut content: Vec<Vec<String>> = Vec::new();
    let mut header: bool = enable_header.clone();
    let mut header_row: Option<Vec<String>> = None;

    for line in lines {
        let mut nested: bool = false;
        let mut prev_char: char = '\0';
        let mut field_buffer: String = String::new();
        let mut line_result: Vec<String> = Vec::new();

        for c in line.chars() {
            match c {
                QUOTE => {
                    if prev_char == ESCAPE || prev_char == QUOTE {
                        field_buffer.push(c);
                    } else {
                        nested = !nested;
                    }
                }
                ESCAPE => (),
                _ => {
                    if c == separator.clone() {
                        if nested || prev_char == ESCAPE {
                            field_buffer.push(c);
                        } else {
                            line_result.push(field_buffer.clone());
                            field_buffer = "".to_string();
                        }
                    } else {
                        if prev_char == ESCAPE && c == 'n' {
                            field_buffer.push('\n');
                        } else {
                            field_buffer.push(c);
                        }
                    }
                }
            }
            prev_char = c;
        }

        line_result.push(field_buffer.clone());
        if header {
            header_row = Some(line_result.clone());
            header = false;
        } else {
            content.push(line_result.clone());
        }
    }

    return Ok(CsvContent {
        header: header_row,
        content: content,
    });
}
