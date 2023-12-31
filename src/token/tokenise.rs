/*
	Copyright 2023 Gabriel Bjørnager Jensen.

	This file is part of AAS.

	AAS is free software: you can redistribute it
	and/or modify it under the terms of the GNU
	General Public License as published by the Free
	Software Foundation, either version 3 of the
	License, or (at your option) any later version.

	AAS is distributed in the hope that it will
	be useful, but WITHOUT ANY WARRANTY; without
	even the implied warranty of MERCHANTABILITY or
	FITNESS FOR A PARTICULAR PURPOSE. See the GNU
	General Public License for more details.

	You should have received a copy of the GNU
	General Public License along with AAS. If not,
	see <https://www.gnu.org/licenses/>.
*/

use crate::is_valid_character;
use crate::token::Token;

impl Token {
	#[must_use]
	pub fn tokenise(input: &str) -> Result<Vec<Self>, String> {
		let mut tokens: Vec<Self> = Vec::new();

		let mut input_index: usize = 0x0;
		while let Some(token) = get_next_token(&input, &mut input_index)? { tokens.push(token) }

		return Ok(tokens);
	}
}

#[must_use]
fn get_next_token(input: &str, index: &mut usize) -> Result<Option<Token>, String> {
	use Token::*;

	let mut word = String::new();

	let mut in_comment = false;
	let mut in_string  = false;

	for c in input.chars().skip(*index) {
		// Skip until we're out of the comment.
		if in_comment {
			if c != '\n' {
				*index += 0x1;
				continue;
			}

			in_comment = false;
		}

		// Finish the string (if inside one) and return.
		if in_string {
			*index += 0x1;

			if c != '"' {
				word.push(c);
				continue;
			}

			return Ok(Some(StringLiteral(word)));
		}

		// We don't care about invalid character inside of
		// comments or strings.
		if !is_valid_character(c) { return Err(format!("invalid character U+{:04X} '{c}' ({index} / {})", c as u32, input.len())) };

		// Check if the word is terminated. If it was, we
		// don't count this character.
		if !word.is_empty() {
			match c {
				| ' '
				| '\t'
				| '\n'
				| '.'
				| ','
				| ':'
				| ';'
				| '@'
				=> return Ok(Some(Word(word))),

				_ => {},
			};
		}

		// There aren't any more things to complete
		// (comments, strings, or words), so we know now
		// that no more characters will be skipped.
		*index += 0x1;

		match c {
			| ' '
			| '\t'
			=> continue,

			'\n' => return Ok(Some(Return)),
			'['  => return Ok(Some(BracketLeft)),
			']'  => return Ok(Some(BracketRight)),
			'.'  => return Ok(Some(FullStop)),
			','  => return Ok(Some(Comma)),
			':'  => return Ok(Some(Colon)),
			'#'  => return Ok(Some(Hashtag)),

			| ';'
			| '@'
			=> {
				in_comment = true;
				continue;
			},

			'"' => {
				in_string = true;
				continue;
			}

			_ => {},
		};

		word.push(c);
	}

	if in_string { return Err("unterminated string".to_string()) };

	return Ok(None);
}
