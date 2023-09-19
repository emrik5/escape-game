use std::{io::{stdout, self}, time::Instant, ops::Add};

use crossterm::{queue, terminal, style::Print};
use device_query::{DeviceState, DeviceEvents};
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
    let device = DeviceState::new();
    terminal::enable_raw_mode()?;
    queue!(
        term,
        terminal::Clear(terminal::ClearType::All),
    )?;
    render::draw_frame(&mut term)?;
    let glob_time = Instant::now();

    let mut player = Player::create(0, 0);

    'main: loop  {
        let start_time = glob_time.elapsed();
        for event in event::handle_input(&device).unwrap_or(vec![]) {
            match event {
                ProgramEvent::Exit => break 'main,
    
                ProgramEvent::MovePlayer(move_dir) => {player.set_move_dir(move_dir); player.update_pos()},
                _ => {}
            };
        }

        // player.update_pos();
        render::render(&mut term, &player)?;

        let frame_time = glob_time.elapsed() - start_time;
        std::thread::sleep(std::time::Duration::from_millis(
            (1.0/FRAME_RATE * 1000.0 - frame_time.as_millis() as f32).max(0.0) as u64
        ));
    }
    terminal::disable_raw_mode()?;
    queue!(
        term,
        terminal::Clear(terminal::ClearType::All),
    )?;
    Ok(())
}
