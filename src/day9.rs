use dashmap::DashSet;
use itertools::Itertools;
use tap::prelude::*;

/// https://adventofcode.com/2021/day/9

#[aoc_generator(day9)]
pub fn generator(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(str::chars)
        .map(|x| x.map(|y| y as i64 - 0x30).collect())
        .collect()
}

#[aoc(day9, part1)]
pub fn solve_part1(input: &[Vec<i64>]) -> i64 {
    (0..input[0].len() as isize)
        .cartesian_product(0..input.len() as isize)
        .filter(|(x, y)| {
            [(1, 0), (-1, 0), (0, 1), (0, -1)]
                .iter()
                .filter_map(|(mx, my)| {
                    Some(
                        input[*y as usize][*x as usize]
                            < *input
                                .iter()
                                .nth((y + my) as usize)
                                .and_then(|nx| nx.iter().nth((x + mx) as usize))?,
                    )
                })
                .all(|bl| bl)
        })
        .map(|(x, y)| input[y as usize][x as usize] + 1)
        .sum()
}


#[aoc(day9, part2)]
pub fn solve_part2(input: &[Vec<i64>]) -> i64 {

    fn inner(
        (x, y): (i64, i64),
        input: &[Vec<i64>],
        remaining: &DashSet<(i64, i64)>,
    ) -> Option<i64> {
        if (0..(input[0].len() as i64)).contains(&x)
            && (0..(input.len() as i64)).contains(&y)
            && input[y as usize][x as usize] != 9
            && remaining.contains(&(x, y))
        {
            remaining.remove(&(x, y)).pipe(|_| {
                Some(
                    1 + inner((x + 1, y), input, remaining).unwrap_or(0)
                        + inner((x - 1, y), input, remaining).unwrap_or(0)
                        + inner((x, y + 1), input, remaining).unwrap_or(0)
                        + inner((x, y - 1), input, remaining).unwrap_or(0),
                )
            })
        } else {
            None
        }
    }

    fn bind_dm(input: &[Vec<i64>], remaining: &DashSet<(i64, i64)>) -> i64 {
        (0..input[0].len() as i64)
            .cartesian_product(0..input.len() as i64)
            .filter_map(|(x, y)| inner((x, y), input, &remaining))
            .sorted()
            .rev()
            .take(3)
            .fold(1, |sum, next| sum * next)
    }

    bind_dm(
        input,
        &(0..input[0].len() as i64)
            .cartesian_product(0..input.len() as i64)
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_part1() {
        let provided = "2199943210
3987894921
9856789892
8767896789
9899965678";
        assert_eq!(15, solve_part1(&generator(provided)));
        assert_eq!(
            498,
            solve_part1(&generator(include_str!("../input/2021/day9.txt")))
        );
    }

    #[test]
    fn it_works_part2() {
        let provided = "2199943210
3987894921
9856789892
8767896789
9899965678";
        assert_eq!(1134, solve_part2(&generator(provided)));
        assert_eq!(1071000, solve_part2(&generator(include_str!("../input/2021/day9.txt"))));
    }
}
