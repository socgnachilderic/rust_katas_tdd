use tdd_katas::roman_numerals_kata::convert;

fn main() {
    for i in 1..=100 {
        format(i, 10);
    }

    for i in (150..=1000).step_by(50) {
        format(i, 550)
    }
}

fn format(i: i32, sep: i32) {
    println!("{i} = {}", convert(i));

    if i % sep == 0 {
        println!()
    }
}
