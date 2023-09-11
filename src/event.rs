use std::{time::Duration, io};

use crossterm::event::{poll, read, Event, KeyEvent, KeyCode, KeyEventKind};

use crate::entity::MoveDir;

pub enum ProgramEvent {
	None,
	Exit,
	MovePlayer(MoveDir),
}
pub struct HeldKeys {
	keys: Vec<KeyCode>
}

pub fn handle_input() -> io::Result<(ProgramEvent, HeldKeys)> {
	loop {
		let held_keys = "";
		break if poll(Duration::from_millis(0))? {
			match read()? {
				Event::Key(KeyEvent { code: KeyCode::Char('w'), .. }) => { Ok(ProgramEvent::MovePlayer(MoveDir::Up)) },
				Event::Key(KeyEvent { code: KeyCode::Char('s'), .. }) => { Ok(ProgramEvent::MovePlayer(MoveDir::Down)) },
				Event::Key(KeyEvent { code: KeyCode::Char('a'), .. }) => { Ok(ProgramEvent::MovePlayer(MoveDir::Left)) },
				Event::Key(KeyEvent { code: KeyCode::Char('d'), .. }) => { Ok(ProgramEvent::MovePlayer(MoveDir::Right)) },

				// Exit Program
				Event::Key(KeyEvent { code: KeyCode::Esc, .. }) => { Ok(ProgramEvent::Exit) },
				_ => { Ok(ProgramEvent::None) }
			}
		} else{
			Ok(ProgramEvent::None)
		} 
	}
}