use tdd_katas::wardrobe_kata::*;

fn main() {
    let combinations = combinations_of_wardrobe(250);
    let cheapest_combination = get_cheapest_combinations(&combinations);

    println!("combinations are: {:?}", combinations);
    println!("the cheapest combination is {:?}", cheapest_combination);
}
