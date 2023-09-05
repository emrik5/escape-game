use std::io::{stdout, self};

use crossterm::{queue, terminal};
use event::ProgramEvent;

pub mod render;
pub mod player;
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
    loop {
        match event::handle_input()? {
            ProgramEvent::Exit => break Ok(()),
            _ => {}
        };
        // render::render(&mut term)?;
        std::thread::sleep(std::time::Duration::from_millis((1.0/FRAME_RATE * 1000.0) as u64));
    }
}
