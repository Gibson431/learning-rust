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
        .expect("Expected a number");
    println!("{}", number);

    let range = 2..=(number / 2);

    let mut divisors = Vec::new();
    for x in range {
        if number % x == 0 {
            divisors.push(x)
        };
    }

    for x in divisors {
        print!("{} ", x)
    }
}
