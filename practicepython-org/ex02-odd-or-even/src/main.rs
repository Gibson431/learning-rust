use std::io;

fn input(message: &str) -> io::Result<String> {
    use std::io::Write;

    print!("{}", message);

    let _ = io::stdout().flush()?;

    let mut buffer: String = String::new();
    io::stdin().read_line(&mut buffer)?;

    Ok(buffer.trim_end().to_owned())
}

fn main() {
    let number = input("Input a number: ")
        .unwrap()
        .parse::<i32>()
        .expect("Expected a number.");

    let check = input("Input `check` number: ")
        .unwrap()
        .parse::<i32>()
        .expect("Expected a number.");

    if number % 2 == 0 {
        println!("`{}` is even", number);
    } else {
        println!("`{}` is odd", number);
    }

    if number % check == 0 {
        println!("`{}` divides by `{}`", number, check)
    } else {
        println!("`{}` does not divide by `{}`", number, check)
    }
}
