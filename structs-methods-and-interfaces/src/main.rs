
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

struct Triangle {
    base: f64,
    height: f64,
}

impl Shape for Triangle {
    fn area(&self) -> f64 {
        return (self.base * self.height) * 0.5
    }
}

fn print_area<T: Shape>(shape: T) {
    println!("{}", shape.area());
}

fn main() {
    print_area(Rectangle{width:12.0, height:6.0});
    print_area(Circle{radius:10.0});
    print_area(Triangle{base:12.0, height:6.0});
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Rectangle{width:12.0, height:6.0}.area(), 72.0);
        assert_eq!(Circle{radius:10.0}.area(), 314.1592653589793);
        assert_eq!(Triangle{ base:12.0, height:6.0}.area(), 36.0);
    }
}
