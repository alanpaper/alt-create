use std::{
    collections::VecDeque,
    time::{Duration, Instant},
};

use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use rand::Rng;

// 游戏状态
struct Game {
    snake: VecDeque<(u16, u16)>, // 蛇身体 (x, y)
    food: (u16, u16),            // 食物位置
    direction: Direction,        // 当前移动方向
    width: u16,                  // 游戏区域宽度
    height: u16,                 // 游戏区域高度
    game_over: bool,             // 游戏结束标志
    score: u32,                  // 得分
}

// 移动方向枚举
#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Game {
    // 初始化新游戏
    fn new(width: u16, height: u16) -> Self {
        let mut snake = VecDeque::new();
        // 初始蛇身 (居中位置)
        let start_x = width / 2;
        let start_y = height / 2;
        snake.push_back((start_x, start_y));
        snake.push_back((start_x - 1, start_y));
        snake.push_back((start_x - 2, start_y));

        let food = Game::generate_food(&snake, width, height);

        Game {
            snake,
            food,
            direction: Direction::Right,
            width,
            height,
            game_over: false,
            score: 0,
        }
    }

    // 生成食物位置
    fn generate_food(snake: &VecDeque<(u16, u16)>, width: u16, height: u16) -> (u16, u16) {
        let mut rng = rand::rng();
        loop {
            let x = rng.random_range(1..width - 1);
            let y = rng.random_range(1..height - 1);
            // 确保食物不会生成在蛇身上
            if !snake.contains(&(x, y)) {
                return (x, y);
            }
        }
    }

    // 更新游戏状态
    fn update(&mut self) {
        if self.game_over {
            return;
        }

        // 获取蛇头位置
        let head = self.snake.front().unwrap();
        let new_head = match self.direction {
            Direction::Up => (head.0, head.1 - 1),
            Direction::Down => (head.0, head.1 + 1),
            Direction::Left => (head.0 - 1, head.1),
            Direction::Right => (head.0 + 1, head.1),
        };

        // 检查碰撞
        if new_head.0 == 0
            || new_head.0 >= self.width - 1
            || new_head.1 == 0
            || new_head.1 >= self.height - 1
            || self.snake.contains(&new_head)
        {
            self.game_over = true;
            return;
        }

        // 移动蛇
        self.snake.push_front(new_head);

        // 检查是否吃到食物
        if new_head == self.food {
            self.score += 10;
            self.food = Game::generate_food(&self.snake, self.width, self.height);
        } else {
            self.snake.pop_back(); // 没吃到食物，移除尾部
        }
    }

    // 绘制游戏界面
    fn draw(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut stdout = std::io::stdout();
        execute!(stdout, crossterm::cursor::MoveTo(0, 0))?;

        // 绘制边界
        for y in 0..self.height {
            for x in 0..self.width {
                if x == 0 || x == self.width - 1 || y == 0 || y == self.height - 1 {
                    print!("I");
                } else if (x, y) == self.food {
                    print!("$"); // 食物
                } else if self.snake.contains(&(x, y)) {
                    print!("*"); // 蛇身
                } else {
                    print!(" ");
                }
            }
            println!();
        }

        // 显示分数
        println!("Score: {}", self.score);
        if self.game_over {
            println!("Game Over! Press 'q' to quit, 'r' to restart");
        }

        Ok(())
    }

    // 处理输入
    fn handle_input(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Char('q') => return Ok(true), // 退出游戏
                    KeyCode::Char('r') if self.game_over => {
                        *self = Game::new(self.width, self.height);
                    }
                    KeyCode::Up if self.direction != Direction::Down => {
                        self.direction = Direction::Up
                    }
                    KeyCode::Down if self.direction != Direction::Up => {
                        self.direction = Direction::Down
                    }
                    KeyCode::Left if self.direction != Direction::Right => {
                        self.direction = Direction::Left
                    }
                    KeyCode::Right if self.direction != Direction::Left => {
                        self.direction = Direction::Right
                    }
                    _ => {}
                }
            }
        }
        Ok(false)
    }
}

pub async fn init_game() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化终端
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, Hide)?;

    // 获取终端大小
    let (width, height) = crossterm::terminal::size()?;
    let mut game = Game::new(width, height - 2); // 留两行给分数和消息

    // 游戏主循环
    let mut last_update = Instant::now();
    let base_speed = Duration::from_millis(150);
    let update_interval = match game.direction {
        Direction::Up | Direction::Down => base_speed * 2,
        _ => base_speed,
    };

    loop {
        // 处理输入
        if game.handle_input()? {
            break;
        }

        // 定时更新游戏状态
        if last_update.elapsed() >= update_interval && !game.game_over {
            game.update();
            last_update = Instant::now();
        }

        // 绘制游戏
        game.draw()?;
    }

    // 恢复终端设置
    execute!(stdout, Show, LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}