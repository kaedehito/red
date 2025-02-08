use crate::ast::Ast;

pub fn parse(input: &str) -> Vec<Ast> {
    let mut result = Vec::new();
    let mut num_buffer = String::new();

    for ch in input.chars() {
        if ch.is_ascii_digit() {
            num_buffer.push(ch);
        } else {
            // 数値がバッファに溜まっていたら、Numberとして追加
            if !num_buffer.is_empty() {
                if let Ok(num) = num_buffer.parse::<u64>() {
                    result.push(Ast::Number(num));
                }
                num_buffer.clear();
            }

            // 数字以外の通常のトークンを処理
            let token = match ch {
                'p' => Some(Ast::Print),
                ',' => Some(Ast::Quote),
                'a' => Some(Ast::Add),
                'i' => Some(Ast::Insert),
                '=' => Some(Ast::Equal),
                'q' => Some(Ast::Quit),
                'w' => Some(Ast::Write),
                'd' => Some(Ast::Delete),
                '!' => Some(Ast::Exclamation),
                _ => {
                    eprintln!("?");
                    return Vec::new();
                }
            };

            if let Some(t) = token {
                result.push(t);
            }
        }
    }

    // 最後に溜まっている数値を処理
    if !num_buffer.is_empty() {
        if let Ok(num) = num_buffer.parse::<u64>() {
            result.push(Ast::Number(num));
        }
    }

    result
}
