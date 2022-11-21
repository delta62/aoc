#![feature(iterator_try_collect)]

#[macro_use]
extern crate aoc_runner_derive;
extern crate aoc_runner;

mod parse_helpers;

#[macro_use]
mod test_helpers;

mod d01;
mod d02;
mod d03;
mod d04;
mod d05;
mod d06;
mod d07;
mod d08;
mod d09;
mod d10;

aoc_lib! { year = 2021 }
