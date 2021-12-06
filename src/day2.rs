use reformation::Reformation;
use tap::prelude::*;

#[derive(Reformation, Eq, PartialEq, Debug)]
pub enum Direction {
    #[reformation(r"forward {}", no_regex=true)]
    Forward(i64),
    #[reformation(r"down {}", no_regex=true)]
    Down(i64),
    #[reformation(r"up {}", no_regex=true)]
    Up(i64),
}

/// https://adventofcode.com/2021/day/2


#[aoc_generator(day2)]
pub fn generator(input: &str) -> Vec<Direction> {
    input.split("\n").flat_map(Direction::parse).collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[Direction]) -> i64 {
    input.into_iter().fold((0, 0), |(pos, depth), next| {
        match next {
            Direction::Forward(magnitude) => (pos + magnitude, depth),
            Direction::Up(magnitude) => (pos, depth - magnitude),
            Direction::Down(magnitude) => (pos, depth + magnitude),
        }
    }).pipe(|(a,b)| a * b)
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[Direction]) -> i64 {
    input.into_iter().fold((0, 0, 0), |(pos, depth, aim), next| {
        match next {
            Direction::Forward(magnitude) => (pos + magnitude, depth + aim * magnitude, aim),
            Direction::Up(magnitude) => (pos, depth, aim - magnitude),
            Direction::Down(magnitude) => (pos, depth, aim + magnitude),
        }
    }).pipe(|(a,b,_)| a * b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_part1() {
        let provided = "forward 5
down 5
forward 8
up 3
down 8
forward 2";
        assert_eq!(150, solve_part1(&generator(provided)));
        assert_eq!(1507611, solve_part1(&generator(include_str!("../input/2021/day2.txt"))));

    }

    #[test]
    fn it_works_part2() {
        let provided = "forward 5
down 5
forward 8
up 3
down 8
forward 2";
        assert_eq!(900, solve_part2(&generator(provided)));
        assert_eq!(1880593125, solve_part2(&generator(include_str!("../input/2021/day2.txt"))));

    }
}
