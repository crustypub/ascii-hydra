use crate::game::gamestate::GameState;
use crossterm::{cursor::MoveTo, execute, queue, style::Print};
use std::io::{Result, stdout};

pub fn render(state: &GameState) -> Result<()> {
    let mut stdout = stdout();

    if let Some((x, y)) = state.tail_to_clear {
        execute!(stdout, MoveTo(x, y), Print(" "))?;
    }

    for i in 0..state.snake.len() {
        let (x, y) = state.snake[i];
        execute!(stdout, MoveTo(x, y), Print(if i == 0 { "@" } else { "*" }))?;
    }

    Ok(())
}
