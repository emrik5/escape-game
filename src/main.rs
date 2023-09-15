use std::{io::{stdout, self}, time::Instant};

use crossterm::{queue, terminal};
use entity::EntityCommon;
use event::ProgramEvent;
use player::Player;

pub mod render;
pub mod player;
pub mod entity;
pub mod event;

const FRAME_RATE: f32 = 10.0;

fn main() -> io::Result<()> {
    let mut term = stdout();
    terminal::enable_raw_mode()?;
    queue!(
        term,
        terminal::Clear(terminal::ClearType::All),
    )?;
    render::draw_frame(&mut term)?;
    let glob_time = Instant::now();

    let mut player = Player::create(0, 0);

    loop {
        let start_time = glob_time.elapsed();
        match event::handle_input()? {
            ProgramEvent::Exit => break Ok(()),

            ProgramEvent::MovePlayer(move_dir) => {println!("{:?}", move_dir); player.set_move_dir(move_dir); player.update_pos();},
            _ => {}
        };
        // render::render(&mut term, &player)?;

        let frame_time = glob_time.elapsed() - start_time;
        std::thread::sleep(std::time::Duration::from_millis(
            (1.0/FRAME_RATE * 1000.0 - frame_time.as_millis() as f32).max(0.0) as u64
        ));
    }
}
