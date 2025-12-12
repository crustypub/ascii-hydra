use crossterm::{
    cursor::Hide,
    event::{self, Event, KeyCode, KeyEvent, read},
    execute, queue,
    style::{Color, Print, SetBackgroundColor},
    terminal::{self, Clear, ClearType, disable_raw_mode, enable_raw_mode},
};
use std::{
    io::{Write, stdout},
    thread,
    time::{Duration, Instant},
};
mod game;

fn main() -> std::io::Result<()> {
    // Частота обновления
    let frame_duration = Duration::from_millis(500); // .5 секунд

    // Глобальное состояние игры.
    let mut game_state = game::gamestate::GameState::new();

    // Включаем raw-режим терминала для мгновенного чтения клавиш
    enable_raw_mode()?;
    // Прячем курсор
    execute!(stdout(), Hide)?;

    // Отрисовка фона
    game::field::draw_field(&game_state)?;

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
                    KeyCode::Left => {
                        game_state.change_direction("left".to_string());
                    }
                    KeyCode::Right => {
                        game_state.change_direction("right".to_string());
                    }
                    KeyCode::Up => {
                        game_state.change_direction("up".to_string());
                    }
                    KeyCode::Down => {
                        game_state.change_direction("down".to_string());
                    }
                    _ => {} // Игнорируем другие клавиши
                }
            }
        }

        // Обновление
        game_state.update();

        // Рендер
        game::gameplay::render(&game_state)?;

        let frame_start = Instant::now(); // Для замера времени

        let elapsed = frame_start.elapsed();
        if elapsed < frame_duration {
            thread::sleep(frame_duration - elapsed);
        }
    }

    // Восстанавливаем нормальный режим терминала перед выходом
    disable_raw_mode()?;
    execute!(stdout(), SetBackgroundColor(Color::Reset))?;
    Ok(())
}
