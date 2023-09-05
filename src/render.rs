use std::io::{Stdout, self, Write};

use terminal_size::{Width, Height};
use crossterm::{queue, terminal, style, cursor};

const HORIZ_FRAME_CHAR: char = '=';
const VERTI_FRAME_CHAR: char = '0';


pub fn render(term: &mut Stdout) -> io::Result<()> {
	let (width, height) = if let Some((Width(width), Height(height))) = terminal_size::terminal_size() {
		(width, height)
	} else {
		panic!()
	};
	queue!(
		term, 
		terminal::Clear(terminal::ClearType::All),
		terminal::Clear(terminal::ClearType::Purge),
		cursor::MoveTo(0, 0),
		style::SetForegroundColor(style::Color::White)
	)?;
	term.flush()?;
	Ok(())
}
pub fn draw_frame(term: &mut Stdout) -> io::Result<()> {
	let (width, height) = if let Some((Width(width), Height(height))) = terminal_size::terminal_size() {
		(width, height)
	} else {
		panic!()
	};
	for i in 0..width {
		queue!(
			term,
			cursor::MoveTo(i, 0),
			style::Print(HORIZ_FRAME_CHAR),
			cursor::MoveTo(i, height),
			style::Print(HORIZ_FRAME_CHAR),
		)?
	}
	for i in 1..(height - 1) {
		queue!(
			term,
			cursor::MoveTo(0, i),
			style::Print(VERTI_FRAME_CHAR),
			cursor::MoveTo(width, i),
			style::Print(VERTI_FRAME_CHAR),
		)?
	}
	term.flush()?;
	Ok(())
}