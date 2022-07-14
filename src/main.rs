use std::io::{self, Write};

fn main() -> io::Result<()> {
	let stdin = io::stdin();
	let mut stdout = io::stdout();
	let mut input = String::new();

	loop {
		write!(stdout, "> ")?;
		stdout.flush()?;

		stdin.read_line(&mut input)?;

		if input.is_empty() {
			break;
		}

		let tokens = didactic_eureka::lexer::lex(&input);
		dbg!(&tokens);

		let source_file = didactic_eureka::parser::parse(&tokens);
		dbg!(&source_file);
		let basic_block = didactic_eureka::tycheck::lower(&source_file);
		dbg!(&basic_block);

		input.clear();
	}

	Ok(())
}
