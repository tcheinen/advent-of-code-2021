use itertools::Itertools;

/// https://adventofcode.com/2021/day/1

#[aoc_generator(day1)]
pub fn generator(input: &str) -> Vec<u32> {
    input.split("\n").flat_map(|x| x.parse()).collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[u32]) -> usize {
    input.into_iter().tuple_windows().map(|(a,b)| a < b).filter(|x| *x).count()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[u32]) -> usize {
    input.into_iter().tuple_windows().map(|(a,b,c)| a + b + c).tuple_windows().map(|(a,b)| a < b).filter(|x| *x).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_part1() {
        let provided = "199
200
208
210
200
207
240
269
260
263";
        assert_eq!(7, solve_part1(&generator(provided)));
    }

    #[test]
    fn it_works_part2() {
        let provided = "199
200
208
210
200
207
240
269
260
263";
        assert_eq!(5, solve_part2(&generator(provided)));
    }
}
