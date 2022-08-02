use std::io::{self, Write};

fn input(message: &str) -> io::Result<String>{
    print!("{}", message);

    let _ = io::stdout().flush()?;

    let mut buffer: String = String::new();
    io::stdin().read_line(&mut buffer)?;

    Ok(buffer.trim_end().to_owned())
}
fn main() {
    let name = input("Give me your name: ").unwrap();
    print!("Your name is: {}", name);
    let age = input("Enter your age: ").unwrap();
    print!("Your age is: {}", age)
}
