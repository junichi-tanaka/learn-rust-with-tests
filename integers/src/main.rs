fn add(x: i64, y: i64) -> i64 {
    return x + y
}

fn main() {
    println!("{}", add(1, 2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(add(2, 2), 4);
        assert_eq!(add(3, -2), 1);
        assert_eq!(add(-3, 2), -1);
    }
}
