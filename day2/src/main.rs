fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> String {
    let reports = input.lines().collect::<Vec<&str>>();
    let result: Vec<Vec<i32>> = reports
        .iter()
        .map(|x| {
            x.split_whitespace()
                .map(|y| y.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .filter(|x| {
            let is_ascending = x.windows(2).all(|w| {
                let diff = (w[0] - w[1]).abs();
                w[0] <= w[1] && diff <= 3 && diff > 0
            });
            let is_descending = x.windows(2).all(|w| {
                let diff = (w[0] - w[1]).abs();
                w[0] >= w[1] && diff <= 3 && diff > 0
            });
            is_ascending || is_descending
        })
        .collect::<Vec<_>>();
    result.len().to_string()
}

fn list_is_sorted(list: &Vec<i32>) -> bool {
    let is_ascending = list.windows(2).all(|w| {
        let diff = (w[0] - w[1]).abs();
        w[0] <= w[1] && diff <= 3 && diff > 0
    });
    let is_descending = list.windows(2).all(|w| {
        let diff = (w[0] - w[1]).abs();
        w[0] >= w[1] && diff <= 3 && diff > 0
    });

    is_ascending || is_descending
}

fn list_can_be_sorted(list: &Vec<i32>) -> bool {
    for i in 0..list.len() {
        let mut temp_list = list.clone();
        temp_list.remove(i);

        if list_is_sorted(&temp_list) {
            return true;
        }
    }
    false
}

fn part2(input: &str) -> String {
    let reports = input.lines().collect::<Vec<&str>>();
    let result: Vec<Vec<i32>> = reports
        .iter()
        .map(|x| {
            x.split_whitespace()
                .map(|y| y.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .filter(|x| list_is_sorted(x) || list_can_be_sorted(x))
        .collect::<Vec<_>>();
    result.len().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!(part1(input), "2");
    }

    #[test]
    fn test_part2() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!(part2(input), "4");
    }
}
