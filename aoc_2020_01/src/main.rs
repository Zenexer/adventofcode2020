use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
	let p1 = part1().unwrap();
	let p2 = part2().unwrap();

	println!("Part 1: {} ({} * {})", p1.0, p1.1, p1.2);
	println!("Part 2: {}", part2().unwrap());
}

fn part1() -> std::io::Result<(usize, usize, usize)> {
	let file = File::open("input.txt")?;
	let rx = BufReader::new(file);

	let numbers = rx.lines().map(|x| x.unwrap().parse::<usize>().expect("Failed to parse as usize"));

	// We know none of the numbers are greater than 2048 by glancing at the input, so this is efficient 
	let mut keyed: [bool; 2048] = [false; 2048];

	for n in numbers {
		let complement = 2020 - n;

		if keyed[complement] {
			return Ok((n * complement, n, complement));
		}

		keyed[n] = true;
	}

	panic!("Couldn't find matching numbers");
}

fn part2() -> std::io::Result<usize> {
	panic!("TODO");
}
