use std::{time::Duration, io};

use crossterm::event::{poll, read, Event, KeyEvent, KeyCode};

pub enum ProgramEvent {
	None,
	Exit
}

pub fn handle_input() -> io::Result<ProgramEvent> {
	loop {
		if poll(Duration::from_millis(0))? {
			match read()? {
				Event::Key(KeyEvent { code: KeyCode::Char('w'), .. }) => { print!("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAA") },
				Event::Key(KeyEvent { code: KeyCode::Esc, .. }) => { break Ok(ProgramEvent::Exit); },
				_ => {}
			}
		} else{
			break Ok(ProgramEvent::None);
		} 
	}
}