use std::{time::Duration, io};

use device_query::{DeviceState, DeviceQuery, Keycode};

use crate::entity::MoveDir;
#[derive(Debug)]
pub enum ProgramEvent {
	None,
	Exit,
	MovePlayer(MoveDir),
}
pub struct HeldKeys {
	keys: Vec<Keycode>
}

pub fn handle_input(device: &DeviceState) -> Option<Vec<ProgramEvent>> {
	let keys = device.get_keys();
	let mut vec = Vec::with_capacity(keys.len());
	for key in keys {
		use Keycode as K;
		use ProgramEvent as P;
		vec.push(match key {
			K::Escape => {ProgramEvent::Exit},

			K::W => {P::MovePlayer(MoveDir::Up)},
			K::S => {P::MovePlayer(MoveDir::Down)},
			K::D => {P::MovePlayer(MoveDir::Right)},
			K::A => {P::MovePlayer(MoveDir::Left)},

			_ => {return None;}
		});
	}
	Some(vec)
}