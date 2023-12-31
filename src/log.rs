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

#[macro_export]
macro_rules! log {
	(error, $($message: tt)*) => {{
		eprintln!("\u{1B}[1m\u{1B}[91merror\u{1B}[0m: {}", format!($($message)?));
	}};

	(note, $($message: tt)*) => {{q
		eprintln!("\u{1B}[1m\u{1B}[95mnote\u{1B}[0m: {}", format!($($message)?));
	}};

	(status, $($message: tt)*) => {{
		eprintln!("{}", format!($($message)?));
	}};

	(warning, $($message: tt)*) => {{
		use crate::log::log;
		eprintln!("\u{1B}[1m\u{1B}[93mwarning\u{1B}[0m: {}", format!($($message)?));
	}};
}
