use crossterm::{
    cursor::MoveTo,
    execute, queue,
    style::{Color, Print, SetBackgroundColor, SetForegroundColor},
    terminal::{self, Clear, ClearType},
};
use std::io::{Write, stdout};

struct GameField {
    width: u16,
    height: u16,
}

pub fn draw_field() -> std::io::Result<()> {
    let (width, height) = terminal::size()?; // (columns, rows)
    let mut score: u16 = 0;

    let field_width = width;
    let field_height = height - 2;

    let field: GameField = GameField {
        width: field_width,
        height: field_height,
    };

    let mut stdout = stdout();

    // 1. Очистка экрана (или части)

    execute!(
        stdout,
        SetBackgroundColor(Color::DarkGrey),
        SetForegroundColor(Color::White),
        terminal::Clear(ClearType::All),
    )?;

    // 2. Рисование границ
    // Верхняя и нижняя границы
    for x in 0..field.width {
        queue!(stdout, MoveTo(x, 0), Print("#"),)?; // Верх
        queue!(stdout, MoveTo(x, field.height - 1), Print("#"),)?; // Низ
    }
    // Левая и правая границы
    for y in 0..field.height {
        queue!(stdout, MoveTo(0, y), Print("#"))?; // Лево
        queue!(stdout, MoveTo(field.width - 1, y), Print("#"))?; // Право
    }

    // 3. Вывод игровой информации (счет)
    let score_text = format!("Score: {}", score);
    queue!(stdout, MoveTo(2, field.height), Print(score_text))?;

    stdout.flush()?;
    Ok(())
}
