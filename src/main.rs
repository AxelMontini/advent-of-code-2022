use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use anyhow::Context;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "aoc22", about, author)]
struct Args {
    /// Day to run. If the day is not available, program exits with error code.
    #[arg(value_parser = clap::value_parser!(u8).range(1..=31), long, short)]
    day: u8,
    /// File containing input.
    #[arg()]
    path: PathBuf,
}

fn day1(file: File) -> anyhow::Result<()> {
    let reader = BufReader::new(file);

    let elves: Vec<Vec<u32>> =
        reader
            .lines()
            .enumerate()
            .try_fold(vec![vec![]], |mut list, (i, l)| {
                match l?.as_str() {
                    "" => list.push(vec![]),
                    d => list.last_mut().unwrap().push(
                        d.parse()
                            .with_context(|| format!("invalid number on line {}", i + 1))?,
                    ),
                };
                Ok(list) as anyhow::Result<_>
            })?;

    let calories = |elf: &Vec<_>| elf.iter().sum();

    let max_calories = elves.iter().map(calories).max().unwrap_or(0);
    println!(
        "Elf carrying the most calories has {} calories",
        max_calories,
    );

    let max3_calories = elves
        .iter()
        .map(calories)
        .fold([0, 0, 0], |acc, elf| match acc {
            [a, b, _c] if a < elf => [elf, a, b],
            [a, b, _c] if b < elf => [a, elf, b],
            [a, b, c] if c < elf => [a, b, elf],
            o => o,
        });
    println!(
        "Top 3 elves are carrying a total of {} calories",
        max3_calories.iter().sum::<u32>()
    );

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let a = Args::parse();

    let file = File::open(&a.path).context("opening input file")?;

    let f = match a.day {
        1 => day1,
        d => unimplemented!("Day {} is not implemented", d),
    };

    f(file)?;

    Ok(())
}
