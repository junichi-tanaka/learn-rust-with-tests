
fn sum(numbers: &[i32]) -> i32 {
    let mut sum: i32 = 0;
    for number in numbers {
        sum += number;
    }
    return sum;
}

fn main() {
    println!("{}", sum(&[1, 2, 3, 4, 5]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(sum(&[1, 2, 3, 4, 5]), 15);
        assert_eq!(sum(&[1, 2, 3]), 6);
    }
}

