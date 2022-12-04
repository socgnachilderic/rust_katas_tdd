use super::{Pos, GRID_SIZE};

pub struct LightsGrid([[bool; GRID_SIZE]; GRID_SIZE]);

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

    pub fn brightness(&self) -> usize {
        self.0.iter().flatten().filter(|light| **light).count()
    }

    fn turn_light(&mut self, from: Pos, to: Pos, action: LightAction) {
        for y in from.1..=to.1 {
            for x in from.0..=to.0 {
                self.0[y][x] = match action {
                    LightAction::TurnOn => true,
                    LightAction::TurnOff => false,
                    LightAction::Toggle => !self.0[y][x],
                }
            }
        }
    }
}

impl Default for LightsGrid {
    fn default() -> Self {
        Self([[false; GRID_SIZE]; GRID_SIZE])
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
    fn should_be_1000000_when_turn_on_0_0_through_999_999() {
        let mut lights_grid = LightsGrid::default();

        lights_grid.turn_on(Pos(0, 0), Pos(999, 999));

        assert_eq!(1_000_000, lights_grid.brightness());
    }

    #[test]
    fn should_be_1000_when_toggle_0_0_through_999_0() {
        let mut lights_grid = LightsGrid::default();

        lights_grid.toggle(Pos(0, 0), Pos(999, 0));

        assert_eq!(1_000, lights_grid.brightness())
    }

    #[test]
    fn should_be_999996_when_toggle_499_499_through_500_500() {
        let mut lights_grid = LightsGrid::default();
        lights_grid.turn_on(Pos(0, 0), Pos(999, 999));

        lights_grid.turn_off(Pos(499, 499), Pos(500, 500));

        assert_eq!(1_000_000 - 4, lights_grid.brightness())
    }
}
