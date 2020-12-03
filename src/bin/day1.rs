use aoc_2020::solve_day;

solve_day! {
	Day1<usize, usize>(1) {
		fn part1(&self, input: &[usize]) -> usize {
			assert!(*input.iter().max().unwrap() <= 2020, "It doesn't make sense for any item to be greater than 2020");

			let mut keyed = [false; 2020];

			for &n in input {
				let complement = 2020 - n;

				if keyed[complement] {
					return complement * n;
				}

				keyed[n] = true;
			}

			panic!("Failed to find solution")
		}
	}
}