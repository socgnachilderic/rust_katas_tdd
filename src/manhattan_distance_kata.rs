pub fn manhattan_distance(point1: Point, point2: Point) -> f64 {
    point1.manhattan_distance_with(point2)
}

pub struct Point(f64, f64);

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self(x, y)
    }

    pub fn manhattan_distance_with(&self, other: Self) -> f64 {
        (self.0 - other.0).abs() + (self.1 - other.1).abs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_be_0_when_0_0_and_0_0() {
        let point1 = Point::new(0., 0.);
        let point2 = Point::new(0., 0.);

        assert_eq!(0., manhattan_distance(point1, point2))
    }

    #[test]
    fn should_be_0_when_1_1_and_1_1() {
        let point1 = Point::new(1., 1.);
        let point2 = Point::new(1., 1.);

        assert_eq!(0., manhattan_distance(point1, point2))
    }

    #[test]
    fn should_be_4_when_5_4_and_3_2() {
        let point1 = Point::new(5., 4.);
        let point2 = Point::new(3., 2.);

        assert_eq!(4., manhattan_distance(point1, point2))
    }

    #[test]
    fn should_be_3_when_1_1_and_0_3() {
        let point1 = Point::new(1., 1.);
        let point2 = Point::new(0., 3.);

        assert_eq!(3., manhattan_distance(point1, point2))
    }

    #[test]
    fn should_be_1_when_1_0_and_0_0() {
        let point1 = Point::new(1., 0.);
        let point2 = Point::new(0., 0.);

        assert_eq!(1., manhattan_distance(point1, point2))
    }

    #[test]
    fn should_be_8_when_165_64ne_and_378ne_82() {
        let point1 = Point::new(165., -64.);
        let point2 = Point::new(-378., 82.);

        assert_eq!(689., manhattan_distance(point1, point2))
    }
}
