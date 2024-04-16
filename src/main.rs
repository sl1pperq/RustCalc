fn plus(a: u64, b: u64) -> u64 {
    return a + b;
}

fn minus(a: u64, b: u64) -> u64 {
    return a - b;
}

fn multi(a: u64, b: u64) -> u64 {
    return a * b;
}

fn divide(a: u64, b: u64) -> u64 {
    return a / b;
}

fn master(a: u64, b: u64, sign: &str) {
    if sign == "+" {
        println!("{}", plus(a, b));
    } else if sign == "-" {
        println!("{}", minus(a, b));
    } else if sign == "x" {
        println!("{}", multi(a, b));
    } else if sign == ":" {
        println!("{}", divide(a, b));
    }
}

fn main() {
    master(10, 5, "x");
}
