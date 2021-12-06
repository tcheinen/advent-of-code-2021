use core::str::Split;
use tap::Pipe;

fn transpose<T>(v: &[Vec<T>]) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

fn win(board: Vec<Vec<i32>>) -> bool {
    fn check_rows(board: &[Vec<i32>]) -> bool {
        board.iter().any(|row| row.iter().all(|y| *y < 0))
    }
    fn check_cols(board: &[Vec<i32>]) -> bool {
        board
            .pipe(transpose)
            .iter()
            .any(|col| col.iter().all(|y| *y < 0))
    }
    check_rows(&board) || check_cols(&board)
}

/// https://adventofcode.com/2021/day/4
#[aoc_generator(day4)]
pub fn generator(input: &str) -> (Vec<i32>, Vec<Vec<Vec<i32>>>) {
    fn board(input: &str) -> Vec<Vec<i32>> {
        input
            .split("\n")
            .map(|line| line.split_whitespace().flat_map(str::parse).collect())
            .collect()
    }
    fn inner(mut input: Split<&str>) -> (Vec<i32>, Vec<Vec<Vec<i32>>>) {
        (
            input
                .next()
                .unwrap()
                .split(",")
                .flat_map(str::parse)
                .collect(),
            input
                .map(String::from)
                .collect::<Vec<String>>()
                .join("\n")
                .trim_left()
                .split("\n\n")
                .map(|x| board(x))
                .collect(),
        )
    }
    inner(input.split("\n"))
}

fn score(board: Vec<Vec<i32>>) -> i32 {
    board
        .iter()
        .map(|row| row.iter().filter(|x| **x > 0).sum::<i32>())
        .sum()
}

fn recurse(
    prev: i32,
    rng: &[i32],
    boards: Vec<Vec<Vec<i32>>>,
    filter: impl Fn(&[Vec<Vec<i32>>]) -> Vec<Vec<Vec<i32>>>,
) -> (i32, Vec<Vec<i32>>) {
    if let Some(board) = boards.iter().find(|x| win(x.to_vec())).cloned() {
        (prev, board.clone())
    } else {
        boards
            .iter()
            .map(|board| {
                board
                    .iter()
                    .map(|row| {
                        row.iter()
                            .map(|element| if *element == rng[0] { -1 } else { *element })
                            .collect()
                    })
                    .collect()
            })
            .collect::<Vec<_>>()
            .pipe(|boards| recurse(rng[0], &rng[1..], filter(&boards), filter))
    }
}

#[aoc(day4, part1)]
pub fn solve_part1((rng, boards): &(Vec<i32>, Vec<Vec<Vec<i32>>>)) -> i32 {
    recurse(-1, rng, boards.clone(), |x| x.to_vec()).pipe(|(prev, board)| prev * score(board))
}

#[aoc(day4, part2)]
pub fn solve_part2((rng, boards): &(Vec<i32>, Vec<Vec<Vec<i32>>>)) -> i32 {
    recurse(-1, rng, boards.clone(), |x| {
        if x.len() == 1 {
            x.to_vec()
        } else {
            x.into_iter()
                .filter(|x| !win(x.to_vec()))
                .cloned()
                .collect()
        }
    })
    .pipe(|(prev, board)| prev * score(board))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_part1() {
        let provided = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";
        assert_eq!(4512, solve_part1(&generator(provided)));
        assert_eq!(
            12796,
            solve_part1(&generator(include_str!("../input/2021/day4.txt")))
        );
    }

    #[test]
    fn it_works_part2() {
        let provided = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";
        assert_eq!(1924, solve_part2(&generator(provided)));
        assert_eq!(
            18063,
            solve_part2(&generator(include_str!("../input/2021/day4.txt")))
        );
    }

    #[test]
    fn it_generates() {
        let provided = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";
        let (rng, boards) = generator(provided);
        assert_eq!(
            vec![
                7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8,
                19, 3, 26, 1
            ],
            rng
        );
        assert_eq!(
            boards[0],
            vec![
                vec![22, 13, 17, 11, 0],
                vec![8, 2, 23, 4, 24],
                vec![21, 9, 14, 16, 7],
                vec![6, 10, 3, 18, 5],
                vec![1, 12, 20, 15, 19],
            ]
        );
    }

    #[test]
    fn checks_win() {
        assert!(!vec![
            vec![1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1],
        ]
        .pipe(win));

        assert!(!vec![
            vec![-1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1],
        ]
        .pipe(win));

        assert!(vec![
            vec![-1, 1, 1, 1, 1],
            vec![-1, 1, 1, 1, 1],
            vec![-1, 1, 1, 1, 1],
            vec![-1, 1, 1, 1, 1],
            vec![-1, 1, 1, 1, 1],
        ]
        .pipe(win));

        assert!(vec![
            vec![-1, -1, -1, -1, -1],
            vec![1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1],
        ]
        .pipe(win));
    }
}
