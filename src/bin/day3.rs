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
		part2 = "336";
	}

	Day3<Row, usize>(3) {
		fn part1(&self, input: &[Row]) -> Solution<usize> {
			Solution::Ok(self.count_trees(input, 1, 3))
		}

		fn part2(&self, input: &[Row]) -> Solution<usize> {
			Solution::Ok(
				vec![
					(1, 1),
					(3, 1),
					(5, 1),
					(7, 1),
					(1, 2),
				]
					.into_iter()
					.map(|x| self.count_trees(input, x.1, x.0))
					.product()
			)
		}
	}

	impl {
		fn count_trees(&self, input: &[Row], numerator: usize, denominator: usize) -> usize {
			input
				.iter()
				.step_by(numerator)
				.enumerate()
				.map(|x| x.1.tree_val(x.0 * denominator))
				.sum()
		}
	}
}
