use std::{num::ParseIntError, str::FromStr};

use aoc_2020::{solve_day, Solution};

struct PasswordEntry {
	min: usize,
	max: usize,
    letter: char,
    password: String,
}

impl FromStr for PasswordEntry {
    type Err = ParseIntError;

    // Format: 1-9 a: abcdefg
    fn from_str(s: &str) -> Result<Self, Self::Err> {
		let g: Vec<&str> = s.split(&['-', ' ', ':'][..]).collect();

        Ok(PasswordEntry {
			min: usize::from_str(g[0])?,
			max: usize::from_str(g[1])?,
            letter: g[2].chars().nth(0).unwrap(),
            password: g[4].to_string(),
        })
    }
}

impl PasswordEntry {
    fn is_valid1(&self) -> bool {
        let occurrences = self.password.chars().filter(|c| *c == self.letter).count();
        (self.min..(self.max + 1)).contains(&occurrences)
	}

    fn is_valid2(&self) -> bool {
		[self.min, self.max].iter().all(|&i| match self.password.chars().nth(i - 1) {
			Some(c) => c == self.letter,
			None => false,
		})
    }
}

solve_day! {
	tests {
		input = "
			1-3 a: abcde
			1-3 b: cdefg
			2-9 c: ccccccccc
		";
		part1 = "2";
		part2 = "1";
	}

    Day2<PasswordEntry, usize>(2) {
        fn part1(&self, input: &[PasswordEntry]) -> Solution<usize> {
			Solution::Ok(input.iter().filter(|x| x.is_valid1()).count())
		}
		
        fn part2(&self, input: &[PasswordEntry]) -> Solution<usize> {
			Solution::Ok(input.iter().filter(|x| x.is_valid2()).count())
        }
    }
}
