/*
	Copyright 2023 Gabriel Jensen.

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

use crate::log;
use crate::app::App;

use std::process::exit;

impl App {
	pub fn main() -> ! {
		let app = match App::init() {
			Ok(app) => app,

			Err(message) => {
				log!(error, "{message}");
				exit(0x1);
			},
		};

		exit(match app.run() {
			Err(message) => {
				log!(error, "{message}");
				0x1
			},

			_ => 0x0,
		});
	}
}
