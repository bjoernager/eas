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

use crate::app::App;
use crate::cpu::Cpu;
use crate::format::Format;

use enum_iterator::all;
use std::process::exit;

impl App {
	pub fn print_help() -> ! {
		println!("\u{1B}[1mUsage\u{1B}[0m: \u{1B}[1maas\u{1B}[0m \u{1B}[3m[options]\u{1B}[0m <input>");
		println!();
		println!("\u{1B}[1mOptions\u{1B}[0m:");
		println!("    \u{1B}[1m-f\u{1B}[0m\u{1B}[3m<format>\u{1B}[0m set the target executable format (see below)");
		println!("    \u{1B}[1m-h\u{1B}[0m         print help");
		println!("    \u{1B}[1m-m\u{1B}[0m\u{1B}[3m<target>\u{1B}[0m set the target cpu (see below)");
		println!("    \u{1B}[1m-v\u{1B}[0m         print version");

		println!();
		println!("\u{1B}[1mCPUs\u{1B}[0m:");
		for cpu in all::<Cpu>() { println!("    \u{1B}[3m{cpu}\u{1B}[0m") }

		println!();
		println!("\u{1B}[1mFormats\u{1B}[0m:");
		for format in all::<Format>() { println!("    \u{1B}[3m{format}\u{1B}[0m") }

		exit(0x0);
	}
}
