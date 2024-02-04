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

use crate::error::Error;
use crate::node::Node;
use crate::source_location::SourceLocation;
use crate::token::Token;

macro_rules! complete_node {
	($nodes: expr, $new_node: expr, $flag: ident) => {{
		$nodes.push($new_node);
		$flag = true;
		continue;
	}};
}

impl Node {
	#[must_use]
	pub fn parse(tokens: &[(SourceLocation, Token)]) -> Result<Vec<Self>, Error> {
		assert!(tokens.len() > 0x0);

		let mut file: Option<&str> = None;

		let mut nodes: Vec<Self> = Vec::new();

		let mut got_end       = false;
		let mut node_complete = true;

		for (location, token) in tokens {
			use Token::*;

			file = Some(location.file());

			match token {
				Word(word) => match word.as_str() {
					| "CODE16"
					| "code16"
					| "THUMB"
					| "thumb"
					=> complete_node!(nodes, Node::Code16, node_complete),

					| "ARM"
					| "arm"
					| "CODE32"
					| "code32"
					=> complete_node!(nodes, Node::Code32, node_complete),

					| "END"
					| "end"
					=> {
						got_end = true;
						complete_node!(nodes, Node::End, node_complete);
					},

					_ => return Err(Error::UnknownMnemonic(word.clone(), location.clone())),
				},

				Return => {
					if !node_complete { return Err(Error::IncompleteNode(location.clone())) };
					continue;
				},

				_ => {},
			};
		}

		if !got_end { return Err(Error::EndOfFile(file.unwrap().to_string())) };

		return Ok(nodes);
	}
}
