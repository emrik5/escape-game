pub const STEP_LENGTH: i32 = 1;
pub trait EntityCommon {
	fn update_pos(&mut self);
	fn set_move_dir(&mut self, move_dir: MoveDir);
	fn create(xpos: i32, ypos: i32) -> Self;
	fn x(&self) -> i32;
	fn y(&self) -> i32;
	fn term_x(&self) -> u16;
	fn term_y(&self) -> u16;
}
#[derive(Debug)]
pub enum MoveDir {
	Up,
	Down,
	Left,
	Right,
}