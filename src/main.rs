use crossterm::{
    cursor::Hide,
    event::{self, Event, KeyCode, KeyEvent, read},
    execute, queue,
    style::{Color, Print, SetBackgroundColor},
    terminal::{self, Clear, ClearType, disable_raw_mode, enable_raw_mode},
};
use std::io::{Write, stdout};
mod game;

fn main() -> std::io::Result<()> {
    // Включаем raw-режим терминала для мгновенного чтения клавиш
    enable_raw_mode()?;
    // Прячем курсор
    execute!(stdout(), Hide)?;

    // Начальная отрисовка фона
    // execute!(
    //     stdout(),
    //     SetBackgroundColor(Color::DarkGrey),
    //     Clear(ClearType::All)
    // )?;

    game::field::draw_field();

    // ГЛАВНЫЙ ИГРОВОЙ ЦИКЛ
    'game_loop: loop {
        // 1. ОБРАБОТКА ВВОДА
        if event::poll(std::time::Duration::from_millis(16))? {
            // ~60 FPS
            if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                match code {
                    KeyCode::Char('q') | KeyCode::Esc => {
                        // Выход из игры
                        break 'game_loop;
                    }
                    _ => {} // Игнорируем другие клавиши
                }
            }
        }
    }

    // Восстанавливаем нормальный режим терминала перед выходом
    disable_raw_mode()?;
    execute!(stdout(), SetBackgroundColor(Color::Reset))?;
    Ok(())
}
