use std::collections::HashMap;
use std::fmt::Formatter;
use std::hash::Hash;
use std::str::Split;

use crate::{frequence_accumulate, frequency};
use dashmap::DashMap;
use itertools::Itertools;
use pathfinding::prelude::dijkstra;
use tap::{Pipe, Tap};

/// https://adventofcode.com/2021/day/15

#[derive(Debug, Clone)]
pub struct Grid {
    grid: Vec<i32>,
    width: usize,
    height: usize,
}

impl Grid {
    fn neighbors(&self, &(x, y): &(usize, usize)) -> Vec<((usize, usize), i32)> {
        [(0, 1), (0, -1), (1, 0), (-1, 0)]
            .into_iter()
            .map(|(xm, ym)| ((x as isize) + xm, (y as isize) + ym))
            .filter(|(a, _)| *a >= 0 && *a < self.width as isize)
            .filter(|(_, b)| *b >= 0 && *b < self.height as isize)
            .map(|(a, b)| (a as usize, b as usize))
            .map(|(a, b)| ((a, b), self.grid[b * self.width + a]))
            .collect()
    }

    fn make_big(&self) -> Grid {
        Grid {
            grid: self
                .grid
                .chunks(self.width)
                .flat_map(|chunk| {
                    (0..5).flat_map(|x| chunk.iter().map(|y| (*y + x - 1) % 9 + 1).collect_vec())
                })
                .collect_vec()
                .pipe(|first_block| {
                    (0..5)
                        .map(|x| {
                            (&first_block)
                                .iter()
                                .map(|y| (*y + x - 1) % 9 + 1)
                                .collect_vec()
                        })
                        .collect_vec()
                })
                .into_iter()
                .flatten()
                .collect(),
            width: self.width * 5,
            height: self.height * 5,
        }
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Ok(self.grid.chunks(self.width).for_each(|x| {
            f.write_str(&x.into_iter().map(|x| x.to_string()).collect::<String>())
                .tap(|_| f.write_str("\n").unwrap())
                .unwrap()
        }))
    }
}

#[aoc_generator(day15)]
pub fn generator(input: &str) -> Grid {
    Grid {
        grid: input
            .chars()
            .filter(|x| *x != '\n')
            .map(|x| x as i32 - 0x30)
            .collect(),
        width: input.find('\n').unwrap(),
        height: input.chars().filter(|x| *x == '\n').count() + 1,
    }
}

#[aoc(day15, part1)]
pub fn solve_part1(input: &Grid) -> i32 {
    dijkstra(
        &(0, 0),
        |x| input.neighbors(x).into_iter(),
        |&(x, y)| x == input.width - 1 && y == input.height - 1,
    )
    .unwrap()
    .1
}

#[aoc(day15, part2)]
pub fn solve_part2(input: &Grid) -> i32 {
    fn bind(input: &Grid) -> i32 {
        dijkstra(
            &(0, 0),
            |x| input.neighbors(x).into_iter(),
            |&(x, y)| x == input.width - 1 && y == input.height - 1,
        )
        .unwrap()
        .1
    }
    bind(&input.make_big())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_part1() {
        let provided = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";
        assert_eq!(40, solve_part1(&generator(provided)));
        assert_eq!(
            540,
            solve_part1(&generator(
                include_str!("../input/2021/day15.txt").trim_end()
            ))
        );
    }
    #[test]
    fn it_works_part2() {
        let provided = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

        assert_eq!(315, solve_part2(&generator(provided)));
        // assert_eq!(
        //     3288891573057,
        //     solve_part2(&generator(
        //         include_str!("../input/2021/day15.txt").trim_end()
        //     ))
        // );
    }
}
