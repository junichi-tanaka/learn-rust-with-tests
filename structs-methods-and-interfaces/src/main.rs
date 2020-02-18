
trait Shape {
    fn area(&self) -> f64;
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        return self.width * self.height
    }
}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        return std::f64::consts::PI * self.radius * self. radius
    }
}

fn print_area<T: Shape>(shape: T) {
    println!("{}", shape.area());
}

fn main() {
    let rectangle = Rectangle { width:12.0, height:6.0 };
    print_area(rectangle);
    let circle = Circle { radius:10.0 };
    print_area(circle);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let rectangle = Rectangle { width:12.0, height:6.0 };
        assert_eq!(rectangle.area(), 72.0);
        let circle = Circle { radius:10.0 };
        assert_eq!(circle.area(), 314.1592653589793);
    }
}
