use aoc::solutions;
use std::fs;

fn main() {
    // let file_content =
    //     fs::read_to_string("./inputs/day01.txt").expect("Should have been able to read the file");

    // let result_part_one = solutions::day01::resolve_part_one(file_content.clone());
    // let result_part_two = solutions::day01::resolve_part_two(file_content);

    // println!("Result of part one: {}", result_part_one);
    // println!("Result of part two: {}", result_part_two);

    // let file_content =
    //     fs::read_to_string("./inputs/day02.txt").expect("Should have been able to read the file");

    // let result_part_one = solutions::day02::resolve_part_one(file_content.clone());
    // let result_part_two = solutions::day02::resolve_part_two(file_content);

    // println!("Result of part one: {}", result_part_one);
    // println!("Result of part two: {}", result_part_two);

    // let file_content =
    //     fs::read_to_string("./inputs/day03.txt").expect("Should have been able to read the file");
    //
    // let result_part_one = solutions::day03::resolve_part_one(file_content);
    //
    // println!("Result of part one: {}", result_part_one);

    let file_content =
        fs::read_to_string("./inputs/day24.txt").expect("Should have been able to read the file");

    let result_part_one = solutions::day24::resolve_part_one(file_content);

    println!("Result of part one: {}", result_part_one);
}
