use std::ops::Sub;
use tap::{Pipe, Tap};

/// https://adventofcode.com/2021/day/3
#[aoc_generator(day3)]
pub fn generator(input: &str) -> Vec<String> {
    input
        .split("\n")
        .map(String::from)
        .filter(|x| !x.is_empty())
        .collect()
}

fn most_common_bits(input: &[String]) -> Vec<bool> {
    input
        .into_iter()
        .fold(vec![0; input[0].len()], |sum, next| {
            sum.clone()
                .into_iter()
                .enumerate()
                .map(|(x, i)| {
                    i + (if next.as_bytes()[x] == '1' as u8 {
                        1
                    } else {
                        0
                    })
                })
                .collect::<Vec<_>>()
        })
        .into_iter()
        .map(|y| {
            if y * 2 == input.len() {
                true
            } else {
                y > (input.len() / 2)
            }
        })
        // .rev()
        .collect()
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &[String]) -> usize {
    input
        .pipe(most_common_bits)
        .into_iter()
        .rev()
        .enumerate()
        .fold(0, |sum, (idx, next)| sum | ((next as usize) << idx))
        .pipe(|x| x * (!x & 2usize.pow(input[0].len() as u32).sub(1)))
}

fn recurse(input: Vec<Vec<bool>>, idx: usize, filter: &[bool], negate: bool) -> Vec<Vec<bool>> {
    fn inner(input: &[Vec<bool>], negate: bool) -> Vec<bool> {
        input
            .into_iter()
            .fold(vec![0; input[0].len()], |sum, next| {
                sum.clone()
                    .into_iter()
                    .enumerate()
                    .map(|(x, i)| i + (if next[x] { (1) as usize } else { 0 as usize }))
                    .collect::<Vec<_>>()
            })
            .into_iter()
            .map(|y| {
                if y * 2 == input.len() {
                    true
                } else {
                    y > (input.len() / 2)
                }
            })
            // .rev()
            .map(|x| if negate { !x } else { x })
            .collect::<Vec<bool>>()
    }
    if input.len() == 1 {
        // vec![input[0].iter().map(|x| !x).collect()]
        input
    } else {
        input
            .into_iter()
            .filter(|x| x[idx] == filter[idx])
            .collect::<Vec<Vec<bool>>>()
            .pipe(|x| recurse(x.clone(), idx + 1, &inner(&x, negate), negate))
    }
}

// fm cp

#[aoc(day3, part2)]
pub fn solve_part2(input: &[String]) -> usize {
    fn oxygen(input: &[String]) -> usize {
        input
            .into_iter()
            .map(|x| {
                x.clone()
                    .bytes()
                    .enumerate()
                    .map(|(x, i)| (if i == '1' as u8 { true } else { false }))
                    .collect()
            })
            .collect::<Vec<Vec<bool>>>()
            .pipe(|x| recurse(x, 0, &most_common_bits(input), false))[0]
            .clone()
            .into_iter()
            .rev()
            .enumerate()
            .fold(0, |sum, (idx, next)| sum | { ((next as usize) << idx) })
    }

    fn co2(input: &[String]) -> usize {
        input
            .into_iter()
            .map(|x| {
                x.clone()
                    .bytes()
                    .enumerate()
                    .map(|(x, i)| (if i == '1' as u8 { true } else { false }))
                    .collect()
            })
            .collect::<Vec<Vec<bool>>>()
            .pipe(|x| recurse(x, 0, &(most_common_bits(input).into_iter().map(|x| !x).collect::<Vec<_>>()), true))[0]
            .clone()
            .into_iter()
            .rev()
            .enumerate()
            .fold(0, |sum, (idx, next)| sum | { ((next as usize) << idx) })
    }

    oxygen(input) * co2(input)
    // recurse(
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_part1() {
        let provided = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";
        assert_eq!(198, solve_part1(&generator(provided)));
        assert_eq!(
            738234,
            solve_part1(&generator(include_str!("../input/2021/day3.txt")))
        );
    }

    #[test]
    fn it_works_part2() {
        let provided = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";
        assert_eq!(230, solve_part2(&generator(provided)));
        //         assert_eq!(
        //             1518,
        //             solve_part2(&generator(include_str!("../input/2021/day1.txt")))
        //         );
    }

    #[test]
    fn test_most_common_bits() {
        {
            let control = [false, false, false, false, false];
            let provided = vec!["00000".to_owned()];
            assert_eq!(control.as_slice(), most_common_bits(&provided).as_slice())
        }
        {
            let control = [false, false, false, false, true];
            let provided = ["00000", "00001", "00001"]
                .into_iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>();
            assert_eq!(control.as_slice(), most_common_bits(&provided).as_slice())
        }
        {
            let control = [false, true, false, false, true];
            let provided = ["01000", "01001", "00001"]
                .into_iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>();
            assert_eq!(control.as_slice(), most_common_bits(&provided).as_slice())
        }
        {
            let control = [false, true, false, false, true];
            let provided = ["01000", "01001"]
                .into_iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>();
            assert_eq!(control.as_slice(), most_common_bits(&provided).as_slice())
        }
    }
}
