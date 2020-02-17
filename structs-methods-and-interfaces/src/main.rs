fn perimeter(width: f64, height: f64) -> f64 {
    return 2.0 * (width + height)
}

fn area(width: f64, height: f64) -> f64 {
    return width * height
}

fn main() {
    println!("{}", perimeter(10.0, 10.0));
    println!("{}", area(12.0, 6.0));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(perimeter(10.0, 10.0), 40.0);
        assert_eq!(area(12.0, 6.0), 72.0);
    }
}
