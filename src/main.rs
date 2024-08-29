use std::path::Path;

// function, int, if
// かんすう, せいすう, なら
const RESERVED_WORDS: [&str; 3] = ["関数", "整数", "なら"];

enum Token {
    Function,
    Int,
    If,
}
fn check_file(source_file: &Path) -> Result<String, String> {
    println!("{}をコンパイルする", source_file.display());
    match source_file.exists() {
        false => Err(format!("{}見つかりません", source_file.display())),
        true => {
            println!("{}が見つかりました", source_file.display());
            if source_file.extension().unwrap().to_str().unwrap().eq("nh") {
                Ok("ファイルフォーマットが正しいです".to_string())
            } else {
                Err("ファイルフォーマットが正しくありません".to_string())
            }
        }
    }
}

fn read_file(source_file: &Path) -> Option<Vec<String>> {
    let mut lines = Vec::new();
    match std::fs::read_to_string(source_file) {
        Ok(contents) => {
            regex::Regex::new(r"\s*([一-龠|ぁ-ゔ])*\s*;")
                .unwrap()
                .captures_iter(&contents)
                .for_each(|cap| {
                    lines.push(
                        cap.get(0)
                            .unwrap()
                            .as_str()
                            .to_string()
                            .split_whitespace()
                            .collect(), // TODO i think this can be done better
                    );
                });

            match lines.len() {
                0 => None,
                _ => Some(lines),
            }
        }
        Err(e) => {
            panic!("{}", e);
        }
    }
}

fn tokenizer(code_bytes: Vec<String>) -> Vec<Token> {
    let mut tokens = Vec::new();
    code_bytes.into_iter().for_each(|byte| {
        println!("b:{} ", byte);
    });
    //code_bytes.into_iter().for_each(|byte| {
    //    match byte {
    //        b'関' => tokens.push(Token::Function),
    //        b'数' => tokens.push(Token::Int),
    //        b'な' => tokens.push(Token::If),
    //        _ => (),
    //    }
    //});
    tokens
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("引数のサイズが不正です");
        std::process::exit(1);
    }
    let source_file = Path::new(&args[1]);
    match check_file(source_file) {
        Ok(_) => tokenizer(read_file(source_file).unwrap()),
        Err(e) => panic!("{}", e),
    };
}
