use core::str::Split;
use maplit::{hashmap, hashset};
use std::collections::{HashMap, HashSet};
use tap::prelude::*;

/// https://adventofcode.com/2021/day/8

// idk if this is correct tbh i made it in the throes of energy drink and then made changes until it works
// 1, 4, 7, 8 can be derived by size alone
// 6 can be derived by being the only size 6 set which does not contain the chars of 1
// diff of 9 and 0 is single char; if we take two size 6 sets remaining and compute a - b and b - a we can derive which is 9 because 9 - 0 is in 4 and 0 - 9 is not
// remaining are 2, 3, 5
// 1 is a subset of 3, leaving 2 and 5
// if you remove the common chars of 2 and 5 the diff is be and ag; diff of 5 is a subset of 4 and 2 is the only remaining

fn derive_letter(
    known: HashMap<char, HashSet<char>>,
    remaining: Vec<HashSet<char>>,
    target: char,
    f: impl Fn(HashMap<char, HashSet<char>>, Vec<HashSet<char>>) -> HashSet<char>,
) -> (HashMap<char, HashSet<char>>, Vec<HashSet<char>>) {
    fn inner(
        known: HashMap<char, HashSet<char>>,
        remaining: Vec<HashSet<char>>,
    ) -> (HashMap<char, HashSet<char>>, Vec<HashSet<char>>) {
        (
            known.clone(),
            remaining
                .into_iter()
                .filter(|x| known.iter().find(|(_, v)| &x == v).is_none())
                .collect(),
        )
    }
    known
        .tap_mut(|known| {
            known
                .insert(target, f(known.clone(), remaining.clone()))
                .pipe(|_| ())
        })
        .pipe(|x| inner(x, remaining))
}

fn derive_mapping(
    known: HashMap<char, HashSet<char>>,
    remaining: Vec<HashSet<char>>,
) -> HashMap<char, HashSet<char>> {
    (known, remaining)
        .pipe(|(known, remaining)| {
            derive_letter(known, remaining, '1', |_, remaining| {
                remaining.iter().find(|x| x.len() == 2).unwrap().clone()
            })
        })
        .pipe(|(known, remaining)| {
            derive_letter(known, remaining, '4', |_, remaining| {
                remaining.iter().find(|x| x.len() == 4).unwrap().clone()
            })
        })
        .pipe(|(known, remaining)| {
            derive_letter(known, remaining, '7', |_, remaining| {
                remaining.iter().find(|x| x.len() == 3).unwrap().clone()
            })
        })
        .pipe(|(known, remaining)| {
            derive_letter(known, remaining, '8', |_, remaining| {
                remaining.iter().find(|x| x.len() == 7).unwrap().clone()
            })
        })
        .pipe(|(known, remaining)| {
            derive_letter(known, remaining, '6', |known, remaining| {
                remaining
                    .iter()
                    .filter(|x| x.len() == 6)
                    .find(|x| !known.get(&'1').unwrap().is_subset(x))
                    .unwrap()
                    .clone()
            })
        })
        .pipe(|(known, remaining)| {
            derive_letter(known, remaining, '9', |known, remaining| {
                remaining
                    .iter()
                    .filter(|x| x.len() == 6)
                    .cloned()
                    .collect::<Vec<_>>()
                    .pipe(|x| (x[0].clone(), x[1].clone()))
                    .pipe(|(a, b)| {
                        if (&a - &b).is_subset(&known.get(&'4').unwrap()) {
                            a.clone()
                        } else {
                            b.clone()
                        }
                    })
            })
        })
        .pipe(|(known, remaining)| {
            derive_letter(known, remaining, '0', |_, remaining| {
                remaining
                    .iter()
                    .filter(|x| x.len() == 6)
                    .next()
                    .unwrap()
                    .clone()
            })
        })
        .pipe(|(known, remaining)| {
            derive_letter(known, remaining, '3', |known, remaining| {
                remaining
                    .iter()
                    .find(|x| known.get(&'1').unwrap().is_subset(x))
                    .unwrap()
                    .clone()
            })
        })
        .pipe(|(known, remaining)| {
            derive_letter(known, remaining, '5', |known, remaining| {
                remaining
                    .iter()
                    .filter(|x| x.len() == 5)
                    .cloned()
                    .collect::<Vec<_>>()
                    .pipe(|x| (x[0].clone(), x[1].clone()))
                    .pipe(|(a, b)| {
                        if (&b - &a).is_subset(known.get(&'4').unwrap()) {
                            b.clone()
                        } else {
                            a.clone()
                        }
                    })
            })
        })
        .pipe(|(known, remaining)| {
            derive_letter(known, remaining, '2', |_, remaining| {
                remaining.iter().next().unwrap().clone()
            })
        })
        .0
}

#[aoc_generator(day8)]
pub fn generator(input: &str) -> Vec<(HashMap<char, HashSet<char>>, Vec<HashSet<char>>)> {
    fn inner(mut input: Split<&str>) -> (Vec<HashSet<char>>, Vec<HashSet<char>>) {
        (
            input
                .next()
                .unwrap()
                .split(' ')
                .map(|s| s.chars().collect())
                .collect(),
            input
                .next()
                .unwrap()
                .split(' ')
                .map(|s| s.chars().collect())
                .collect(),
        )
    }
    input
        .lines()
        .map(|x| x.split(" | "))
        .map(inner)
        .map(|(a, b)| (derive_mapping(hashmap![], a), b))
        .collect()
}

#[aoc(day8, part1)]
pub fn solve_part1(input: &[(HashMap<char, HashSet<char>>, Vec<HashSet<char>>)]) -> usize {
    input
        .into_iter()
        .map(|(_, lights)| lights)
        .map(|x| {
            x.iter()
                .map(HashSet::len)
                .filter(|x| hashset![2, 3, 4, 7].contains(x))
                .count()
        })
        .sum()
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &[(HashMap<char, HashSet<char>>, Vec<HashSet<char>>)]) -> i64 {
    input
        .into_iter()
        .map(|(a, b)| {
            b.iter()
                .map(|x| a.iter().find(|(_, v)| v == &x).unwrap().0)
                .copied()
                .collect::<String>()
        })
        .map(|x| x.parse::<i64>().unwrap())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_part1() {
        let provided =
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
        assert_eq!(26, solve_part1(&generator(provided)));
        assert_eq!(
            245,
            solve_part1(&generator(include_str!("../input/2021/day8.txt")))
        );
    }

    #[test]
    fn it_works_part2() {
        let provided =
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
        assert_eq!(61229, solve_part2(&generator(provided)));
        assert_eq!(
            983026,
            solve_part2(&generator(include_str!("../input/2021/day8.txt")))
        );
    }

    #[test]
    fn it_solves_mapping() {
        let provided =
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
        let correct = "acedgfb: 8
cdfbe: 5
gcdfa: 2
fbcad: 3
dab: 7
cefabd: 9
cdfgeb: 6
eafb: 4
cagedb: 0
ab: 1"
            .lines()
            .map(|x| x.split(": ").collect::<Vec<_>>())
            .map(|x| (x[0], x[1]))
            .map(|(a, b)| (a.chars().collect::<HashSet<_>>(), b.chars().nth(0).unwrap()))
            .map(|(a, b)| (b, a))
            .collect::<HashMap<_, _>>();
    }
}
