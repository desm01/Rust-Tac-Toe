use std::{io, error::Error, time::Duration};

use crossterm::{terminal::{self, EnterAlternateScreen, LeaveAlternateScreen}, ExecutableCommand, cursor::{Hide, Show}, event::{Event, self, KeyCode}};
use tic_tac_toe::board::Board;





fn main() -> Result<(), Box<dyn Error>> {
    let mut board = Board::new();
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;
    board.draw_board(&mut stdout);

    'gameloop: loop {
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => break 'gameloop,
                    _ => board.draw_board(&mut stdout)
                }
            }
        }
    }

    //Clean up
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    //Cleanup
     Ok(())
}
