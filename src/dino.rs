use crossterm::{
    cursor, queue,
    style::{ Color, Print, SetForegroundColor},
    terminal::{self, Clear, ClearType},
    ExecutableCommand,
};
use std::io::Result;
use rand::Rng;
use std::{
    io::{stdout, Write},
    thread,
    time::{Duration, Instant},
};

const WIDTH: usize = 80;
const HEIGHT: usize = 20;
const GROUND_LEVEL: usize = HEIGHT - 2;

struct Dino {
    x: usize,
    y: usize,
    jumping: bool,
    jump_velocity: f32,
    gravity: f32,
}

struct Obstacle {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    passed: bool,
}

struct Game {
    dino: Dino,
    obstacles: Vec<Obstacle>,
    score: u32,
    game_over: bool,
    speed: f32,
    last_obstacle_time: Instant,
}

impl Dino {
    fn new() -> Self {
        Dino {
            x: 10,
            y: GROUND_LEVEL,
            jumping: false,
            jump_velocity: 0.0,
            gravity: 0.5,
        }
    }

    fn jump(&mut self) {
        if !self.jumping {
            self.jumping = true;
            self.jump_velocity = -3.0;
        }
    }

    fn update(&mut self) {
        if self.jumping {
            self.y = (self.y as f32 + self.jump_velocity) as usize;
            self.jump_velocity += self.gravity;

            if self.y >= GROUND_LEVEL {
                self.y = GROUND_LEVEL;
                self.jumping = false;
            }
        }
    }
}

impl Obstacle {
    fn new_cactus(x: usize) -> Self {
        Obstacle {
            x,
            y: GROUND_LEVEL - 1,
            width: 3,
            height: 2,
            passed: false,
        }
    }

    fn new_bird(x: usize) -> Self {
        Obstacle {
            x,
            y: GROUND_LEVEL - 3,
            width: 4,
            height: 1,
            passed: false,
        }
    }

    fn update(&mut self, speed: f32) {
        self.x = (self.x as f32 - speed) as usize;
    }

    fn collides_with(&self, dino: &Dino) -> bool {
        let dino_right = dino.x + 4;
        let dino_bottom = dino.y + 2;
        let obstacle_right = self.x + self.width;
        let obstacle_bottom = self.y + self.height;

        dino.x < obstacle_right &&
        dino_right > self.x &&
        dino.y < obstacle_bottom &&
        dino_bottom > self.y
    }
}

impl Game {
    fn new() -> Self {
        Game {
            dino: Dino::new(),
            obstacles: Vec::new(),
            score: 0,
            game_over: false,
            speed: 2.0,
            last_obstacle_time: Instant::now(),
        }
    }

    fn spawn_obstacle(&mut self) {
        let mut rng = rand::rng();
        let obstacle_type = rng.random_range(0..=1);

        let obstacle = if obstacle_type == 0 {
            Obstacle::new_cactus(WIDTH)
        } else {
            Obstacle::new_bird(WIDTH)
        };

        self.obstacles.push(obstacle);
    }

    fn update(&mut self) {
        if self.game_over {
            return;
        }

        // Update dino
        self.dino.update();

        // Update obstacles
        for obstacle in &mut self.obstacles {
            obstacle.update(self.speed);
            
            // Check if passed
            if !obstacle.passed && obstacle.x + obstacle.width < self.dino.x {
                obstacle.passed = true;
                self.score += 1;
            }
        }

        // Remove obstacles that are off screen
        self.obstacles.retain(|o| o.x + o.width > 0);

        // Spawn new obstacles
        if self.last_obstacle_time.elapsed() > Duration::from_secs_f32(2.0 / self.speed) {
            self.spawn_obstacle();
            self.last_obstacle_time = Instant::now();
        }

        // Check collisions
        for obstacle in &self.obstacles {
            if obstacle.collides_with(&self.dino) {
                self.game_over = true;
                break;
            }
        }

        // Increase speed over time
        self.speed += 0.001;
    }

    fn draw(&self, stdout: &mut std::io::Stdout) -> Result<()> {
        // Clear screen
        queue!(stdout, Clear(ClearType::All))?;

        // Draw ground
        for x in 0..WIDTH {
            queue!(
                stdout,
                cursor::MoveTo(x as u16, GROUND_LEVEL as u16),
                Print("#")
            )?;
        }

        // Draw dino
        let dino_color = if self.game_over { Color::Red } else { Color::Green };
        queue!(
            stdout,
            SetForegroundColor(dino_color),
            cursor::MoveTo(self.dino.x as u16, self.dino.y as u16),
            Print("\\O/"),
            cursor::MoveTo(self.dino.x as u16 + 1, (self.dino.y + 1) as u16),
            Print("/ \\"),
            SetForegroundColor(Color::Reset)
        )?;

        // Draw obstacles
        for obstacle in &self.obstacles {
            for y in obstacle.y..obstacle.y + obstacle.height {
                for x in obstacle.x..obstacle.x + obstacle.width {
                    queue!(
                        stdout,
                        cursor::MoveTo(x as u16, y as u16),
                        Print(if obstacle.height > 1 { "¥" } else { "^" })
                    )?;
                }
            }
        }

        // Draw score
        queue!(
            stdout,
            cursor::MoveTo(0, 0),
            Print(format!("Score: {}", self.score))
        )?;

        // Draw game over message
        if self.game_over {
            queue!(
                stdout,
                cursor::MoveTo((WIDTH / 2 - 5) as u16, (HEIGHT / 2) as u16),
                Print("GAME OVER!"),
                cursor::MoveTo((WIDTH / 2 - 10) as u16, (HEIGHT / 2 + 1) as u16),
                Print("Press SPACE to restart")
            )?;
        }

        stdout.flush()?;
        Ok(())
    }

    fn reset(&mut self) {
        *self = Game::new();
    }
}

pub fn init_game() -> Result<()> {
    // Initialize terminal
    let mut stdout = stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(terminal::EnterAlternateScreen)?;
    stdout.execute(cursor::Hide)?;

    // Game loop
    let mut game = Game::new();
    let mut last_frame = Instant::now();
    let frame_duration = Duration::from_secs_f32(1.0 / 30.0);

    loop {
        // Handle input
        if crossterm::event::poll(Duration::from_secs(0))? {
            if let crossterm::event::Event::Key(key_event) = crossterm::event::read()? {
                match key_event.code {
                    crossterm::event::KeyCode::Char(' ') => {
                        if game.game_over {
                            game.reset();
                        } else {
                            game.dino.jump();
                        }
                    }
                    crossterm::event::KeyCode::Esc => break,
                    _ => {}
                }
            }
        }

        // Update game state
        if last_frame.elapsed() > frame_duration {
            game.update();
            last_frame = Instant::now();
        }

        // Draw
        game.draw(&mut stdout)?;

        // Sleep to prevent high CPU usage
        thread::sleep(Duration::from_millis(10));
    }

    // Clean up terminal
    stdout.execute(terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    stdout.execute(cursor::Show)?;

    Ok(())
}