use crate::error::MyError;
use std::error::Error;

pub fn get_int() -> u64 {
    let mut input = String::new();
    if let Ok(_size) = std::io::stdin().read_line(&mut input) {}
    input.trim().parse().unwrap()
}

pub struct ParsedInput {
    pub cmd: String,
    pub tokv: Vec<String>,
}

#[derive(Copy, Clone)]
pub enum ProcessType {
    Foreground,
    Background,
}

pub struct CmdOptions {
    pub cmd: String,
    pub in_file: Option<String>,
    pub out_file: Option<String>,
    pub bg: ProcessType,
    pub argv: Vec<String>,
}

fn remove_first(token: &String) -> String {
    let mut chars = token.chars();
    chars.next();
    chars.as_str().to_string()
}

impl From<String> for ParsedInput {
    fn from(line: String) -> Self {
        ParsedInput {
            cmd: line.clone(),
            tokv: line.split(" ").map(|s| s.to_string()).collect(),
        }
    }
}

impl ParsedInput {
    pub fn get_int(&self) -> Option<i32> {
        if self.tokv.len() != 2 {
            None
        } else {
            self.tokv[1].parse().ok()
        }
    }

    pub fn to_cmd(&self) -> Result<CmdOptions, Box<dyn Error>> {
        let mut cmd_options = CmdOptions {
            cmd: self.cmd.clone(),
            in_file: None,
            out_file: None,
            bg: ProcessType::Foreground,
            argv: Vec::new(),
        };
        for (i, token) in self.tokv.iter().enumerate() {
            if let Some(c) = token.chars().nth(0) {
                match c {
                    '&' => {
                        if token.len() != 1 && i != self.tokv.len() - 1 {
                            return Err(MyError::new("No command before &"));
                        }
                        cmd_options.bg = ProcessType::Background;
                    }
                    '<' => {
                        if cmd_options.in_file.is_some() {
                            return Err(MyError::new("Can only have one input file"));
                        }
                        cmd_options.in_file = Some(remove_first(&token));
                    }
                    '>' => {
                        if cmd_options.out_file.is_some() {
                            return Err(MyError::new("Can only have one output file"));
                        }
                        cmd_options.out_file = Some(remove_first(&token));
                    }
                    _ => cmd_options.argv.push(token.to_string()),
                }
            }
        }

        Ok(cmd_options)
    }
}
