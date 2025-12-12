# ASCII Hydra
![Demo](https://github.com/crustypub/ascii-hydra/blob/master/gameplay.gif)

ASCII Hydra is a classic snake game implemented in Rust that runs directly in your terminal. Control the growing hydra, eat the food, and avoid collisions with yourself and the walls.

## How to Play

The objective is to guide the snake (`@***`) to eat the food (`X`) that appears on the screen. Each piece of food eaten makes the snake longer and increases your score. The game ends if the snake runs into the walls or its own body. As your score increases, the game's speed will also increase.

**Controls:**
- **Arrow Keys (Up, Down, Left, Right):** Change the snake's direction.
- **`q` or `Esc`:** Quit the game.

## Features

- **Terminal-Based Interface:** Play the classic snake game in any modern terminal.
- **Score Tracking:** Your current score is displayed at the bottom of the screen.
- **Increasing Difficulty:** The snake's speed increases as you eat more food.
- **Cross-Platform:** Built with Rust and `crossterm`, it can run on Windows, macOS, and Linux.

## Installation & Usage

To run ASCII Hydra, you need to have the Rust toolchain installed on your system.

1.  **Clone the repository:**
    ```sh
    git clone https://github.com/crustypub/ascii-hydra.git
    ```

2.  **Navigate to the project directory:**
    ```sh
    cd ascii-hydra
    ```

3.  **Build and run the game:**
    ```sh
    cargo run
    ```

Cargo will handle the dependencies, compile the project, and launch the game in your terminal.

## Project Structure

The project is organized into several modules to separate concerns:

-   `src/main.rs`: The main application entry point. It initializes the game, handles the main game loop, and processes user input.
-   `src/game/gamestate.rs`: Defines the `GameState` struct which holds all the game's state, including snake position, score, food location, and game speed. It also contains the core game logic for updating the state each frame.
-   `src/game/field.rs`: Contains functions for drawing the initial game field, including the borders.
-   `src/game/gameplay.rs`: Handles the rendering of dynamic game elements like the snake, food, and score during each frame of the game loop.
-   `Cargo.toml`: Contains project metadata and dependencies.

## Dependencies

-   [crossterm](https://github.com/crossterm-rs/crossterm): A cross-platform terminal manipulation library for cursor control, styling, and reading input.
-   [rand](https://github.com/rust-random/rand): A library for random number generation, used to place food at random positions.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
