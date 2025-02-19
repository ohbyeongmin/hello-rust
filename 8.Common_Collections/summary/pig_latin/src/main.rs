// 1. Find first character
// 2. if is consonant?
//   2-2. save first char
//   2-3. cut first char
//   2-4. add voca-*ay
// 3. if is gather -> keep it.
//   3-2. add vaca-hay

use std::io;

fn main() {
    let mut input = String::new();
    let mut suffix = String::from("ay");

    println!("Input Text.");
    io::stdin()
        .read_line(&mut input)
        .expect("Faild to read line.");

    input = input.trim().to_string();

    let first_char = &input[..1];

    let pig_latin = match is_gather(first_char) {
        false => {
            let first_str = &input[1..];
            suffix = format!("{first_char}{suffix}");
            format!("{first_str}-{suffix}")
        }
        _ => {
            suffix = format!("h{suffix}");
            format!("{input}-{suffix}")
        }
    };

    println!("Result: {pig_latin}");
}

fn is_gather(c: &str) -> bool {
    matches!(c, "a" | "e" | "i" | "o" | "u")
}
