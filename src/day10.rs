use itertools::Itertools;
use tap::Pipe;

/// https://adventofcode.com/2021/day/10

#[aoc_generator(day10)]
pub fn generator(input: &str) -> Vec<String> {
    input.lines().map(String::from).collect()
}

fn find_first_wrong(input: &str) -> (Option<char>, Vec<char>) {
    fn inner(input: &[char], stack: &mut Vec<char>) -> Option<char> {
        if input.len() == 0 {
            None
        } else {
            match input[0] {
                '(' | '[' | '{' | '<' => stack.push(input[0]).pipe(|_| inner(&input[1..], stack)),
                _ if stack.len() == 0 => Some(input[0]),
                ')' => {
                    if stack.last().expect("starts with closing bracket") == &'(' {
                        stack.pop().pipe(|_| inner(&input[1..], stack))
                    } else {
                        Some(input[0])
                    }
                }
                ']' => {
                    if stack.last().expect("starts with closing bracket") == &'[' {
                        stack.pop().pipe(|_| inner(&input[1..], stack))
                    } else {
                        Some(input[0])
                    }
                }
                '}' => {
                    if stack.last().expect("starts with closing bracket") == &'{' {
                        stack.pop().pipe(|_| inner(&input[1..], stack))
                    } else {
                        Some(input[0])
                    }
                }
                '>' => {
                    if stack.last().expect("starts with closing bracket") == &'<' {
                        stack.pop().pipe(|_| inner(&input[1..], stack))
                    } else {
                        Some(input[0])
                    }
                }
                _ => panic!("invalid input"),
            }
        }
    }
    fn bind(input: &[char], stack: &mut Vec<char>) -> (Option<char>, Vec<char>) {
        (inner(input, stack), stack.clone())
    }
    bind(&input.chars().collect::<Vec<_>>(), &mut Vec::new())
}

#[aoc(day10, part1)]
pub fn solve_part1(input: &[String]) -> usize {
    input
        .into_iter()
        .map(String::as_ref)
        .map(find_first_wrong)
        .flat_map(|x| x.0)
        .map(|x| match x {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => panic!("should never happen"),
        })
        .sum()
}

fn reverse(input: Vec<char>) -> Vec<char> {
    input
        .into_iter()
        .rev()
        .map(|x| match x {
            '(' => ')',
            '[' => ']',
            '{' => '}',
            '<' => '>',
            _ => panic!("lol"),
        })
        .collect()
}

fn get_median(input: &[i64]) -> i64 {
    *input.into_iter().sorted().nth(input.len() / 2).unwrap()
}

#[aoc(day10, part2)]
pub fn solve_part2(input: &[String]) -> i64 {
    input
        .into_iter()
        .map(String::as_ref)
        .map(find_first_wrong)
        .filter(|(a, b)| a.is_none() && !b.is_empty())
        .map(|(_, b)| b)
        .map(reverse)
        .map(|line| {
            line.into_iter()
                .map(|x| match x {
                    ')' => 1,
                    ']' => 2,
                    '}' => 3,
                    '>' => 4,
                    _ => panic!("should never happen"),
                })
                .fold(0, |sum, next| sum * 5 + next)
        })
        .collect::<Vec<_>>()
        .pipe(|x| get_median(&x))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_part1() {
        let provided = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";
        assert_eq!(26397, solve_part1(&generator(provided)));
        assert_eq!(
            345441,
            solve_part1(&generator(include_str!("../input/2021/day10.txt")))
        );
    }

    #[test]
    fn it_works_part2() {
        let provided = "[({(<(())[]>[[{[]{<()<>>
    [(()[<>])]({[<{<<[]>>(
    {([(<{}[<>[]}>{[]{[(<()>
    (((({<>}<{<{<>}{[]{[]{}
    [[<[([]))<([[{}[[()]]]
    [{[{({}]{}}([{[{{{}}([]
    {<[[]]>}<{[{[{[]{()[[[]
    [<(<(<(<{}))><([]([]()
    <{([([[(<>()){}]>(<<{{
    <{([{{}}[<[[[<>{}]]]>[]]";

        assert_eq!(288957, solve_part2(&generator(provided)));
        assert_eq!(
            3235371166,
            solve_part2(&generator(include_str!("../input/2021/day10.txt")))
        );
    }

    #[test]
    fn it_counts() {
        assert_eq!(None, find_first_wrong("<<<>>>").0);
        assert_eq!(Some(']'), find_first_wrong("<<<>>>]").0);
        assert_eq!(Some('}'), find_first_wrong("{([(<{}[<>[]}>{[]{[(<()>").0);
    }
}
