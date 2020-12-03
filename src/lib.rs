use reqwest::{Url, header};
use reqwest::header::{HeaderMap, HeaderValue};
use std::fmt::Display;
use std::fs;
use std::str::FromStr;

#[macro_export]
macro_rules! solve_day {
	{
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
	};
}

pub trait Day<Input: FromStr, Output: Display> {
	fn day(&self) -> u8;
	fn part1(&self, input: &[Input]) -> Output;
	fn part2(&self, _input: &[Input]) -> Option<Output> { None }

	fn parse(&self, raw: &str) -> Vec<Input> {
		raw
			.lines()
			.map(|x| x.parse::<Input>().ok().expect(&format!("Failed to parse line: {}", x)))
			.collect()
	}

	fn run(&self) {
		let day = self.day();
		let raw = get_raw_input(day);
		let input: &[Input] = &self.parse(&raw);

		println!("Day {}, part 1: {}", day, self.part1(input));

		if let Some(part2) = self.part2(input) {
			println!("Day {}, part 2: {}", day, part2);
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