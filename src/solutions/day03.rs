use std::collections::HashSet;

struct Number {
    value: i64,
    positions: HashSet<(i64, i64)>,
}

impl Number {
    fn new(chr: char, row: i64, col: i64) -> Self {
        Number {
            value: (chr as u8 - b'0') as i64,
            positions: HashSet::from([
                (row - 1, col - 1),
                (row, col - 1),
                (row + 1, col - 1), // left hand side
                (row - 1, col),
                (row + 1, col), // above and below
                (row - 1, col + 1),
                (row, col + 1),
                (row + 1, col + 1), // right hand side
            ]),
        }
    }

    fn add_digit(&mut self, chr: char, row: i64, col: i64) {
        self.value = self.value * 10 + (chr as u8 - b'0') as i64;
        self.positions
            .extend([(row - 1, col + 1), (row, col + 1), (row + 1, col + 1)]);
    }

    fn get_value(&self) -> i64 {
        self.value
    }

    fn is_next_to_symbol(&self, symbols: &HashSet<(i64, i64)>) -> bool {
        self.positions.intersection(symbols).next().is_some()
    }
}

pub fn resolve_part_one(file_content: String) -> i64 {
    let lines: Vec<&str> = file_content.lines().collect();
    let mut matrix: Vec<Vec<char>> = Vec::new();

    let mut symbols: HashSet<(i64, i64)> = HashSet::new();
    let mut gears: HashSet<(i64, i64)> = HashSet::new();
    let mut numbers: Vec<Number> = Vec::new();

    for line in lines {
        let chars: Vec<char> = line.chars().collect();

        matrix.push(chars);
    }

    let mut current_number: Option<Number> = None;

    for (row, line) in matrix.iter().enumerate() {
        for (col, chr) in line.iter().enumerate() {
            if !chr.is_ascii_digit() {
                if let Some(num) = current_number.take() {
                    numbers.push(num);
                }

                if chr == &'.' {
                    continue;
                }

                if chr == &'*' {
                    gears.insert((row as i64, col as i64));
                }

                symbols.insert((row as i64, col as i64));
            } else {
                if let Some(ref mut num) = current_number {
                    num.add_digit(*chr, row as i64, col as i64);
                } else {
                    current_number = Some(Number::new(*chr, row as i64, col as i64));
                }
            }
        }
    }

    let result: i64 = numbers
        .iter()
        .filter(|num| num.is_next_to_symbol(&symbols))
        .map(Number::get_value)
        .sum();

    // let result_part_two: i64 = gears.iter().fold(0, |count, gear| {
    //     let mut gear_numbers: Vec<i64> = Vec::new();

    //     for number in numbers.iter() {
    //         if number.positions.contains(gear) {
    //             gear_numbers.push(number.get_value());
    //         }
    //     }

    //     if gear_numbers.len() == 2 {
    //         return count + gear_numbers[0] * gear_numbers[1];
    //     }

    //     count
    // });

    result
}
