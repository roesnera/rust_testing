#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    // #[test]
    // fn another() {
    //     panic!("Make this test fail");
    // }

    #[test]
    fn larger_can_hold_smaller() {
        let (larger, smaller) = get_larger_and_smaller();

        assert!(larger.can_hold(&smaller));
    }

    fn get_larger_and_smaller() -> (Rectangle, Rectangle) {
        
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        return (larger, smaller);
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let (larger,smaller) = get_larger_and_smaller();
        assert!(!smaller.can_hold(&larger));
    }
}
