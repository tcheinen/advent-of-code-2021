use tap::prelude::*;

/// https://adventofcode.com/2021/day/6

#[aoc_generator(day6)]
pub fn generator(input: &str) -> FishCollection {
    fn inner(input: &str, fish: &mut Vec<i64>) -> Vec<i64> {
        input
            .split(",")
            .flat_map(str::parse)
            .for_each(|f: usize| fish[f] += 1)
            .pipe(|()| fish.to_vec())
    }
    FishCollection {
        fish: inner(input, &mut vec![0; 9]),
    }
}

#[derive(Clone)]
pub struct FishCollection {
    fish: Vec<i64>,
}

impl FishCollection {
    fn tick(&self) -> FishCollection {
        FishCollection {
            fish: self.fish[1..]
                .into_iter()
                .copied()
                .chain(Some(self.fish[0]))
                .collect::<Vec<i64>>()
                .tap_mut(|x| x[7] += x[0]),
        }
    }
    fn sum(&self) -> i64 {
        self.fish.iter().copied().sum()
    }
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &FishCollection) -> i64 {
    (0..79).fold(input.clone(), |sum, _| sum.tick()).sum()
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &FishCollection) -> i64 {
    (0..255).fold(input.clone(), |sum, _| sum.tick()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_part1() {
        let provided = "3,4,3,1,2";
        assert_eq!(5934, solve_part1(&generator(provided)));
        assert_eq!(
            374994,
            solve_part1(&generator(include_str!("../input/2021/day6.txt")))
        );
    }

    #[test]
    fn it_works_part2() {
        let provided = "3,4,3,1,2";

        assert_eq!(26984457539, solve_part2(&generator(provided)));
        assert_eq!(
            1686252324092,
            solve_part2(&generator(include_str!("../input/2021/day6.txt")))
        );
    }
}
