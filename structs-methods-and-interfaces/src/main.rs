struct Rectangle {
    width: f64,
    height: f64,
}

fn perimeter(rectangle: Rectangle) -> f64 {
    return 2.0 * (rectangle.width + rectangle.height)
}

fn area(rectangle: Rectangle) -> f64 {
    return rectangle.width * rectangle.height
}

fn main() {
    let mut rectangle = Rectangle { width:10.0, height:10.0 };
    println!("{}", perimeter(rectangle));
    rectangle = Rectangle { width:12.0, height:6.0 };
    println!("{}", area(rectangle));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut rectangle = Rectangle { width:10.0, height:10.0 };
        assert_eq!(perimeter(rectangle), 40.0);
        rectangle = Rectangle { width:12.0, height:6.0 };
        assert_eq!(area(rectangle), 72.0);
    }
}
