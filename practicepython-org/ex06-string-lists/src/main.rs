use std::io::{self, Write};

fn input(message: &str) -> io::Result<String> {
    print!("{}", message);

    let _ = io::stdout().flush()?;

    let mut buffer: String = String::new();
    io::stdin().read_line(&mut buffer)?;

    Ok(buffer.trim_end().to_owned())
}

fn main() {
    let message = input("Enter a string: ").unwrap();

    // let msg_len = message.len();

    // for x in 0..msg_len {
    //     if message.chars().nth(x).unwrap() != message.chars().nth(msg_len - 1 - x).unwrap() {
    //         println!("`{}` not a palindrome", message);
    //         return
    //     }
    // }
    // println!("`{}` is a palindrome", message);

    let half = message.len() / 2;
    let rev_message = message.chars().rev();

    let is_palindrome = message.chars().take(half).eq(rev_message.take(half));
    println!(
        "`{}` {} a palindrome",
        message,
        if is_palindrome { "is" } else { "is not" }
    );
}
