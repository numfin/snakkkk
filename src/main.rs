use glutin_window::GlutinWindow;
use graphics;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::{
    event_loop::{EventLoop, EventSettings, Events},
    input::*,
    window::WindowSettings,
};
use std::collections::LinkedList;
use std::iter::FromIterator;

const SCREEN_SIZE: (f64, f64) = (1000.0, 800.0);
const SNAKE_SIZE: f64 = 20.0;
const MAX_X: f64 = (SCREEN_SIZE.0 / SNAKE_SIZE) - 1.0;
const MAX_Y: f64 = (SCREEN_SIZE.1 / SNAKE_SIZE) - 1.0;
pub struct Game {
    gl: GlGraphics,
    snake: Snake,
}

impl Game {
    fn render(&mut self, args: &RenderArgs) {
        const BG_COLOR: [f32; 4] = [0.5, 1.3, 0.8, 0.5];
        self.gl.draw(args.viewport(), |_c, gl| {
            graphics::clear(BG_COLOR, gl);
        });

        self.snake.render(&mut self.gl, args)
    }

    fn update(&mut self) {
        self.snake.update()
    }

    fn pressed(&mut self, button: &Button) {
        let last_direction = self.snake.direction.clone();

        self.snake.direction = match button {
            &Button::Keyboard(Key::W) if last_direction != Direction::Down => Direction::Up,
            &Button::Keyboard(Key::S) if last_direction != Direction::Up => Direction::Down,
            &Button::Keyboard(Key::D) if last_direction != Direction::Left => Direction::Right,
            &Button::Keyboard(Key::A) if last_direction != Direction::Right => Direction::Left,
            _ => last_direction,
        };
    }
}

#[derive(Clone, PartialEq, Debug)]
enum Direction {
    Left,
    Right,
    Down,
    Up,
}

#[derive(Debug, Clone)]
struct Position {
    x: f64,
    y: f64,
}

struct Snake {
    body: LinkedList<Position>,
    direction: Direction,
}

impl Snake {
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        const SNAKE_COLOR: [f32; 4] = [0.8, 0.6, 1.0, 1.0];

        for position in self.body.iter() {
            let square = graphics::rectangle::square(
                (position.x * SNAKE_SIZE).into(),
                (position.y * SNAKE_SIZE).into(),
                SNAKE_SIZE.into(),
            );
            gl.draw(args.viewport(), |c, gl| {
                let transform = c.transform;
                graphics::rectangle(SNAKE_COLOR, square, transform, gl)
            })
        }
    }

    fn update(&mut self) {
        let mut new_head = (*self.body.front().expect("Snake has no body")).clone();

        match self.direction {
            Direction::Left => {
                new_head.x = if new_head.x - 1.0 < 0.0 {
                    MAX_X
                } else {
                    new_head.x - 1.0
                }
            }
            Direction::Right => {
                new_head.x = if new_head.x + 1.0 > MAX_X {
                    0.0
                } else {
                    new_head.x + 1.0
                }
            }
            Direction::Up => {
                new_head.y = if new_head.y - 1.0 < 0.0 {
                    MAX_Y
                } else {
                    new_head.y - 1.0
                }
            }
            Direction::Down => {
                new_head.y = if new_head.y + 1.0 > MAX_Y {
                    0.0
                } else {
                    new_head.y + 1.0
                }
            }
        }

        self.body.push_front(new_head);
        self.body.pop_back().unwrap();
    }
}

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: GlutinWindow =
        WindowSettings::new("spinning-square", [SCREEN_SIZE.0, SCREEN_SIZE.1])
            .graphics_api(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap();

    let mut game = Game {
        gl: GlGraphics::new(opengl),
        snake: Snake {
            body: LinkedList::from_iter(
                vec![
                    Position { x: 0.0, y: 0.0 },
                    Position { x: 1.0, y: 0.0 },
                    Position { x: 3.0, y: 0.0 },
                    Position { x: 4.0, y: 0.0 },
                    Position { x: 5.0, y: 0.0 },
                    Position { x: 6.0, y: 0.0 },
                    Position { x: 7.0, y: 0.0 },
                ]
                .into_iter(),
            ),
            direction: Direction::Right,
        },
    };

    let mut events = Events::new(EventSettings::new().ups(30));

    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            game.render(&args);
        }
        if let Some(args) = e.button_args() {
            if args.state == ButtonState::Press {
                game.pressed(&args.button);
            }
        }
        if let Some(_) = e.update_args() {
            game.update();
        }
    }
}
