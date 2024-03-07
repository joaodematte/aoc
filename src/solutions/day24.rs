use std::fs;

type Point = (i128, i128, i128);

struct Hailstone {
    position: Point,
    final_position: Point,
}

impl Hailstone {
    fn new(position: Point, velocity: (i128, i128, i128)) -> Self {
        Hailstone {
            position,
            final_position: (
                position.0 + velocity.0,
                position.1 + velocity.1,
                position.2 + velocity.2,
            ),
        }
    }

    fn get_position(&self) -> Point {
        self.position
    }

    fn get_final_position(&self) -> Point {
        self.final_position
    }

    fn intersects(&self, target_hailstone: &Hailstone) -> bool {
        let test_area_range = 200000000000000.0..=400000000000000.0;

        let target_position = target_hailstone.get_position();
        let target_final_position = target_hailstone.get_final_position();

        let divisor = (self.position.0 - self.final_position.0)
            * (target_position.1 - target_final_position.1)
            - (self.position.1 - self.final_position.1)
                * (target_position.0 - target_final_position.0);

        let p_x_dividend = (self.position.0 * self.final_position.1
            - self.position.1 * self.final_position.0)
            * (target_position.0 - target_final_position.0)
            - (self.position.0 - self.final_position.0)
                * (target_position.0 * target_final_position.1
                    - target_position.1 * target_final_position.0);

        let p_y_dividend = (self.position.0 * self.final_position.1
            - self.position.1 * self.final_position.0)
            * (target_position.1 - target_final_position.1)
            - (self.position.1 - self.final_position.1)
                * (target_position.0 * target_final_position.1
                    - target_position.1 * target_final_position.0);

        if divisor == 0 {
            return false;
        }

        let x = p_x_dividend as f64 / divisor as f64;
        let y = p_y_dividend as f64 / divisor as f64;

        if !test_area_range.contains(&x)
            || !test_area_range.contains(&y)
            || (x > self.position.0 as f64) != (self.final_position.0 > self.position.0)
            || (x > target_position.0 as f64) != (target_final_position.0 > target_position.0)
        {
            return false;
        }

        true
    }
}

pub fn resolve_part_one(file_content: String) -> i128 {
    let lines: Vec<&str> = file_content.lines().collect();

    let mut hailstones: Vec<Hailstone> = Vec::new();

    for line in lines {
        let (positions, velocities) = line.split_once(" @ ").unwrap();

        let position_vec: Vec<i128> = positions
            .split(", ")
            .map(|num| num.trim().parse::<i128>().unwrap())
            .collect();

        let velocities_vec: Vec<i128> = velocities
            .split(", ")
            .map(|num| num.trim().parse::<i128>().unwrap())
            .collect();

        let position = (position_vec[0], position_vec[1], position_vec[2]);
        let velocity = (velocities_vec[0], velocities_vec[1], velocities_vec[2]);
        let hailstone = Hailstone::new(position, velocity);

        hailstones.push(hailstone);
    }

    let mut result = 0;

    for (index, hailstone) in hailstones.iter().enumerate() {
        let _position = hailstone.get_position();
        let _final_position = hailstone.get_final_position();

        let rest = &hailstones[index + 1..hailstones.len()];

        for target in rest {
            if hailstone.intersects(target) {
                result += 1;
            }
        }
    }

    result
}
