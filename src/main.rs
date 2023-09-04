use std::io::stdout;

pub mod render;

fn main() {
    let mut term = stdout();
    render::render(&mut term);
    loop {
        
    }
}
