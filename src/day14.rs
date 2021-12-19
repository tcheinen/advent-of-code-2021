use std::collections::HashMap;
use std::hash::Hash;
use std::str::Split;

use dashmap::DashMap;
use itertools::Itertools;
use tap::{Pipe, Tap};
use crate::frequency;

/// https://adventofcode.com/2021/day/14

#[aoc_generator(day14)]
pub fn generator(input: &str) -> (Vec<char>, HashMap<(char, char), char>) {
    fn parse_rule(input: &str) -> ((char, char), char) {
        fn bind(mut input: Split<&str>) -> ((char, char), char) {
            (
                input.next().unwrap().chars().collect_tuple().unwrap(),
                input.next().unwrap().chars().next().unwrap(),
            )
        }
        bind(input.split(" -> "))
    }

    fn inner(mut input: Split<&str>) -> (Vec<char>, HashMap<(char, char), char>) {
        (
            input.next().unwrap().chars().collect(),
            HashMap::new().tap_mut(|map| {
                input
                    .next()
                    .unwrap()
                    .lines()
                    .map(parse_rule)
                    .for_each(|((a, b), c)| map.insert((a, b), c).pipe(|_| ()))
            }),
        )
    }
    inner(input.split("\n\n"))
}

fn mutate(mut input: Vec<char>, polymers: &HashMap<(char, char), char>) -> Vec<char> {
    input.clone()
        .array_windows::<2>()
        .map(|[a, b]| (*a, *b))
        .enumerate()
        .filter_map(|(idx, chars)| Some((idx, polymers.get(&chars)?)))
        .enumerate()
        .for_each(|(offset, (idx, result))| {
            input.insert(offset + idx + 1, *result).pipe(|_|())
        }).pipe(|_| input)
}

#[aoc(day14, part1)]
pub fn solve_part1((elements, polymer): &(Vec<char>, HashMap<(char, char), char>)) -> usize {
    fn inner(count: HashMap<char, usize>) -> usize {
        count.iter().map(|(a,b)| *b).max().unwrap() - count.iter().map(|(a,b)| *b).min().unwrap()
    }
    (0..10).fold(elements.clone(), |sum, _| {mutate(sum, polymer)}).pipe(frequency).pipe(inner)
}

#[aoc(day14, part2)]
pub fn solve_part2((elements, polymer): &(Vec<char>, HashMap<(char, char), char>)) -> usize {
    fn inner(count: HashMap<char, usize>) -> usize {
        count.iter().map(|(a,b)| *b).max().unwrap() - count.iter().map(|(a,b)| *b).min().unwrap()
    }
    (0..40).fold(elements.clone(), |sum, _| {mutate(sum, polymer)}).pipe(frequency).pipe(inner)
}

#[cfg(test)]
mod tests {
    use super::*;

        #[test]
        fn it_works_part1() {
            let provided = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

            assert_eq!(1588, solve_part1(&generator(provided)));
            assert_eq!(
                3048,
                solve_part1(&generator(
                    include_str!("../input/2021/day14.txt").trim_end()
                ))
            );
        }
    #[test]
    fn it_works_part2() {
        let provided = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

        assert_eq!(2188189693529, solve_part2(&generator(provided)));
        // assert_eq!(
        //     3048,
        //     solve_part2(&generator(
        //         include_str!("../input/2021/day14.txt").trim_end()
        //     ))
        // );
    }

}
