use super::{Pos, GRID_SIZE};

pub struct LightsGrid([[u8; GRID_SIZE]; GRID_SIZE]);

impl LightsGrid {
    pub fn turn_on(&mut self, from: Pos, to: Pos) {
        self.turn_light(from, to, LightAction::TurnOn)
    }
    pub fn turn_off(&mut self, from: Pos, to: Pos) {
        self.turn_light(from, to, LightAction::TurnOff)
    }

    pub fn toggle(&mut self, from: Pos, to: Pos) {
        self.turn_light(from, to, LightAction::Toggle)
    }

    pub fn brightness(&self) -> u32 {
        self.0
            .iter()
            .flatten()
            .map(|num| {
                let num: u32 = (*num).into();

                num
            })
            .sum()
    }

    fn turn_light(&mut self, from: Pos, to: Pos, action: LightAction) {
        for y in from.1..=to.1 {
            for x in from.0..=to.0 {
                let cell = self.0[y][x];

                self.0[y][x] = match action {
                    LightAction::TurnOn => cell + 1,
                    LightAction::TurnOff => {
                        if cell != 0 {
                            cell - 1
                        } else {
                            0
                        }
                    }
                    LightAction::Toggle => cell + 2,
                }
            }
        }
    }
}

impl Default for LightsGrid {
    fn default() -> Self {
        Self([[0; GRID_SIZE]; GRID_SIZE])
    }
}

enum LightAction {
    TurnOn,
    TurnOff,
    Toggle,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_be_1_when_toggle_0_0_through_0_0() {
        let mut lights_grid = LightsGrid::default();

        lights_grid.turn_on(Pos(0, 0), Pos(0, 0));

        assert_eq!(1, lights_grid.brightness())
    }

    #[test]
    fn should_be_2000000_when_turn_on_0_0_through_999_999() {
        let mut lights_grid = LightsGrid::default();

        lights_grid.toggle(Pos(0, 0), Pos(999, 999));

        assert_eq!(2_000_000, lights_grid.brightness());
    }
}
