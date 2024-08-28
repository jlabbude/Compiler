use std::path::Path;

// function, int, if
// かんすう, せいすう, なら
const RESERVED_WORDS: [&str; 3] = ["関数", "整数", "なら"];

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
        },
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("引数のサイズが不正です");
        std::process::exit(1);
    }
    match check_file(Path::new(&args[1])){
        Ok(_) => std::process::exit(0),
        Err(e) => panic!("{}", e),
    }
}