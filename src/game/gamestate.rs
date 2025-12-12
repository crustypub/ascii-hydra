use crossterm::terminal;
use rand::Rng;

pub enum Action {
    Continue,
    Break,
}

pub struct GameState {
    pub score: u32,
    pub high_score: u32,
    pub snake: Vec<(u16, u16)>,
    pub direction: String,
    pub tail_to_clear: Option<(u16, u16)>,
    pub berry_position: (u16, u16),
    pub ate_food: bool,
    pub game_speed: u64,
}

fn get_snake_coordinates() -> Vec<(u16, u16)> {
    let (width, height) = terminal::size().expect("Error with read terminal params");

    let mut coordinates = vec![];

    coordinates.push((width / 2, height / 2));
    coordinates.push((width / 2 + 1, height / 2));
    coordinates.push((width / 2 + 2, height / 2));
    return coordinates;
}

fn get_berry_random_coordinates() -> (u16, u16) {
    let (max_width, max_height) = terminal::size().expect("Error with read terminal params");

    let mut rng = rand::thread_rng();
    let x = rng.random_range(1..max_width - 1);
    let y = rng.random_range(1..max_height - 4);
    (x, y)
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            score: 0,
            high_score: 0,
            snake: get_snake_coordinates(),
            direction: "left".to_string(),
            tail_to_clear: get_snake_coordinates().last().copied(),
            berry_position: get_berry_random_coordinates(),
            ate_food: false,
            game_speed: 250,
        }
    }

    fn game_speed_up(&mut self) {
        if self.game_speed > 50 {
            self.game_speed = self.game_speed - 50;
        }
    }

    pub fn add_score(&mut self) {
        self.score += 1;
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

    pub fn update(&mut self) -> Action {
        let (width, height) = terminal::size().expect("Cant get terminal size"); // (columns, rows)

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

        let (head_x, head_y) = new_head;

        // Проверка на поедание самой себя
        for (x, y) in self.snake.clone() {
            if head_x == x && head_y == y {
                println!("Game Over!");
                return Action::Break;
            }
        }

        // Проверка на пересечение границы
        if head_x == 0 || head_y == 0 || head_x == width - 1 || head_y == height - 3 {
            println!("Game Over");
            return Action::Break;
        }

        if self.berry_position == head {
            self.berry_position = get_berry_random_coordinates();
            self.ate_food = true;
            self.add_score();
            self.game_speed_up();
        } else if self.berry_position != head {
            self.ate_food = false;
        }

        // 3. Двигаем змейку: добавляем голову, удаляем хвост
        self.snake.insert(0, new_head);

        // // Если не съели еду - удаляем хвост
        if !self.ate_food {
            self.snake.pop();
        } else {
            self.ate_food = false;
            // Если съели еду, хвост НЕ удаляем, змейка растёт
        }

        return Action::Continue;
    }
}
