mod day1;

use shmalloc::Shmeap;

#[global_allocator]
static ALLOCATOR: Shmeap<
    0,
    { 1 << 24 },
    { libc::PROT_READ | libc::PROT_WRITE },
    { libc::MAP_ANONYMOUS | libc::MAP_SHARED },
    "",
> = Shmeap::new();

#[macro_use]
extern crate aoc_runner_derive;
use aoc_runner_derive::aoc_lib;


aoc_lib! { year = 2021 }
