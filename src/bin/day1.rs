use aoc_2020::{solve_day, Solution};

const MAX: usize = 2020;

solve_day! {
	Day1<usize, usize>(1) {
		fn check_input(&self, input: &[usize]) {
			self.assert_max(input, MAX);
		}

		fn part1(&self, input: &[usize]) -> Solution<usize> {
			let mut keyed = [false; MAX];

			for &n in input {
				let complement = MAX - n;

				if keyed[complement] {
					return Solution::Ok(complement * n);
				}

				keyed[n] = true;
			}

			Solution::Failed
		}

		fn part2(&self, input: &[usize]) -> Solution<usize> {
			let mut keyed = [false; MAX];

			for &n in input {
				keyed[n] = true;
			}

			for &x in input {
				let max = MAX - x;

				for &y in input {
					if y < max {
						let z = MAX - x - y;

						if keyed[z] {
							return Solution::Ok(x * y * z);
						}
					}
				}
			}

			Solution::Failed
		}
	}
}