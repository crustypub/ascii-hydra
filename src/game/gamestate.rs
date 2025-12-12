use crossterm::terminal;
use std::io::Result;

pub struct GameState {
    pub score: u32,
    pub high_score: u32,
    pub snake: Vec<(u16, u16)>,
    pub direction: String,
    pub tail_to_clear: Option<(u16, u16)>,
}

fn get_snake_coordinates() -> Vec<(u16, u16)> {
    let (width, height) = terminal::size().expect("Error with read terminal params");

    let mut coordinates = vec![];

    coordinates.push((width / 2, height / 2));
    coordinates.push((width / 2 - 1, height / 2));
    coordinates.push((width / 2 - 2, height / 2));
    return coordinates;
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            score: 0,
            high_score: 0,
            snake: get_snake_coordinates(),
            direction: "left".to_string(),
            tail_to_clear: get_snake_coordinates().last().copied(),
        }
    }

    pub fn add_score(&mut self, points: u32) {
        self.score += points;
        if self.score > self.high_score {
            self.high_score = self.score;
        }
    }

    pub fn reset(&mut self) {
        self.score = 0;
    }

    pub fn change_direction(&mut self, direction: String) {
        self.direction = direction;
    }

    pub fn update(&mut self) {
        // 1. Запоминаем старый хвост
        self.tail_to_clear = self.snake.last().copied();

        // 2. Рассчитываем новую позицию головы
        let head = self.snake[0];
        let new_head = match self.direction.as_str() {
            "left" => (head.0 - 1, head.1),
            "right" => (head.0 + 1, head.1),
            "up" => (head.0, head.1 - 1),
            "down" => (head.0, head.1 + 1),
            _ => head,
        };

        self.snake.pop();
        // 3. Двигаем змейку: добавляем голову, удаляем хвост
        self.snake.insert(0, new_head);

        // // Если не съели еду - удаляем хвост
        // if !self.ate_food {
        //     self.snake.pop();
        //     // tail_to_clear теперь верный - это старый хвост
        // } else {
        //     self.ate_food = false;
        //     // Если съели еду, хвост НЕ удаляем, змейка растёт
        // }
    }
}
