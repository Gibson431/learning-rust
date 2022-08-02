fn main() {
    let b = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13];
    let a = [1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89];
    // TODO randomise arrays

    let mut overlap = Vec::new();

    for x in b {
        let y = x; // TODO figure out how borrowing works properly
        if a.contains(&x) {
            overlap.push(y);
        }
    }

    for z in overlap {
        print!("{} ", z);
    }
}
