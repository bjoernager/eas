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
use crate::error::Error;
use crate::node::Node;
use crate::source_location::SourceLocation;
use crate::token::Token;

use std::fs::read_to_string;

impl App {
	#[must_use]
	pub fn run(self) -> Result<(), Error> {
		if cfg!(debug_assertions) {
			println!("\u{1B}[1mSettings\u{1B}[0m:");
			println!("\u{B7} input  \u{1B}[3m\"{}\"\u{1B}[0m", self.input);
			println!("\u{B7} output \u{1B}[3m\"{}\"\u{1B}[0m", self.output);
			println!();
			println!("\u{B7} encoding  \u{1B}[3m{}\u{1B}[0m", self.encoding);
			println!("\u{B7} processor \u{1B}[3m{}\u{1B}[0m", self.processor);
			println!("\u{B7} format    \u{1B}[3m{}\u{1B}[0m", self.format);
			println!();
		}

		let input = match read_to_string(&self.input) {
			Ok(content) => content,

			_ => return Err(Error::AccessDenied(self.input.clone())),
		};

		let tokens = Token::tokenise(&input, &mut SourceLocation::new(&self.input))?;

		if cfg!(debug_assertions) {
			eprintln!("\u{1B}[1mTokens\u{1B}[0m:");
			for (location, token) in &tokens {
				eprintln!("\u{B7} {location}: {token:?}");
			}
		}

		let nodes = Node::parse(&tokens)?;

		if cfg!(debug_assertions) {
			eprintln!("\u{1B}[1mNodes\u{1B}[0m:");
			for node in &nodes {
				eprintln!("\u{B7} {node:?}");
			}
		}

		return Ok(());
	}
}
