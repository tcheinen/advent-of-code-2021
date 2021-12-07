use reformation::Reformation;
use std::cmp::{max, min};
use tap::prelude::*;
/// https://adventofcode.com/2021/day/2

#[derive(Reformation, Debug, PartialEq, Eq, Copy, Clone)]
#[reformation(r"{a} -> {b}")]
pub struct Line {
    pub a: Point,
    pub b: Point,
}

#[derive(Reformation, Debug, PartialEq, Eq, Copy, Clone)]
#[reformation(r"{x},{y}")]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

#[aoc_generator(day5)]
pub fn generator(input: &str) -> Vec<Line> {
    input.lines().map(Line::parse).map(|x| x.unwrap()).collect()
}

pub fn process_lines(input: &[Line], diagonal: bool) -> Vec<Vec<i64>> {
    fn make_matrix(x_size: i64, y_size: i64) -> Vec<Vec<i64>> {
        vec![vec![0; x_size as usize]; y_size as usize]
    }
    make_matrix(
        input
            .into_iter()
            .flat_map(|line| [line.a.x, line.b.x].into_iter())
            .max()
            .unwrap()
            + 1,
        input
            .into_iter()
            .flat_map(|line| [line.a.y, line.b.y].into_iter())
            .max()
            .unwrap()
            + 1,
    )
    .pipe(|mut matrix| {
        input
            .into_iter()
            .for_each(|next| {
                if next.a.x == next.b.x {
                    (min(next.a.y, next.b.y)..=max(next.a.y, next.b.y))
                        .map(|x| x as usize)
                        .for_each(|lol| matrix[lol][next.a.x as usize] += 1)
                } else if next.a.y == next.b.y {
                    (min(next.a.x, next.b.x)..=max(next.a.x, next.b.x))
                        .map(|x| x as usize)
                        .for_each(|lol| matrix[next.a.y as usize][lol] += 1)
                } else {
                    if diagonal {
                        if (next.a.x..=next.b.x).count() == 0 {
                            (next.b.x..=next.a.x).rev().collect::<Vec<_>>().into_iter()
                        } else {
                            (next.a.x..=next.b.x).collect::<Vec<_>>().into_iter()
                        }
                        .zip(
                            if (next.a.y..=next.b.y).count() == 0 {
                                (next.b.y..=next.a.y).rev().collect::<Vec<_>>().into_iter()
                            } else {
                                (next.a.y..=next.b.y).collect::<Vec<_>>().into_iter()
                            },
                        )
                        .for_each(|(x, y)| matrix[y as usize][x as usize] += 1)
                    }
                }
            })
            .pipe(|_| matrix)
    })
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &[Line]) -> usize {
    process_lines(input, false)
        .into_iter()
        .map(|row| row.iter().filter(|elem| **elem >= 2).count())
        .sum()
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &[Line]) -> usize {
    process_lines(input, true)
        .into_iter()
        .map(|row| row.iter().filter(|elem| **elem >= 2).count())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_part1() {
        let provided = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";
        assert_eq!(5, solve_part1(&generator(provided)));
        assert_eq!(
            7297,
            solve_part1(&generator(include_str!("../input/2021/day5.txt")))
        )
    }

    #[test]
    fn it_works_part2() {
        let provided = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";
        assert_eq!(12, solve_part2(&generator(provided)));
        assert_eq!(
            21038,
            solve_part2(&generator(include_str!("../input/2021/day5.txt")))
        )
    }

    #[test]
    fn it_generates() {
        {
            let provided = "0,0 -> 1,1";
            assert_eq!(
                vec![Line {
                    a: Point { x: 0, y: 0 },
                    b: Point { x: 1, y: 1 }
                }],
                generator(provided)
            );
        }
        {
            let provided = "0,0 -> 1,1\n2,2 -> 3,3";
            assert_eq!(
                vec![
                    Line {
                        a: Point { x: 0, y: 0 },
                        b: Point { x: 1, y: 1 }
                    },
                    Line {
                        a: Point { x: 2, y: 2 },
                        b: Point { x: 3, y: 3 }
                    }
                ],
                generator(provided)
            );
        }
    }
}
