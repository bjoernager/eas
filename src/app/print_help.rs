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
use crate::encoding::Encoding;
use crate::format::Format;
use crate::processor::Processor;

use enum_iterator::all;
use std::process::exit;

impl App {
	pub fn print_help() -> ! {
		println!("\u{1B}[1mUsage\u{1B}[0m: eas {{<options>}} <input>");
		println!();
		println!("\u{1B}[1mOptions\u{1B}[0m:");
		println!("    \u{1B}[1m-c\u{1B}[0m\u{1B}[3m<encoding>\u{1B}[23m  sets the expected input file encoding (character set) to \u{1B}[3mencoding\u{1B}[23m");
		println!("    \u{1B}[1m-f\u{1B}[0m\u{1B}[3m<format>\u{1B}[23m    sets the target executable format to \u{1B}[3mformat\u{1B}[23m");
		println!("    \u{1B}[1m-m\u{1B}[0m\u{1B}[3m<processor>\u{1B}[23m sets the target processor (machine) to \u{1B}[3mmachine\u{1B}[23m");
		println!();
		println!("    \u{1B}[1m--help\u{1B}[0m        prints help");
		println!("    \u{1B}[1m--version\u{1B}[0m     prints versioning");

		println!();
		println!("\u{1B}[1mEncodings\u{1B}[0m:");
		for encoding in all::<Encoding>() { println!("    \u{1B}[3m{encoding}\u{1B}[0m") }

		println!();
		println!("\u{1B}[1mProcessors\u{1B}[0m:");
		for processor in all::<Processor>() { println!("    \u{1B}[3m{processor}\u{1B}[0m") }

		println!();
		println!("\u{1B}[1mFormats\u{1B}[0m:");
		for format in all::<Format>() { println!("    \u{1B}[3m{format}\u{1B}[0m") }

		exit(0x0);
	}
}
