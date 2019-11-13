#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    #[allow(dead_code)]
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn it_works2() {
        assert_eq!(2 + 3, 5);
    }

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    #[should_panic(expected = "Make this test fail")]
    fn another() {
        panic!("Make this test fail");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };

        assert!(larger.can_hold(&smaller));
    }
}
