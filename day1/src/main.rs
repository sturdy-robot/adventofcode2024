use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> String {
    let mut first_list = Vec::new();
    let mut second_list = Vec::new();
    for line in input.lines() {
        if let Some((first, second)) = line.split_once("   ") {
            if let (Ok(num1), Ok(num2)) = (first.parse::<i32>(), second.parse::<i32>()) {
                first_list.push(num1);
                second_list.push(num2);
            }
        }
    }

    first_list.sort();
    second_list.sort();
    let mut distances = Vec::new();
    for (first, second) in first_list.iter().zip(second_list.iter()) {
        let distance = (first - second).abs();
        distances.push(distance);
    }
    let result: i32 = distances.iter().sum();
    result.to_string()
}

fn part2(input: &str) -> String {
    let mut first_list = Vec::new();
    let mut second_list = Vec::new();
    for line in input.lines() {
        if let Some((first, second)) = line.split_once("   ") {
            if let (Ok(num1), Ok(num2)) = (first.parse::<i32>(), second.parse::<i32>()) {
                first_list.push(num1);
                second_list.push(num2);
            }
        }
    }

    let mut matching_numbers = HashMap::new();
    for &num in &second_list {
        *matching_numbers.entry(num).or_insert(0) += 1;
    }
    let result: i32 = first_list
        .iter()
        .map(|&num| *matching_numbers.get(&num).unwrap_or(&0) * num)
        .sum();
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!(part1(input), "11");
    }

    #[test]
    fn test_part2() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!(part2(input), "31");
    }
}
