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

use std::fmt::{Display, Formatter};
use std::num::NonZeroUsize;

#[derive(Clone, Debug)]
pub struct SourceLocation {
	file:   String,
	line:   NonZeroUsize,
	column: NonZeroUsize,
}

impl SourceLocation {
	pub fn new(file: &str) -> Self { Self {
		file:   file.to_string(),
		line:   NonZeroUsize::new(0x1).unwrap(),
		column: NonZeroUsize::new(0x1).unwrap(),
	} }

	pub fn next_column(&mut self) {
		self.column = self.column.checked_add(0x1).unwrap();
	}

	pub fn return_carriage(&mut self) {
		self.line   = self.line.checked_add(0x1).unwrap();
		self.column = NonZeroUsize::new(0x1).unwrap();
	}

	#[inline(always)]
	#[must_use]
	pub fn file<'a>(&'a self) -> &'a str { &self.file }
}

impl Display for SourceLocation {
	fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
		return write!(f, "in \"{}\", at {}:{}", &self.file, self.line, self.column);
	}
}
