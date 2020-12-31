use std::{str::FromStr, collections::HashMap};

use aoc_2020::{InvalidInputError, Solution, solve_day};

struct Passport {
	fields: HashMap<String, String>,
}

impl<'a> FromStr for Passport {
    type Err = InvalidInputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
		let fields: HashMap<String, String> = s
			.trim()
			.split(&[' ', '\n', '\r'][..])
			.map(|s: &str| {
				let mut parts = s.splitn(2, ':');
				(parts.next().unwrap().to_string(), parts.next().unwrap().to_string())
			})
			.collect();

		Ok(Passport { fields })
    }
}

impl Passport {
	fn is_valid(&self) -> bool {
		match self.fields.len() {
			8 => true,
			7 => !self.fields.contains_key("cid"),
			_ => false,
		}
	}
}

solve_day! {
	tests {
		input = "
			ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
			byr:1937 iyr:2017 cid:147 hgt:183cm
			
			iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
			hcl:#cfa07d byr:1929
			
			hcl:#ae17e1 iyr:2013
			eyr:2024
			ecl:brn pid:760753108 byr:1931
			hgt:179cm
			
			hcl:#cfa07d eyr:2025 pid:166559648
			iyr:2011 ecl:brn hgt:59in
		";
		part1 = "2";
	}

	Day4<Passport, usize>(4) {
		fn part1(&self, input: &[Passport]) -> Solution<usize> {
			Solution::Ok(input
				.into_iter()
				.filter(|p| p.is_valid())
				.count())
		}

		fn split<'f>(&self, raw: &'f str) -> Vec<&'f str> {
			let delim = if raw.contains('\r') { "\r\n\r\n" } else { "\n\n" };
			raw.trim().split(delim).collect()
		}
	}
}