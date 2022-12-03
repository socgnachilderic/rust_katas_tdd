pub fn fizzbuzz(number: u8) -> String {
    match number {
        _ if number % 15 == 0 => "FizzBuzz".to_string(),
        _ if number % 5 == 0 => "Buzz".to_string(),
        _ if number % 3 == 0 => "Fizz".to_string(),
        _ => number.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::fizzbuzz;

    #[test]
    fn should_be_1_when_1() {
        assert_eq!(fizzbuzz(1), "1")
    }

    #[test]
    fn should_be_2_when_2() {
        assert_eq!(fizzbuzz(2), "2")
    }

    #[test]
    fn should_be_fizz_when_3() {
        assert_eq!(fizzbuzz(3), "Fizz")
    }

    #[test]
    fn should_be_buzz_when_5() {
        assert_eq!(fizzbuzz(5), "Buzz")
    }

    #[test]
    fn should_be_fizz_when_6() {
        assert_eq!(fizzbuzz(6), "Fizz")
    }

    #[test]
    fn should_be_buzz_when_10() {
        assert_eq!(fizzbuzz(10), "Buzz")
    }

    #[test]
    fn should_be_fizzbuzz_when_15() {
        assert_eq!(fizzbuzz(15), "FizzBuzz")
    }

    #[test]
    fn should_be_fizzbuzz_when_30() {
        assert_eq!(fizzbuzz(30), "FizzBuzz")
    }

    #[test]
    fn should_be_31_when_31() {
        assert_eq!(fizzbuzz(31), "31")
    }
}
