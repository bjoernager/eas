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

use crate::source_location::SourceLocation;

// e0??? | Syntax errors
// e1??? | CLI errors
// e2??? | Internal errors

#[derive(Clone)]
pub enum Error {
	EndOfFile(             String),
	UnknownMnemonic(       String, SourceLocation),
	AccessDenied(          String),
	IllegalCharacter(      char, SourceLocation),
	IncompleteNode(        SourceLocation),
	InvalidShortParameter( char),
	MissingInputFile,
	MissingTargetProcessor,
	UnterminatedString(    SourceLocation),
	MissingShortValue(     char),
	InvalidTargetFormat(   String),
	InvalidTargetProcessor(String),
	InvalidLongParameter(  String),
	MissingLongValue(      String),
	MissingShortArgument,
	InvalidFileEncoding(   String),
	MissingLongArgument,
}

impl Error {
	#[must_use]
	pub const fn code(&self) -> u16 {
		use Error::*;

		return match *self {
			EndOfFile(             ..) => 0x0000,
			UnknownMnemonic(       ..) => 0x0001,
			IllegalCharacter(      ..) => 0x0002,
			IncompleteNode(        ..) => 0x0003,
			UnterminatedString(    ..) => 0x0004,

			InvalidShortParameter( ..) => 0x1000,
			MissingInputFile           => 0x1001,
			MissingTargetProcessor     => 0x1002,
			MissingShortValue(     ..) => 0x1003,
			InvalidTargetFormat(   ..) => 0x1004,
			InvalidTargetProcessor(..) => 0x1005,
			InvalidLongParameter(  ..) => 0x1006,
			MissingLongValue(      ..) => 0x1007,
			MissingShortArgument       => 0x1008,
			InvalidFileEncoding(   ..) => 0x1009,
			MissingLongArgument        => 0x100A,

			AccessDenied(          ..) => 0x2000,
		};
	}

	#[must_use]
	pub fn message(&self) -> String {
		use Error::*;

		return match *self {
			AccessDenied(          ref f)    => format!("access denied to \"{f}\""),
			EndOfFile(             _)        => String::from("end of file"),
			IllegalCharacter(      c, _)     => format!("illegal character U+{:04X} '{c}'", c as u32),
			IncompleteNode(        _)        => format!("incomplete node"),
			InvalidFileEncoding(   ref s)    => format!("invalid file encoding \"{s}\""),
			InvalidLongParameter(  ref s)    => format!("invalid target processor `--{s}`"),
			InvalidShortParameter( c)        => format!("invalid short parameter `-{c}`"),
			InvalidTargetFormat(   ref s)    => format!("invalid target format \"{s}\""),
			InvalidTargetProcessor(ref s)    => format!("invalid target processor \"{s}\""),
			MissingInputFile                 => String::from("missing input file"),
			MissingLongArgument              => String::from("missing long argument after `--"),
			MissingLongValue(      ref s)    => format!("missing value for long parameter `--{s}`"),
			MissingShortArgument             => format!("missing short parameter after `-`"),
			MissingShortValue(     c)        => format!("missing value for short parameter `-{c}`"),
			MissingTargetProcessor           => String::from("missing target processor"),
			UnknownMnemonic(       ref s, _) => format!("invalid mnemonic {s}"),
			UnterminatedString(    _)        => format!("unterminated string"),
		};
	}

	#[must_use]
	pub fn note(&self) -> Option<String> {
		use Error::*;

		return match *self {
			EndOfFile(         ref f)     => Some(format!("consider adding an END directive to \"{f}\"")),
			IllegalCharacter(  _,  ref l) => Some(format!("{l}")),
			IncompleteNode(    ref l)     => Some(format!("{l}")),
			UnknownMnemonic(   _,  ref l) => Some(format!("{l}")),
			UnterminatedString(ref l)     => Some(format!("found string delimiter {l}")),

			| InvalidTargetFormat(   _)
			| InvalidTargetProcessor(_)
			| MissingLongArgument
			| MissingShortArgument
			=> Some(String::from("see `eas -h` or refer to the manual for more information")),

			_ => None,
		};
	}
}
