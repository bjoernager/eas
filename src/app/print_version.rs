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

use crate::VERSION;
use crate::app::App;

use std::process::exit;

impl App {
	pub fn print_version() -> ! {
		println!("\u{1B}[1maas\u{1B}[0m {:X}.{:X}.{:X}", VERSION.0, VERSION.1, VERSION.2);
		println!("\u{1B}[3mCopyright \u{A9} 2023 Gabriel Bj\u{F8}rnager Jensen\u{1B}[0m.");

		exit(0x0);
	}
}
