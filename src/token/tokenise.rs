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

use crate::is_valid_character;
use crate::error::Error;
use crate::source_location::SourceLocation;
use crate::token::Token;

impl Token {
	#[must_use]
	pub fn tokenise(input: &str, location: &mut SourceLocation) -> Result<Vec<(SourceLocation, Self)>, Error> {
		let mut tokens: Vec<(SourceLocation, Self)> = Vec::new();

		let mut input_index: usize = 0x0;
		while let Some(token) = get_next_token(&input, &mut input_index, location)? { tokens.push(token) }

		return Ok(tokens);
	}
}

#[must_use]
fn get_next_token(input: &str, index: &mut usize, location: &mut SourceLocation) -> Result<Option<(SourceLocation, Token)>, Error> {
	use Token::*;

	for c in input.chars().skip(*index) {
		if !is_valid_character(c) { return Err(Error::IllegalCharacter(c, location.clone()) ) };

		// There aren't any more things to complete
		// (comments, strings, or words), so we know now
		// that no more characters will be skipped.

		let token_start = location.clone();

		match c {
			| ' '
			| '\t'
			| '\n'
			| '['
			| ']'
			| '.'
			| ','
			| '#'
			| ';'
			| '"'
			=> {
				*index += 0x1;
				location.next_column();
			},

			_ => {},
		};

		match c {
			| ' '
			| '\t'
			=> continue,

			'\n' => {
				location.return_carriage();
				return Ok(Some((token_start, Return)));
			},

			'['  => return Ok(Some((token_start, BracketLeft))),
			']'  => return Ok(Some((token_start, BracketRight))),
			'.'  => return Ok(Some((token_start, FullStop))),
			','  => return Ok(Some((token_start, Comma))),
			'#'  => return Ok(Some((token_start, Hashtag))),

			';' => {
				skip_line(input, index, location);
				return Ok(Some((token_start, Return)));
			},

			'"' => {
				return match complete_string(input, index, location) {
					Ok(string) => Ok(Some((token_start, StringLiteral(string)))),
					_          => Err(Error::UnterminatedString(token_start)),
				};
			}

			_ => {},
		};

		match complete_word(input, index, location) {
			Some(word) => return Ok(Some((token_start, Word(word)))),
			_          => {},
		};
	}

	return Ok(None);
}

#[must_use]
fn complete_word(input: &str, index: &mut usize, location: &mut SourceLocation) -> Option<String> {
	let mut buffer = String::new();

	for c in input.chars().skip(*index) {
		match c {
			| ' '
			| '\t'
			| '\n'
			| '.'
			| ','
			| ';'
			=> return Some(buffer),

			_ => buffer.push(c),
		}

		// Don't count the terminating character.
		*index += 0x1;
		location.next_column();
	}

	return None;
}

#[must_use]
fn complete_string(input: &str, index: &mut usize, location: &mut SourceLocation) -> Result<String, ()> {
	let mut buffer = String::new();

	for c in input.chars().skip(*index) {
		*index += 0x1;

		match c {
			'\n' => return Err(()),
			'"'  => return Ok(buffer),
			_    => {},
		};

		location.next_column();

		buffer.push(c);
	}

	return Err(());
}

fn skip_line(input: &str, index: &mut usize, location: &mut SourceLocation) {
	for c in input.chars().skip(*index) {
		// Skip until we're out of the comment.
		*index += 0x1;
		if c == '\n' { break };
	}

	location.return_carriage();
}
