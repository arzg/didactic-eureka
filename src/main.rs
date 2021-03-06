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
		let source_file = didactic_eureka::parser::parse(&tokens);
		let cfg = didactic_eureka::tycheck::lower(&source_file);
		dbg!(&cfg);
		println!("{cfg}");

		input.clear();
	}

	Ok(())
}
