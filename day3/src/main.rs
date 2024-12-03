use regex::Regex;

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}


fn part1(input: &str) -> String {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let result: i64 = re.captures_iter(input).map(|cap| {
        let n1 = cap.get(1).unwrap().as_str().parse::<i64>().unwrap();
        let n2 = cap.get(2).unwrap().as_str().parse::<i64>().unwrap();
        n1 * n2
    }).sum();
    result.to_string()
}

fn part2(input: &str) -> String {
    let mut text = input.to_string();
    let dos = text.match_indices("do()").map(|(i, _)| i).collect::<Vec<usize>>();
    let donts = text.match_indices("don't()").map(|(i, _)| i).collect::<Vec<usize>>();
    let mut new_matches = Vec::new();
    for dn in donts.iter() {
        let mut found = false;
        for d in dos.iter() {
            if d > dn {
                found = true;
                new_matches.push(&text[*dn..*d]);
                break;
            }
        }
        if !found {
            new_matches.push(&text[*dn..]);
        }
    }
    let mut new_text = text.clone();
    for m in new_matches.iter() {
        new_text = new_text.replace(*m, "");
    }
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    println!("{}", new_text);
    let result: i64 = re.captures_iter(new_text.as_str()).map(|cap| {
        let n1 = cap.get(1).unwrap().as_str().parse::<i64>().unwrap();
        let n2 = cap.get(2).unwrap().as_str().parse::<i64>().unwrap();
        n1 * n2
    }).sum();
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(part1(input), "161");
    }

    // #[test]
    // fn test_part1_with_input() {
    // let input = include_str!("../input.txt");
    // assert_eq!(part1(input), "2065338");
    // }

    #[test]
    fn test_part2() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(part2(input), "48");
    }

    //     #[test]
    //     fn test_part2_with_input() {
    //         let input = include_str!("../input.txt");
    //         assert_eq!(part2(input), "34934171");
    //     }
}
