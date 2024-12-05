fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn get_rules_for_number(list: &Vec<Vec<usize>>, number: usize) -> Vec<usize> {
    let mut result = Vec::new();
    for (i, rule) in list.iter().enumerate() {
        if rule[0] == number {
            result.push(rule[1]);
        }
    }

    result
}

fn part1(input: &str) -> String {
    let text = input.split("\n\n").collect::<Vec<&str>>();
    let rules = text[0]
        .lines()
        .map(|x| {
            x.split("|")
                .map(|x| x.trim().parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();
    let numbers = text[1]
        .lines()
        .map(|x| {
            x.split(',')
                .map(|x| x.trim().parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();
    let mut ordered_lines = Vec::new();
    for line in numbers.iter() {
        let mut ordered = true;
        for (i, number) in line.iter().enumerate() {
            for j in i..line.len() {
                let rule = get_rules_for_number(&rules, line[j]);
                if rule.contains(&number) && j > i {
                    ordered = false;
                    break;
                }
            }
            if !ordered {
                break;
            }
        }
        if ordered {
            ordered_lines.push(line);
        }
    }
    let mut count = 0;
    for line in ordered_lines.iter() {
        let mid_line = line.len() / 2;
        count += line[mid_line];
    }
    count.to_string()
}

fn part2(input: &str) -> String {
    let text = input.split("\n\n").collect::<Vec<&str>>();
    let rules = text[0]
        .lines()
        .map(|x| {
            x.split("|")
                .map(|x| x.trim().parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();
    let numbers = text[1]
        .lines()
        .map(|x| {
            x.split(',')
                .map(|x| x.trim().parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();
    let mut not_ordered_lines = Vec::new();
    for line in numbers.iter() {
        let mut ordered = true;
        for (i, number) in line.iter().enumerate() {
            for j in i..line.len() {
                let rule = get_rules_for_number(&rules, line[j]);
                if rule.contains(&number) && j > i {
                    ordered = false;
                    break;
                }
            }
            if !ordered {
                break;
            }
        }
        if !ordered {
            not_ordered_lines.push(line);
        }
    }

    let new_ordered_lines: Vec<Vec<usize>> = not_ordered_lines
        .into_iter()
        .map(|line| {
            let mut line = line.clone(); // Clone the line vector
            line.sort_by(|a, b| {
                let rule_a = get_rules_for_number(&rules, *a);

                if rule_a.contains(b) {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Greater
                }
            });
            line // Return a reference to the sorted vector
        })
        .collect();
    let mut count = 0;
    for line in new_ordered_lines.iter() {
        let mid_line = line.len() / 2;
        count += line[mid_line];
    }
    count.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        assert_eq!(part1(input), "143");
    }

    // #[test]
    // fn test_part1_with_input() {
    // let input = include_str!("../input.txt");
    // assert_eq!(part1(input), "2065338");
    // }

    #[test]
    fn test_part2() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        assert_eq!(part2(input), "123");
    }

    //     #[test]
    //     fn test_part2_with_input() {
    //         let input = include_str!("../input.txt");
    //         assert_eq!(part2(input), "34934171");
    //     }
}
