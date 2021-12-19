use itertools::Itertools;
use reformation::Reformation;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use tap::Pipe;

/// https://adventofcode.com/2021/day/13

#[derive(Reformation, Debug, Copy, Clone)]
pub enum Fold {
    #[reformation(r"fold along x={}")]
    Vertical(usize),
    #[reformation(r"fold along y={}")]
    Horizontal(usize),
}

#[derive(Clone)]
pub struct Paper {
    board: HashSet<(usize, usize)>,
}

impl Display for Paper {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        fn get_size(input: &HashSet<(usize, usize)>) -> (usize, usize) {
            (
                input.iter().map(|(a, _)| *a).max().unwrap(),
                input.iter().map(|(_, b)| *b).max().unwrap(),
            )
        }
        f.write_str("\n").unwrap().pipe(|_| {
            Ok(get_size(&self.board).pipe(|(a, b)| {
                (0..=b).for_each(|y| {
                    f.write_str(
                        &(0..=a)
                            .map(|x| {
                                if self.board.contains(&(x, y)) {
                                    "█"
                                } else {
                                    "░"
                                }
                            })
                            .chain(["\n"].into_iter())
                            .collect::<String>(),
                    )
                    .unwrap()
                })
            }))
        })
    }
}

impl Paper {
    fn from_str(s: &str) -> Self {
        Paper {
            board: s
                .lines()
                .flat_map(|x| x.split(",").collect_tuple())
                .filter_map(|(a, b)| Some((a.parse().ok()?, b.parse().ok()?)))
                .collect(),
        }
    }

    fn fold(&self, fold: Fold) -> Self {
        // println!("{:?}", self.board);
        Paper {
            board: self
                .board
                .iter()
                .filter(|(a, b)| match fold {
                    Fold::Horizontal(f) => *b < f,
                    Fold::Vertical(f) => *a < f,
                })
                .copied()
                .collect::<HashSet<(usize, usize)>>()
                .union(
                    &self
                        .board
                        .iter()
                        .filter(|(a, b)| match fold {
                            Fold::Horizontal(f) => *b > f,
                            Fold::Vertical(f) => *a > f,
                        })
                        .map(|(a, b)| {
                            match fold {
                                Fold::Horizontal(f) => (*a, f - (b - f)),
                                Fold::Vertical(f) => (f - (a - f), *b),
                            }
                        })
                        .collect::<HashSet<(usize, usize)>>(),
                )
                .copied()
                .collect(),
        }
    }
}

#[aoc_generator(day13)]
pub fn generator(input: &str) -> (Paper, Vec<Fold>) {
    fn bind((board, folds): (&str, &str)) -> (Paper, Vec<Fold>) {
        (
            Paper::from_str(board),
            folds.lines().flat_map(Fold::parse).collect(),
        )
    }
    bind(input.split("\n\n").collect_tuple().unwrap())
}

#[aoc(day13, part1)]
pub fn solve_part1((paper, folds): &(Paper, Vec<Fold>)) -> usize {
    paper.fold(folds[0]).board.len()
}

#[aoc(day13, part2)]
pub fn solve_part2((paper, folds): &(Paper, Vec<Fold>)) -> Paper {
    folds
        .into_iter()
        .fold(paper.clone(), |sum, next| sum.fold(*next))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_part1() {
        let provided = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

        assert_eq!(17, solve_part1(&generator(provided)));
    }

}
