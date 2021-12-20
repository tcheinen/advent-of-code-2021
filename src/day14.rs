use std::collections::HashMap;
use std::hash::Hash;
use std::str::Split;

use crate::{frequence_accumulate, frequency};
use dashmap::DashMap;
use itertools::Itertools;
use tap::{Pipe, Tap};

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

fn mutate(
    mut input: HashMap<(char, char), usize>,
    polymers: &HashMap<(char, char), char>,
) -> HashMap<(char, char), usize> {
    fn bind(
        items: impl IntoIterator<Item = ((char, char), usize)>,
        map: &mut HashMap<(char, char), usize>,
    ) -> HashMap<(char, char), usize> {
        items
            .into_iter()
            .for_each(|(k, v)| {
                map.entry(k)
                    .and_modify(|y| *y += v)
                    .or_insert(v)
                    .pipe(|_| ())
            })
            .pipe(|_| std::mem::take(map))
    }
    input
        .iter()
        .flat_map(|(k, v)| {
            if polymers.contains_key(k) {
                polymers
                    .get(k)
                    .unwrap()
                    .pipe(|c| vec![((k.0, *c), *v), ((*c, k.1), *v)])
            } else {
                vec![(*k, *v)]
            }
        })
        .collect_vec()
        .pipe(frequence_accumulate)
    // .tap(|x| println)

    // .pipe(|x| bind(x, &mut HashMap::new()))
}

fn expand(
    (elements, polymer): &(Vec<char>, HashMap<(char, char), char>),
    number: usize,
) -> HashMap<char, usize> {
    elements
        .clone()
        .array_windows::<2>()
        .map(|[a, b]| (*a, *b))
        .collect::<Vec<_>>()
        .pipe(frequency)
        .pipe(|m| (0..number).fold(m, |sum, _| mutate(sum, polymer)))
        .into_iter()
        .map(|((a, b), c)| (a, c))
        .collect_vec()
        .pipe(frequence_accumulate)
        .tap_mut(|x| {
            x.entry(*elements.last().unwrap())
                .and_modify(|y| *y += 1)
                .or_insert(1)
                .pipe(|_| ())
        })
}

#[aoc(day14, part1)]
pub fn solve_part1(input: &(Vec<char>, HashMap<(char, char), char>)) -> usize {
    fn inner(count: HashMap<char, usize>) -> usize {
        count.iter().map(|(a, b)| *b).max().unwrap() - count.iter().map(|(a, b)| *b).min().unwrap()
    }
    expand(input, 10).pipe(inner)
}

#[aoc(day14, part2)]
pub fn solve_part2(input: &(Vec<char>, HashMap<(char, char), char>)) -> usize {
    fn inner(count: HashMap<char, usize>) -> usize {
        count.iter().map(|(a, b)| *b).max().unwrap() - count.iter().map(|(a, b)| *b).min().unwrap()
    }
    expand(input, 40).pipe(inner)
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
        assert_eq!(
            3288891573057,
            solve_part2(&generator(
                include_str!("../input/2021/day14.txt").trim_end()
            ))
        );
    }
}
