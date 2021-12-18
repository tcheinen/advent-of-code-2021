use dashmap::DashSet;
use itertools::Itertools;
use std::fmt::Formatter;
use tap::{Pipe, Tap};

/// https://adventofcode.com/2021/day/11

#[derive(Debug, Clone)]
pub struct Board {
    board: Vec<i32>,
    width: usize,
    height: usize,
    flashes: usize,
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Ok(self.board.chunks(self.width).for_each(|x| {
            f.write_str(&x.into_iter().map(|x| x.to_string()).collect::<String>())
                .tap(|_| f.write_str("\n").unwrap())
                .unwrap()
        }))
    }
}

impl Board {
    fn tick(mut self) -> Board {
        self.board
            .iter_mut()
            .for_each(|x| *x += 1)
            .pipe(|_| self.flash_helper(&DashSet::new()))

    }

    fn flash_helper(mut self, flashed: &DashSet<(usize, usize)>) -> Board {
        if (0..self.width)
            .cartesian_product(0..=self.height)
            .any(|(x, y)| self.flash(&flashed, x, y))
        {
            self.flash_helper(&flashed)
        } else {
            flashed
                .iter()
                .map(|x| {
                    (x.key().0, x.key().1)
                })
                .for_each(|(x, y)| (self.board[y * self.width + x] = 0).pipe(|_| self.flashes += 1))
                .pipe(|_| self)
        }
    }

    fn flash(&mut self, flashed: &DashSet<(usize, usize)>, x: usize, y: usize) -> bool {
        if self.board[y * self.width + x] > 9 && !flashed.contains(&(x, y)) {
            (flashed.insert((x, y)))
                .pipe(|_| {
                    self.get_neighbors(x, y).into_iter().for_each(|(a, b)| {
                        self.board[b * self.width + a] += 1
                    })
                })
                .pipe(|_| true)
        } else {
            false
        }
    }

    fn get_neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        [
            (0, 1),
            (0, -1),
            (1, 0),
            (-1, 0),
            (1, 1),
            (-1, -1),
            (-1, 1),
            (1, -1),
        ]
        .into_iter()
        .map(|(xm, ym)| ((x as isize) + xm, (y as isize) + ym))
        .filter(|(a, _)| *a >= 0 && *a < self.width as isize)
        .filter(|(_, b)| *b >= 0 && *b <= self.height as isize)
        .map(|(a, b)| (a as usize, b as usize))
        .collect()
    }
}

#[aoc_generator(day11)]
pub fn generator(input: &str) -> Board {
    Board {
        board: input
            .chars()
            .filter(|x| *x != '\n')
            .map(|x| x as i32 - 0x30)
            .collect(),
        width: input.find('\n').unwrap(),
        height: input.chars().filter(|x| *x == '\n').count(),
        flashes: 0,
    }
}

#[aoc(day11, part1)]
pub fn solve_part1(input: &Board) -> usize {
    (0..100).fold(input.clone(), |sum, _| sum.tick()).flashes
}

#[aoc(day11, part2)]
pub fn solve_part2(input: &Board) -> i64 {
    fn inner(depth: i64, input: Board) -> i64 {
        if input.board.iter().all(|x| *x == 0) {
            depth
        } else {
            inner(depth + 1, input.tick())
        }
    }
    inner(0, input.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_part1() {
        let provided = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

        assert_eq!(1656, solve_part1(&generator(provided)));
        assert_eq!(
            1725,
            solve_part1(&generator(include_str!("../input/2021/day11.txt").trim_end()))
        );
    }

    #[test]
    fn it_works_part2() {
        let provided = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";


        assert_eq!(195, solve_part2(&generator(provided)));
        assert_eq!(
            308,
            solve_part2(&generator(include_str!("../input/2021/day11.txt").trim_end()))
        );
    }

}
