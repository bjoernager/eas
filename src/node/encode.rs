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

use crate::encode_state::EncodeState;
use crate::node::Node;

impl Node {
	#[must_use]
	pub fn encode(&self, _state: &EncodeState) -> Vec<u8> {
		use Node::*;

		let mut binary = Vec::new();

		match *self {
			// These don't leak into the binary:
			| Code16
			| Code32
			| End
			=> {},

			Fill(repeat, value, size) => {
				let mut bytes = Vec::from(value.to_le_bytes());
				bytes.resize(size as usize, 0x00);

				for _ in 0x0..repeat { binary.extend_from_slice(&bytes) }
			}

			Space(length) => for _ in 0x0..length { binary.push(0x00) },

			// In case we missed something.
			#[allow(unreachable_patterns)]
			_ => todo!(),
		};

		return binary;
	}
}
