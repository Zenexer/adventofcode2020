use std::str::FromStr;

use aoc_2020::{InvalidInputError, Solution, solve_day};

struct Row {
	size: usize,
	trees: u64,
}

impl Row {
	fn tree_val(&self, n: usize) -> usize {
		(self.trees >> (n % self.size)) as usize & 1
	}

	// Might need this later; not sure yet.  Delete if not needed.
	/*
	fn is_tree(&self, n: usize) -> bool {
		self.tree_val(n) == 1
	}
	*/
}

impl FromStr for Row {
	type Err = InvalidInputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
		if s.len() > 64 {
			return Err(InvalidInputError());
		}
		
		let size = s.len();
		let mut trees: u64 = 0;
		
		for c in s.chars().into_iter().rev() {
			trees <<= 1;
			trees |= (c == '#') as u64;
		}

		Ok(Row { size, trees })
    }
}

solve_day! {
	tests {
		input = "
			..##.......
			#...#...#..
			.#....#..#.
			..#.#...#.#
			.#...##..#.
			..#.##.....
			.#.#.#....#
			.#........#
			#.##...#...
			#...##....#
			.#..#...#.#
		";
		part1 = "7";
	}

	Day3<Row, usize>(3) {
		fn part1(&self, input: &[Row]) -> Solution<usize> {
			Solution::Ok(input
				.iter()
				.enumerate()
				.map(|x| x.1.tree_val(x.0 * 3))
				.sum())
		}
	}
}
