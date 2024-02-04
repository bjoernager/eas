/*
	Copyright 2023-2024 Gabriel Bjørnager Jensen.

	This file is part of eAS.

	eAS is free software: you can redistribute it
	and/or modify it under the terms of the GNU
	General Public License as published by the Free
	Software Foundation, either version 3 of the
	License, or (at your option) any later version.

	eAS is distributed in the hope that it will
	be useful, but WITHOUT ANY WARRANTY; without
	even the implied warranty of MERCHANTABILITY or
	FITNESS FOR A PARTICULAR PURPOSE. See the GNU
	General Public License for more details.

	You should have received a copy of the GNU
	General Public License along with eAS. If not,
	see <https://www.gnu.org/licenses/>.
*/

use crate::app::App;
use crate::configuration::Configuration;
use crate::encoding::Encoding;
use crate::error::Error;
use crate::processor::Processor;
use crate::format::Format;

use std::env::args;
use std::str::FromStr;

struct ConfigurationTemporary {
	pub input:     Option<String>,
	pub output:    Option<String>,

	pub encoding:  Option<String>,
	pub processor: Option<String>,
	pub format:    Option<String>,
}

impl Configuration {
	pub fn from_arguments() -> Result<Self, Error> {
		if args().len() < 0x2 { return Err(Error::MissingInputFile) };

		let mut temporary = ConfigurationTemporary {
			input:     None,
			output:    None,

			encoding:  None,
			processor: None,
			format:    None,
		};

		for argument in args().skip(0x1) {
			// Skip the first, we assume that it's just the
			// executable invoked.

			if argument.is_empty() { continue };
			let argument: Vec<char> = argument.chars().collect();

			if argument[0x0] != '-' {
				// This argument is the input path.

				temporary.input = Some(argument.into_iter().collect());
				continue;
			}

			match argument.get(0x1) {
				Some('-') => handle_long_argument( &mut temporary, argument.into_iter().skip(0x2).collect())?,
				Some(_)   => handle_short_argument(&mut temporary, &argument[0x1..])?,

				_ => return Err(Error::MissingShortArgument),
			}
		}

		// Check if any of the mandatory parameters have
		// been set.
		if temporary.input.is_none()     { return Err(Error::MissingInputFile) };
		if temporary.processor.is_none() { return Err(Error::MissingTargetProcessor) };

		// The output path defaults to "a.out".
		// TODO: Do something better.
		if temporary.output.is_none() { temporary.output = Some(String::from("a.out")) };

		let processor = Processor::from_str(&temporary.processor.unwrap())?;

		let encoding = match temporary.encoding {
			Some(format) => Encoding::from_str(&format)?,
			_            => Encoding::default(),
		};

		let format = match temporary.format {
			Some(format) => Format::from_str(&format)?,
			_            => Format::default(),
		};

		return Ok(Self {
			input:  temporary.input.unwrap(),
			output: temporary.output.unwrap(),

			encoding,
			processor,
			format,
		});
	}
}

fn handle_short_argument(temporary: &mut ConfigurationTemporary, argument: &[char]) -> Result<(), Error> {
	macro_rules! use_remainder_as_value {
		($argument: expr, $index: expr) => {{
			let argument: &[char] = $argument;
			let index:    usize   = $index;

			let next_index = index + 0x1;

			if argument.len() <= next_index {
				// There are no more characters, and thus no values.
				let c = argument[index];
				Err(Error::MissingShortValue(c))
			} else {
				let value: String = argument[next_index..].iter().collect();
				Ok(value)
			}
		}};
	}

	for (index, c) in argument.iter().enumerate() {
		match c {
			'c' => { temporary.encoding  = Some(use_remainder_as_value!(&argument, index)?); break; },
			'f' => { temporary.format    = Some(use_remainder_as_value!(&argument, index)?); break; },
			'm' => { temporary.processor = Some(use_remainder_as_value!(&argument, index)?); break; },
			'o' => { temporary.output    = Some(use_remainder_as_value!(&argument, index)?); break; },

			'h' => App::print_help(),

			_ => return Err(Error::InvalidShortParameter(*c)),
		};
	}

	return Ok(());
}

fn handle_long_argument(_temporary: &mut ConfigurationTemporary, argument: String) -> Result<(), Error> {
	match argument.as_str() {
		"help"    => App::print_help(),
		"version" => App::print_version(),

		_ => return Err(Error::InvalidLongParameter(argument.to_string())),
	};

	return Ok(());
}
