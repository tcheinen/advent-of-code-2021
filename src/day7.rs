use tap::prelude::*;

/// https://adventofcode.com/2021/day/6

#[aoc_generator(day7)]
pub fn generator(input: &str) -> (i64, i64, Vec<i64>) {
    fn inner(input: Vec<i64>) -> (i64, i64, Vec<i64>) {
        (
            *input.iter().min().unwrap(),
            *input.iter().max().unwrap(),
            input,
        )
    }
    input
        .split(",")
        .map(|x| str::parse(x).unwrap())
        .collect::<Vec<_>>()
        .pipe(inner)
}

#[aoc(day7, part1)]
pub fn solve_part1((min, max, input): &(i64, i64, Vec<i64>)) -> i64 {
    (*min..=*max)
        .map(|target| input.iter().map(|x| (x - target).abs()).sum())
        .min()
        .unwrap()
}

#[aoc(day7, part2)]
pub fn solve_part2((min, max, input): &(i64, i64, Vec<i64>)) -> i64 {
    (*min..=*max)
        .map(|target| {
            input
                .iter()
                .map(|x| ((x - target).abs() * ((x - target).abs() + 1)) / 2)
                .sum()
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_part1() {
        let provided = "16,1,2,0,4,2,7,1,2,14";
        assert_eq!(37, solve_part1(&generator(provided)));
        assert_eq!(
            357353,
            solve_part1(&generator(include_str!("../input/2021/day7.txt")))
        );
    }

    #[test]
    fn it_works_part2() {
        let provided = "16,1,2,0,4,2,7,1,2,14";

        assert_eq!(168, solve_part2(&generator(provided)));
        assert_eq!(
            104822130,
            solve_part2(&generator(include_str!("../input/2021/day7.txt")))
        );
    }
}
