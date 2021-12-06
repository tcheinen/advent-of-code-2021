use bitvec::prelude::*;
use std::ops::Sub;
use tap::{Pipe, Tap};

/// https://adventofcode.com/2021/day/3
#[aoc_generator(day3)]
pub fn generator(input: &str) -> Vec<BitVec> {
    input
        .lines()
        .map(|x| {
            x.bytes()
                .map(|x| if x == '1' as u8 { true } else { false })
                .collect()
        })
        .collect()
}

fn most_common_bits(input: &[BitVec], len: usize, negate: bool) -> BitVec {
    input
        .into_iter()
        .fold(vec![0; len], |sum, next| {
            sum.into_iter()
                .enumerate()
                .map(|(x, i)| i + (if next[x] { 1 } else { 0 }))
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
        .map(|x| if negate { !x } else { x })
        .collect()
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &[BitVec]) -> usize {
    input
        .pipe(|x| most_common_bits(x, x[0].len(), false))
        .into_iter()
        .rev()
        .enumerate()
        .fold(0, |sum, (idx, next)| sum | ((next as usize) << idx))
        .pipe(|x| x * (!x & 2usize.pow(input[0].len() as u32).sub(1)))
}

fn filter_by_matching_bitslice(
    input: Vec<BitVec>,
    idx: usize,
    filter: &BitSlice,
    negate: bool,
) -> Vec<BitVec> {
    if input.len() == 1 {
        input
    } else {
        input
            .into_iter()
            .filter(|x| x[idx] == filter[idx])
            .collect::<Vec<_>>()
            .pipe(|x| {
                filter_by_matching_bitslice(
                    x.clone(),
                    idx + 1,
                    &most_common_bits(&x, x[0].len(), negate),
                    negate,
                )
            })
    }
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &[BitVec]) -> usize {
    fn calc(input: &[BitVec], negate: bool) -> usize {
        input.pipe(|x| {
            filter_by_matching_bitslice(
                x.to_vec(),
                0,
                &most_common_bits(input, input[0].len(), negate),
                negate,
            )
        })[0]
            .clone()
            .into_iter()
            .rev()
            .enumerate()
            .fold(0, |sum, (idx, next)| sum | ((next as usize) << idx))
    }
    calc(input, false) * calc(input, true)
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
            let control = bitvec![0, 0, 0, 0, 0];
            let provided = "00000";
            assert_eq!(&control, &most_common_bits(&generator(provided), 5, false))
        }
        {
            let control = bitvec![0, 0, 0, 0, 1];
            let provided = "00000\n00001\n00001";
            assert_eq!(&control, &most_common_bits(&generator(provided), 5, false))
        }
        {
            let control = bitvec![0, 1, 0, 0, 1];
            let provided = "01000\n01001\n00001";
            assert_eq!(&control, &most_common_bits(&generator(provided), 5, false))
        }
        {
            let control = bitvec![0, 1, 0, 0, 1];
            let provided = "01000\n01001";
            assert_eq!(&control, &most_common_bits(&generator(provided), 5, false))
        }
    }

    #[test]
    fn test_generate() {
        {
            let provided = "11111";
            assert_eq!(vec![bitvec![1, 1, 1, 1, 1]], generator(provided));
        }

        {
            // vec[0] is msb
            let provided = "11110";
            assert_eq!(vec![bitvec![1, 1, 1, 1, 0]], generator(provided));
        }
    }
}
