/*
	Copyright 2023 Gabriel Jensen.

	This file is part of aas.

	aas is free software: you can redistribute it
	and/or modify it under the terms of the GNU
	General Public License as published by the Free
	Software Foundation, either version 3 of the
	License, or (at your option) any later version.

	aas is distributed in the hope that it will
	be useful, but WITHOUT ANY WARRANTY; without
	even the implied warranty of MERCHANTABILITY or
	FITNESS FOR A PARTICULAR PURPOSE. See the GNU
	General Public License for more details.

	You should have received a copy of the GNU
	General Public License along with aas. If not,
	see <https://www.gnu.org/licenses/>.
*/

use crate::app::App;
use crate::cpu::Cpu;
use crate::format::Format;

use std::env::args;
use std::str::FromStr;

macro_rules! use_remainder {
	($destination: ident, $argument: expr, $index: expr) => {{
		let argument: &[char] = $argument;
		let index:    usize   = $index + 0x1;

		if argument.len() <= index {
			let c = argument[index - 0x1];
			Err(format!("missing value for short '{}'", c))
		} else {
			let value: String = argument[$index + 0x1..].iter().collect();
			$destination = Some(value);

			break;
		}
	}};
}

impl App {
	pub fn init() -> Result<Self, String> {
		if args().len() < 0x2 { Self::print_help() };

		let mut input:  Option<String> = None;
		let mut output: Option<String> = None;
		let mut cpu:    Option<String> = None;
		let mut format: Option<String> = None;

		let mut handle_short = |argument: &String| -> Result<(), String> {
			let argument: Vec<char> = argument.chars().collect();

			for (index, c) in argument.iter().enumerate().skip(0x1) {
				match c {
					'f' => use_remainder!(format, &argument, index)?,
					'h' => Self::print_help(),
					'o' => use_remainder!(output, &argument, index)?,
					'm' => use_remainder!(cpu, &argument, index)?,
					'v' => Self::print_version(),

					_ => { return Err(format!("invalid short parameter '{c}'")) },
				};
			}

			return Ok(());
		};

		for argument in args().skip(0x1) {
			if argument.is_empty() { continue };

			if argument.chars().nth(0x0) != Some('-') {
				// This argument is the input path.

				if input.is_some() { return Err(format!("input pathy is already set (to \"{}\")", input.as_ref().unwrap())) };

				input = Some(argument.to_owned());
				continue;
			}

			handle_short(&argument)?;
		}

		// Check if any of the mandatory parameters have
		// been set.
		if input.is_none()  { return Err("missing input path".to_string()) };
		if output.is_none() { output = Some("a.out".to_string()) };
		if cpu.is_none()    { return Err("missing cpu".to_string()) };

		let format = match format {
			Some(format) => Format::from_str(&format)?,
			_            => Format::default(),
		};

		return Ok(Self {
			input:  input.unwrap(),
			output: output.unwrap(),

			cpu:   Cpu::from_str(&cpu.unwrap())?,
			format,
		});
	}
}
