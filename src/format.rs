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

use enum_iterator::Sequence;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Clone, Copy, Eq, PartialEq, Sequence)]
pub enum Format {
	Elf,
}

impl Display for Format {
	fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
		use Format::*;
		return match *self {
			Elf => write!(f, "elf"),
		};
	}
}

impl Default for Format {
	fn default() -> Self {
		use Format::*;
		return Elf;
	}
}

impl FromStr for Format {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		use Format::*;

		let s = s.to_string().to_lowercase();
		return match s.as_str() {
			"elf" => Ok(Elf),

			_ => Err(Error::InvalidTargetFormat(s)),
		};
	}
}
