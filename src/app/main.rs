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

use crate::log;
use crate::app::App;
use crate::configuration::Configuration;

use std::process::exit;

impl App {
	pub fn main() -> ! {
		let configuration = match Configuration::from_arguments() {
			Ok(app) => app,

			Err(error) => {
				log!(error, error);
				exit(0x1);
			},
		};

		let app = App::new(configuration);
		match app.run() {
			Err(error) => {
				log!(error, error);
				exit(0x1);
			},

			_ => {},
		};

		exit(0x0);
	}
}
