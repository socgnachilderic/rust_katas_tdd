use tdd_katas::fizz_buzz_kata::fizzbuzz;

fn main() {
    for i in 1u8..=100 {
        println!("{i}- {}", fizzbuzz(i))
    }
}
