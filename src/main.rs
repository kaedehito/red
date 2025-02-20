use std::{env, fs, io, process};

mod ast;
mod eval;
mod parse;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: red <file_path>");
        process::exit(1);
    }

    let path = &args[1];
    let file_content = fs::read_to_string(path).unwrap_or_else(|e| {
        if e.kind() == io::ErrorKind::NotFound {
            String::new()
        } else {
            eprintln!("Failed to open: {e}");
            std::process::exit(1);
        }
    });

    let mut line = 0u64;

    let mut file_vec: Vec<String> = file_content.lines().map(String::from).collect();
    let chars_num = file_content.chars().count();

    println!("{chars_num}");

    let mut saved = true;

    let mut rl = match rustyline::DefaultEditor::new() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed: {e}");
            std::process::exit(1);
        }
    };

    loop {
        if let Ok(input) = rl.readline("> ") {
            let input = input.trim();
            let parsed = parse::parse(input);
            if let Err(e) = eval::eval(&parsed, &mut file_vec, path, &mut line, &mut saved) {
                eprintln!("Failed: {e}");

                rl.add_history_entry(input).unwrap();
            }
        } else {
            eprintln!("?");
            continue;
        }
    }
}
