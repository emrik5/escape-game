use std::io::{Stdout, self, Write};

use terminal_size::{Width, Height};
use crossterm::{queue, terminal};

pub fn render(term: &mut Stdout) -> io::Result<()> {
	let (width, height) = if let Some((Width(width), Height(height))) = terminal_size::terminal_size() {
		(width, height)
	} else {
		panic!()
	};
	queue!(term, terminal::Clear(terminal::ClearType::All));
	for _ in 0..width {
		print!("=");
	}
	term.flush()?;

	Ok(())
}