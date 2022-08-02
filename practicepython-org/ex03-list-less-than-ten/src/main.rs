fn main() {
    let arr = [1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89];
    println!("Less than 10:");
    for x in arr {
        if x < 10 {
            print!("{} ", x)
        }
    }

    // arr.map(|x| {
    //     if x < 10 {
    //         print!("{} ", x)
    //     };
    //     x
    // });

    println!("Less than 5:");

    let mut small_arr = Vec::new();
    for x in arr {
        if x < 5 {
            small_arr.push(x)
        };
    }

    for x in small_arr {
        print!("{} ", x);
    }
}
