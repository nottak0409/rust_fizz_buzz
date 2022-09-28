use std::io;

fn fizzbuzz(n: usize) {
    for i in 0..n {
        if i % 15 == 0 {
            println!("fizzbuzz");
        } else if i % 3 == 0 {
            println!("fizz");
        } else if i % 5 == 0 {
            println!("buzz");
        } else {
            println!{"{}", i}
        }
    }
}

fn get_input() -> String {
    let mut word = String::new();
    io::stdin().read_line(&mut word).ok();
    return word.trim().to_string();
}

fn main() {
    println!("何か数字を入力してください");

    let answer = get_input();
    let number: usize = answer.parse().unwrap();
    fizzbuzz(number);
}
