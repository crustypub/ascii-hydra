use crate::game::gamestate::GameState;
use crossterm::{cursor::MoveTo, execute, queue, style::Print, terminal};
use std::io::{Result, stdout};

pub fn render(state: &GameState) -> Result<()> {
    let mut stdout = stdout();
    let (_, height) = terminal::size()?; // (columns, rows)

    if let Some((x, y)) = state.tail_to_clear {
        execute!(stdout, MoveTo(x, y), Print(" "))?;
    }

    if let (x, y) = state.berry_position {
        execute!(stdout, MoveTo(x, y), Print("X"))?;
    }

    for i in 0..state.snake.len() {
        let (x, y) = state.snake[i];
        execute!(stdout, MoveTo(x, y), Print(if i == 0 { "@" } else { "*" }))?;
    }

    // 3. Вывод игровой информации (счет)
    let score_text = format!("Score: {}", state.score);
    queue!(stdout, MoveTo(2, height), Print(score_text))?;

    Ok(())
}
