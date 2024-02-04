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

#[macro_export]
macro_rules! log {
	(error, $error: expr) => {{
		use crate::error::Error;
		let error: Error = $error;
		eprintln!("\u{1B}[1m\u{1B}[91merror\u{1B}[0m \u{1B}[1m(e{:04X})\u{1B}[22m: {}", error.code(), error.message());

		if let Some(note) = error.note() { eprintln!("\u{2014} \u{1B}[1m\u{1B}[93mnote\u{1B}[0m: \u{1B}[3m{note}\u{1B}[m") };
	}};

	(status, $($message: tt)*) => {{
		eprintln!("{}", format!($($message)?));
	}};
}
