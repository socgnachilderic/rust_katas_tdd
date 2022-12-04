use std::collections::HashMap;

pub fn combinations_of_wardrobe(total_length: i32) -> Vec<Vec<i32>> {
    let mut wardrobe_elements = vec![50, 75, 100, 120];
    let mut combinations = vec![];

    while !wardrobe_elements.is_empty() {
        combinations.push(extract_elements(total_length, &wardrobe_elements));
        wardrobe_elements.pop();
    }

    combinations
}

pub fn get_cheapest_combinations(combinations: &[Vec<i32>]) -> Vec<i32> {
    combinations
        .iter()
        .min_by_key(|elt| calcul_price(elt))
        .unwrap()
        .clone()
}

fn extract_elements(total_length: i32, wardrobe_elements: &[i32]) -> Vec<i32> {
    let mut wardrobe_elements_clone = wardrobe_elements.to_owned();
    let mut combinations = vec![];
    let mut total_length = total_length;

    while !wardrobe_elements_clone.is_empty() {
        match wardrobe_elements_clone.last() {
            Some(current_element) if *current_element <= total_length => {
                combinations.push(*current_element);
                total_length -= current_element;
            }
            _ => {
                wardrobe_elements_clone.pop();
            }
        };
    }

    combinations
}

fn calcul_price(wardrobe_elements: &[i32]) -> i32 {
    let mut wardrobe_elements_prices = HashMap::new();

    wardrobe_elements_prices.insert(50, 59);
    wardrobe_elements_prices.insert(75, 62);
    wardrobe_elements_prices.insert(100, 90);
    wardrobe_elements_prices.insert(120, 111);

    wardrobe_elements
        .iter()
        .map(|element| wardrobe_elements_prices.get(element).unwrap_or(&0))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn combinations_should_be_120_100_when_extract_250_and_120() {
        let combinations = extract_elements(250, &vec![50, 75, 100, 120]);

        assert_eq!(vec![120, 120], combinations)
    }

    #[test]
    fn combinations_should_be_100_100_50_when_extract_250_and_100() {
        let combinations = extract_elements(250, &vec![50, 75, 100]);

        assert_eq!(vec![100, 100, 50], combinations)
    }

    #[test]
    fn combinations_should_be_75_75_75_when_extract_250_and_75() {
        let combinations = extract_elements(250, &vec![50, 75]);

        assert_eq!(vec![75, 75, 75], combinations)
    }

    #[test]
    fn combinations_should_be_50_50_50_when_extract_250_and_50() {
        let result = extract_elements(250, &vec![50]);

        assert_eq!(vec![50, 50, 50, 50, 50], result)
    }

    #[test]
    fn price_should_be_222_when_size_are_120_120() {
        let price = calcul_price(&vec![120, 120]);

        assert_eq!(222, price);
    }

    #[test]
    fn price_should_be_239_when_size_are_100_100_50() {
        let price = calcul_price(&vec![100, 100, 50]);

        assert_eq!(239, price);
    }

    #[test]
    fn price_should_be_126_when_size_are_75_75_75() {
        let price = calcul_price(&vec![75, 75, 75]);

        assert_eq!(186, price);
    }

    #[test]
    fn price_should_be_295_when_size_are_50_50_50_50_50() {
        let price = calcul_price(&vec![50, 50, 50, 50, 50]);

        assert_eq!(295, price);
    }
}
