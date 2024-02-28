const STRING_DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn parse_line(line: &str, only_digits: bool) -> u32 {
    let mut first_digit = 0;
    let mut last_digit = 0;

    for (char_i, char) in line.chars().enumerate() {
        if char.is_numeric() {
            last_digit = char.to_digit(10).unwrap();
        } else if !only_digits {
            for (str_digit_i, str_digit) in STRING_DIGITS.iter().enumerate() {
                if line[char_i..].starts_with(str_digit) {
                    match u32::try_from(str_digit_i) {
                        Ok(number) => last_digit = number + 1,
                        Err(_) => println!("index {} is out of range for u32", str_digit_i),
                    }
                }
            }
        }

        if first_digit == 0 {
            first_digit = last_digit;
        }
    }

    first_digit * 10 + last_digit
}

pub fn resolve_part_one(file_content: String) -> u32 {
    let result = file_content
        .split("\n")
        .fold(0, |acc, line| acc + parse_line(line, true));

    result
}

pub fn resolve_part_two(file_content: String) -> u32 {
    let result = file_content
        .split("\n")
        .fold(0, |acc, line| acc + parse_line(line, false));

    result
}
