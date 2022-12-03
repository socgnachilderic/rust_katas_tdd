use tdd_katas::christmas_lights_kata::*;

fn main() {
    print_part1();
    print_part2();
}

fn print_part1() {
    let mut lights_grid = part1::LightsGrid::default();

    lights_grid.turn_on(Pos(887, 9), Pos(959, 629));
    lights_grid.turn_on(Pos(454, 398), Pos(844, 448));
    lights_grid.turn_off(Pos(539, 243), Pos(559, 965));
    lights_grid.turn_off(Pos(370, 819), Pos(676, 868));
    lights_grid.turn_off(Pos(145, 40), Pos(370, 997));
    lights_grid.turn_off(Pos(301, 3), Pos(808, 453));
    lights_grid.turn_on(Pos(351, 678), Pos(951, 908));
    lights_grid.toggle(Pos(720, 196), Pos(897, 994));
    lights_grid.toggle(Pos(831, 394), Pos(904, 860));

    println!(
        "Part 1 : number of lights which are lit is {}",
        lights_grid.brightness()
    )
}

fn print_part2() {
    let mut lights_grid = part2::LightsGrid::default();

    lights_grid.turn_on(Pos(887, 9), Pos(959, 629));
    lights_grid.turn_on(Pos(454, 398), Pos(844, 448));
    lights_grid.turn_off(Pos(539, 243), Pos(559, 965));
    lights_grid.turn_off(Pos(370, 819), Pos(676, 868));
    lights_grid.turn_off(Pos(145, 40), Pos(370, 997));
    lights_grid.turn_off(Pos(301, 3), Pos(808, 453));
    lights_grid.turn_on(Pos(351, 678), Pos(951, 908));
    lights_grid.toggle(Pos(720, 196), Pos(897, 994));
    lights_grid.toggle(Pos(831, 394), Pos(904, 860));

    println!(
        "Part 2 : number of lights which are lit is {}",
        lights_grid.brightness()
    )
}
