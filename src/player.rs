use crate::entity::{MoveDir, EntityCommon, STEP_LENGTH};

pub struct Player {
	xpos: i32,
	ypos: i32,
	move_dir: MoveDir,
}
impl EntityCommon for Player {
    fn update_pos(&mut self) {
        match self.move_dir {
            MoveDir::Up => self.ypos -= STEP_LENGTH,
            MoveDir::Down => self.ypos += STEP_LENGTH,
            MoveDir::Left => self.xpos -= STEP_LENGTH,
            MoveDir::Right => self.xpos += STEP_LENGTH,
        };
    }

    fn set_move_dir(&mut self, move_dir: MoveDir) {
        self.move_dir = move_dir;
    }

    fn create(xpos: i32, ypos: i32) -> Self {
        Player { xpos, ypos, move_dir: MoveDir::Down }
    }

    fn x(&self) -> i32 {
        self.xpos
    }

    fn y(&self) -> i32 {
        self.ypos
    }

    fn term_x(&self) -> u16 {
        self.xpos as u16
    }

    fn term_y(&self) -> u16 {
        self.ypos as u16
    }
}