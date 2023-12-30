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

extern crate enum_iterator;

mod app;
mod cpu;
mod format;
mod token;

mod is_valid_character;
mod log;

pub use is_valid_character::*;

pub const VERSION: (u32, u32, u32) = (
	0x0, // Major
	0x0, // Minor
	0x0, // Patch
);

fn main() { app::App::main() }
