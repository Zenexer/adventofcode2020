use reqwest::{Url, header};
use reqwest::header::{HeaderMap, HeaderValue};
use std::fmt::Display;
use std::fs;
use std::str::FromStr;

#[macro_export]
macro_rules! solve_day {
	{
		tests {
			input = $test_input:literal;
			$($test_part:ident = $test_output:literal);* $(;)?
		}

		$name:ident<$input:ty, $output:ty>($day:literal) {
			$($fn:tt)*
		}
	} => {
		struct $name();

		fn main() {
			$crate::Day::run(&$name());
		}
		
		impl $crate::Day<$input, $output> for $name {
			fn day(&self) -> u8 {
				$day
			}
			
			$($fn)*
		}

		#[cfg(test)]
		mod test {
			use super::*;

			$(
				#[test]
				fn $test_part() {
					let x = $name();

					let raw_indented: &str = $test_input;
					let raw: &str = &raw_indented
						.trim()
						.lines()
						.map(str::trim)
						.collect::<Vec<&str>>()
						.join("\n");

					let input: &[$input] = &$crate::Day::parse(&x, raw);
					$crate::Day::check_input(&x, input);
					let solution = $crate::Day::$test_part(&x, input);
					let output = format!("{}", solution);
					assert_eq!(output, $test_output);
				}
			)*
		}
	}
}

pub enum Solution<T: Display> {
	NotImplemented,
	Failed,
	Ok(T),
}

impl<T: Display> Display for Solution<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Solution::NotImplemented => write!(f, "(not implemented)"),
			Solution::Failed => write!(f, "(FAILED!)"),
			Solution::Ok(x) => x.fmt(f),
		}
    }
}

pub trait Day<Input: FromStr, Output: Display> {
	fn day(&self) -> u8;
	fn part1(&self, input: &[Input]) -> Solution<Output>;
	fn part2(&self, _input: &[Input]) -> Solution<Output> { Solution::NotImplemented }

	fn check_input(&self, _input: &[Input]) { }

	fn assert_max(&self, input: &[Input], max: Input)
	where
		Input: Ord + Display
	{
		assert!(input.iter().all(|x| *x <= max), format!("It doesn't make sense for any item to be greater than {}", max));
	}

	fn parse(&self, raw: &str) -> Vec<Input> {
		raw
			.lines()
			.map(|x| x.parse::<Input>().ok().expect(&format!("Failed to parse line: {}", x)))
			.collect()
	}

	fn part(&self, part: u8, input: &[Input]) -> Solution<Output> {
		match part {
			1 => self.part1(input),
			2 => self.part2(input),
			_ => panic!("Logic error"),
		}
	}

	fn run(&self) {
		let day = self.day();
		let raw = get_raw_input(day);
		let input = &self.parse(&raw);
		self.check_input(input);

		for part in 1..3 {
			let solution = self.part(part, input);
			println!("Day {}, Part {}: {}", day, part, solution);
		}
	}
}

fn get_session_id() -> String {
	fs::read_to_string("session.txt")
		.expect("Create a file named session.txt in the current directory containing the value of AoC's session cookie.")
		.trim()
		.to_owned()
}

fn get_raw_input(day: u8) -> String {
	let session_id = get_session_id();
	let cookie_value = HeaderValue::from_str(&format!("session={}", session_id))
		.expect("Session ID in session.txt can't be converted to a header value");

	let url = Url::parse(format!("https://adventofcode.com/2020/day/{}/input", day).as_str())
		.expect("Failed to parse URL");

	let mut headers = HeaderMap::new();
	headers.insert(header::COOKIE, cookie_value);

	let client = reqwest::blocking::Client::builder()
		.default_headers(headers)
		.build()
		.unwrap();

	let response = client
		.get(url)
		.send()
		.expect("Failed to retrieve input.  Is your session.txt file up-to-date?");

	response.text().unwrap()
}