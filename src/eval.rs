use crate::ast::Ast;
use std::{fs, path::Path};

pub fn eval<P: AsRef<Path>>(
    parsed: &[Ast],
    buffer: &mut Vec<String>,
    path: P,
    line: &mut u64,
    saved: &mut bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut current_state = Ast::Normal;
    for &current in parsed {
        match current {
            Ast::Print if current_state == Ast::Quote => {
                for (line, content) in buffer.iter().enumerate() {
                    println!("{:>6}  |{content}", line + 1);
                }
            }

            Ast::Print => {
                if (*line as usize) < buffer.len() {
                    println!("{}", buffer[*line as usize]);
                } else {
                    eprintln!("?");
                }
            }

            Ast::Quote => current_state = Ast::Quote,

            Ast::Add => loop {
                let mut input = rustyline::DefaultEditor::new().unwrap();

                if let Ok(input) = input.readline("") {
                    let input = input.trim().to_string();

                    if input == "." {
                        break;
                    } else {
                        *saved = false;
                    }

                    if buffer.is_empty() {
                        buffer.push(input.clone());
                    } else {
                        let insert_index = (*line as usize).saturating_add(1).min(buffer.len());
                        buffer.insert(insert_index, input);
                        *line = insert_index as u64;
                    }
                } else {
                    break;
                }
            },

            Ast::Equal => {
                if current_state == Ast::Quote {
                    println!("{}", *line + 1);
                } else {
                    eprintln!("?");
                }
            }

            Ast::Insert => loop {
                let mut rl = rustyline::DefaultEditor::new().unwrap();

                if let Ok(input) = rl.readline("") {
                    let input = input.trim().to_string();

                    if input == "." {
                        break;
                    } else {
                        *saved = false;
                    }

                    if buffer.is_empty() {
                        buffer.push(input.clone());
                        *line = 0;
                    } else {
                        let insert_index = (*line as usize).min(buffer.len());
                        buffer.insert(insert_index, input);
                        *line = (insert_index + 1).min(buffer.len()) as u64;
                    }
                } else {
                    break;
                }
            },

            Ast::Delete => {
                if buffer.is_empty() || (*line as usize) >= buffer.len() {
                    eprintln!("?");
                } else {
                    buffer.remove(*line as usize);
                    if *line >= buffer.len() as u64 {
                        *line = buffer.len().saturating_sub(1) as u64;
                    }
                }
            }

            Ast::Normal => { /* No action */ }

            Ast::Quit => {
                current_state = Ast::Quit;
                if !*saved {
                    eprintln!("?");
                } else {
                    std::process::exit(0);
                }
            }

            Ast::Write => {
                *saved = true;
                let path = path.as_ref();
                if let Err(e) = fs::write(path, buffer.join("\n")) {
                    eprintln!("Error writing to file {}: {}", path.display(), e);
                    return Err(e.into());
                }
            }

            Ast::Number(p) => {
                if p > 0 && (p as usize) <= buffer.len() {
                    *line = p - 1; // edのように1-based indexなので、0-based indexに調整
                    println!("{}", buffer[*line as usize]);
                } else {
                    eprintln!("?");
                }
            }

            Ast::Exclamation => {
                if current_state == Ast::Quit {
                    std::process::exit(0);
                } else {
                    eprintln!("?");
                }
            }
        }
    }
    Ok(())
}
