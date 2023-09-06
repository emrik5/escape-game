use std::io::{stdout, self};

use crossterm::{queue, terminal};
use entity::EntityCommon;
use event::ProgramEvent;
use player::Player;

pub mod render;
pub mod player;
pub mod entity;
pub mod event;

const FRAME_RATE: f32 = 30.0;

fn main() -> io::Result<()> {
    let mut term = stdout();
    terminal::enable_raw_mode()?;
    queue!(
        term,
        terminal::Clear(terminal::ClearType::All)
    )?;
    render::draw_frame(&mut term)?;

    let mut player = Player::create(0, 0);

    loop {
        match event::handle_input()? {
            ProgramEvent::Exit => break Ok(()),

            ProgramEvent::MovePlayer(move_dir) => player.set_move_dir(move_dir),
            _ => {}
        };
        player.update_pos();
        render::render(&mut term, &player)?;
        std::thread::sleep(std::time::Duration::from_millis((1.0/FRAME_RATE * 1000.0) as u64));
    }
}
