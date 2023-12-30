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
use crate::token::Token;

use std::fs::read_to_string;

impl App {
	#[must_use]
	pub fn run(self) -> Result<(), String> {
		if cfg!(debug_assertions) {
			println!("\u{1B}[1mSettings\u{1B}[0m:");
			println!("\u{B7} input  \u{1B}[3m\"{}\"\u{1B}[0m", self.input);
			println!("\u{B7} output \u{1B}[3m\"{}\"\u{1B}[0m", self.output);
			println!();
			println!("\u{B7} cpu    \u{1B}[3m{}\u{1B}[0m", self.cpu);
			println!("\u{B7} format \u{1B}[3m{}\u{1B}[0m", self.format);
			println!();
		}

		let input = match read_to_string(&self.input) {
			Ok(content) => content,

			_ => return Err(format!("unable to read file \"{}\"", &self.input)),
		};

		let tokens = Token::tokenise(&input)?;

		eprintln!("\u{1B}[1mTokens\u{1B}[0m:");
		for token in &tokens {
			eprintln!("\u{B7} {token:?}");
		}

		return Ok(());
	}
}
