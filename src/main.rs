use std::fs::read_to_string;

// TODO: convert to command line arguments
// ? May be better to move to struct
const SEPARATOR: char = ',';
const QUOTE: char = '"';
const ESCAPE: char = '\\';
const HEADER: bool = false;
const FILENAME: &str = "test.csv";

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

pub fn parse(filename: &str) -> Result<CsvContent, ()> {
    let lines = read_lines(filename).unwrap(); // TODO: Catch and return instead of panicking
    let mut content: Vec<Vec<String>> = Vec::new();

    for line in lines {
        let mut nested: bool = false;
        let mut prev_char: char = '\0';
        let mut field_buffer: String = String::new();
        let mut line_result: Vec<String> = Vec::new();

        for c in line.chars() {

            // TODO: Replace with match
            if c == QUOTE {
                if prev_char == ESCAPE || prev_char == QUOTE {
                    field_buffer.push(c);
                } else {
                    nested = !nested;
                }
            } else if c == SEPARATOR {
                if nested || prev_char == ESCAPE {
                    field_buffer.push(c);
                } else {
                    line_result.push(field_buffer.clone());
                    field_buffer = "".to_string();
                }
            } else if c != ESCAPE {
                field_buffer.push(c);
            }
            prev_char = c;
        }

        line_result.push(field_buffer.clone());
        content.push(line_result);
    }

    return Ok(CsvContent {
        header: None,
        content: content
    })
}

fn main() {}
