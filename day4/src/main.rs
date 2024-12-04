fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> String {
    let text: Vec<Vec<char>> = input
        .lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect();
    let mut count = 0;
    for (i, line) in text.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if c == &'X' {
                if line.get(j + 1) == Some(&'M')
                    && line.get(j + 2) == Some(&'A')
                    && line.get(j + 3) == Some(&'S')
                {
                    count += 1;
                }

                if j >= 3 {
                    if line.get(j - 1) == Some(&'M')
                        && line.get(j - 2) == Some(&'A')
                        && line.get(j - 3) == Some(&'S')
                    {
                        count += 1;
                    }
                }

                if text.get(i + 1).is_some()
                    && text.get(i + 2).is_some()
                    && text.get(i + 3).is_some()
                {
                    if text[i + 1].get(j) == Some(&'M')
                        && text[i + 2].get(j) == Some(&'A')
                        && text[i + 3].get(j) == Some(&'S')
                    {
                        count += 1;
                    }

                    if text[i + 1].get(j + 1) == Some(&'M')
                        && text[i + 2].get(j + 2) == Some(&'A')
                        && text[i + 3].get(j + 3) == Some(&'S')
                    {
                        count += 1;
                    }

                    if j >= 3 {
                        if text[i + 1].get(j - 1) == Some(&'M')
                            && text[i + 2].get(j - 2) == Some(&'A')
                            && text[i + 3].get(j - 3) == Some(&'S')
                        {
                            count += 1;
                        }
                    }
                }

                if i >= 3 {
                    if text.get(i - 1).is_some()
                        && text.get(i - 2).is_some()
                        && text.get(i - 3).is_some()
                    {
                        if text[i - 1].get(j) == Some(&'M')
                            && text[i - 2].get(j) == Some(&'A')
                            && text[i - 3].get(j) == Some(&'S')
                        {
                            count += 1;
                        }

                        if j >= 3 {
                            if text[i - 1].get(j - 1) == Some(&'M')
                                && text[i - 2].get(j - 2) == Some(&'A')
                                && text[i - 3].get(j - 3) == Some(&'S')
                            {
                                count += 1;
                            }
                        }

                        if text[i - 1].get(j + 1) == Some(&'M')
                            && text[i - 2].get(j + 2) == Some(&'A')
                            && text[i - 3].get(j + 3) == Some(&'S')
                        {
                            count += 1;
                        }
                    }
                }
            }
        }
    }
    count.to_string()
}

fn part2(input: &str) -> String {
    let text: Vec<Vec<char>> = input
        .lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect();
    let mut count = 0;
    for (i, line) in text.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if c == &'A' {
                if i > 0 && j > 0 {
                    if text.get(i - 1).is_some() && text.get(i + 1).is_some() {
                        if text[i - 1].get(j - 1) == Some(&'M')
                            && text[i + 1].get(j - 1) == Some(&'M')
                            && text[i - 1].get(j + 1) == Some(&'S')
                            && text[i + 1].get(j + 1) == Some(&'S')
                        {
                            count += 1;
                        }

                        if text[i - 1].get(j + 1) == Some(&'M')
                            && text[i + 1].get(j + 1) == Some(&'M')
                            && text[i - 1].get(j - 1) == Some(&'S')
                            && text[i + 1].get(j - 1) == Some(&'S')
                        {
                            count += 1;
                        }
                        
                        if text[i - 1].get(j - 1) == Some(&'M')
                            && text[i - 1].get(j + 1) == Some(&'M')
                            && text[i + 1].get(j - 1) == Some(&'S')
                            && text[i + 1].get(j + 1) == Some(&'S')
                        {
                            count += 1;
                        }
                        
                        if text[i + 1].get(j - 1) == Some(&'M')
                            && text[i + 1].get(j + 1) == Some(&'M')
                            && text[i - 1].get(j - 1) == Some(&'S')
                            && text[i - 1].get(j + 1) == Some(&'S')
                        {
                            count += 1;
                        }
                        
                    }
                }
            }
        }
    }
    count.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!(part1(input), "18");
    }

    // #[test]
    // fn test_part1_with_input() {
    // let input = include_str!("../input.txt");
    // assert_eq!(part1(input), "2065338");
    // }

    #[test]
    fn test_part2() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!(part2(input), "9");
    }

    //     #[test]
    //     fn test_part2_with_input() {
    //         let input = include_str!("../input.txt");
    //         assert_eq!(part2(input), "34934171");
    //     }
}
