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

mod tokenise;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Token {
	BracketLeft,
	BracketRight,
	Comma,
	FullStop,
	Hashtag,
	Return,
	StringLiteral(String),
	Word(         String),
}
