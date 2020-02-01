fn repeat(s: &str) -> String {
    let mut repeated = s.to_string();
    let repeat_count = 5;
    for _i in 1..repeat_count {
        repeated = repeated + s;
    }
    return repeated
}

fn main() {
    println!("{}", repeat("a"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(repeat("a"), "aaaaa");
    }
}
