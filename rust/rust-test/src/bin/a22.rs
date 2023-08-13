// Topic: Testing
//
// Requirements:
// * Write tests for the existing program to ensure proper functionality.
//
// Notes:
// * Create at least two test cases for each function.
// * Use `cargo test` to test the program.
// * There are intentional bugs in the program that need to be fixed.
//   * Check the documentation comments for the functions to
//     determine how the they should operate.

/// Ensures n is >= lower and <= upper.
fn clamp(n: i32, lower: i32, upper: i32) -> i32 {
    if n < lower {
        lower
    } else if n > upper {
        upper
    } else {
        n
    }
}

/// Divides a and b.
fn div(a: i32, b: i32) -> Option<i32> {
    match b {
        0 => None,
        b => Some(a/b),
    }
}

/// Takes two strings and places them immediately one after another.
fn concat(first: &str, second: &str) -> String {
    format!("{}{}", first, second)
}

fn main() {}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn check_clamp_upper_bound() {
        let expected = 10;
        let result = clamp(123, 1, 10);
        assert_eq!(result, expected, "123 clamedp for [1;10] should be 10");
    }

    #[test]
    fn check_clamp_reg_value() {
        let expected: i32 = 4;
        let result = clamp(4, 1, 10);
        assert_eq!(result, expected, "4 clamped for [1;10] should be 4");
    }

    #[test]
    fn check_div() {
        let expected = Some(3);
        let result = div(9, 3);
        assert_eq!(result, expected, "9 div 3 should be 3");
    }

    #[test]
    fn check_div_by_zero() {
        let expected = None;
        let result: Option<i32> = div(9, 0);
        assert_eq!(result, expected, "9 div 0 should be none");
    }

    #[test]
    fn check_concat() {
        let expected = "str1str2";
        let result = concat("str1", "str2");
        assert_eq!(result, expected, "should be \"str1str2\" after concat");
    }
}

