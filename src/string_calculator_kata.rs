const DEFAULT_DELIMITER: &str = ",";
const DELIMITER_PREFIX: &str = "//";

pub fn add(numbers: &str) -> i32 {
    let mut data = (Vec::new(), Vec::new());
    let delimiter = get_delimiter(numbers);

    let (negatives, positives) = numbers
        .split('\n')
        .flat_map(|num| num.split(delimiter))
        .fold(&mut data, |data, num| {
            let num = num.parse::<i32>().unwrap_or_default();

            if num.is_negative() {
                data.0.push(num.to_string());
            } else if num <= 1000 {
                data.1.push(num);
            };

            data
        });

    if !negatives.is_empty() {
        panic!("negatives not allowed: {}", negatives.join(", "))
    }

    positives.iter().sum()
}

fn get_delimiter(pattern: &str) -> &str {
    pattern
        .strip_prefix(DELIMITER_PREFIX)
        .and_then(|pattern| pattern.split_once('\n'))
        .map(|(delimiter, _)| {
            delimiter
                .strip_prefix('[')
                .and_then(|delimiter| delimiter.strip_suffix(']'))
                .unwrap_or(delimiter)
        })
        .unwrap_or(DEFAULT_DELIMITER)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_be_0_when_add_empty_string() {
        assert_eq!(0, add(""))
    }

    #[test]
    fn should_be_1_when_add_1() {
        assert_eq!(1, add("1"))
    }

    #[test]
    fn should_be_2_when_add_2() {
        assert_eq!(2, add("2"))
    }

    #[test]
    fn should_be_3_when_add_1_2() {
        assert_eq!(3, add("1,2"))
    }

    #[test]
    fn should_be_27_when_add_2_7_13_5() {
        assert_eq!(27, add("2,7,13,5"))
    }

    #[test]
    fn should_be_6_when_add_1ln2_3() {
        assert_eq!(6, add("1\n2,3"))
    }

    #[test]
    fn should_be_1_when_add_1_ln() {
        assert_eq!(1, add("1,\n"))
    }

    #[test]
    fn should_be_3_when_add_1_2_with_delimeter_ll() {
        assert_eq!(3, add("//;\n1;2"))
    }

    #[test]
    #[should_panic = "negatives not allowed: -1, -3"]
    fn should_be_panic_when_add_1ne_3ne() {
        add("-1,-3");
    }

    #[test]
    fn should_be_2_when_add_2_1001() {
        assert_eq!(2, add("2,1001"))
    }

    #[test]
    fn should_be_5_when_add_1_3_2_with_size_delimeter() {
        assert_eq!(6, add("//[***]\n1***2***3"))
    }
}
