#![feature(array_windows)]
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;


#[macro_use]
extern crate aoc_runner_derive;
extern crate tap;
extern crate dashmap;
extern crate itertools;

use std::collections::HashMap;
use std::hash::Hash;
use aoc_runner_derive::aoc_lib;
use tap::Pipe;

aoc_lib! { year = 2021 }


fn frequency<T: Clone + Eq + Hash>(input: Vec<T>) -> HashMap<T, usize> {
    fn bind<T: Clone + Eq + Hash>(
        input: Vec<T>,
        output: &mut HashMap<T, usize>,
    ) -> HashMap<T, usize> {
        input
            .into_iter()
            .for_each(|x| {
                output
                    .entry(x)
                    .and_modify(|y| *y += 1)
                    .or_insert(1)
                    .pipe(|_| ())
            })
            .pipe(|_| std::mem::take(output))
    }
    bind(input, &mut HashMap::new())
}

fn frequence_accumulate<T: Clone + Eq + Hash>(input: Vec<(T, usize)>) -> HashMap<T, usize> {
    fn bind<T: Clone + Eq + Hash>(
        input: Vec<(T, usize)>,
        output: &mut HashMap<T, usize>,
    ) -> HashMap<T, usize> {
        input
            .into_iter()
            .for_each(|(x, p)| {
                output
                    .entry(x)
                    .and_modify(|y| *y += p)
                    .or_insert(p)
                    .pipe(|_| ())
            })
            .pipe(|_| std::mem::take(output))
    }
    bind(input, &mut HashMap::new())
}