use std::collections::LinkedList;
use piston_window::{Context, G2d};
use piston_window::types::Color;

use rand::{thread_rng, Rng};

use crate::draw::draw_block;

const RED: f32 = 0.00;
const GREEN: f32 = 0.80;
const BLUE: f32 = 0.00;
const OPACITY: f32 = 1.0;
const SNAKE_COLOR: Color = [RED, GREEN, BLUE, OPACITY];

#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
	Up,
	Down,
	Left,
	Right,
}

impl Direction {
	pub fn opposite(&self) -> Direction {
		match *self {
			Direction::Up => Direction::Down,
			Direction::Down => Direction::Up,
			Direction::Left => Direction::Right,
			Direction::Right => Direction::Left,
		}
	}
}

#[derive(Clone, Debug)]
struct Block {
	x: i32,
	y: i32,
}

pub struct Snake {
	direction: Direction,
	body: LinkedList<Block>,
	tail: Option<Block>,
	color: Color,
}


impl Snake {
	pub fn new(x: i32, y: i32) -> Snake {
		let mut body: LinkedList<Block> = LinkedList::new();
		body.push_back(Block {
			x: x + 2,
			y,
		});
		body.push_back(Block {
			x: x + 1,
			y,
		});
		body.push_back(Block {
			x,
			y,
		});

		Snake {
			direction: Direction::Right,
			body,
			tail: None,
			color: SNAKE_COLOR,
		}
	}

	pub fn draw(&self, con: &Context, g: &mut G2d) {
		for block in &self.body {
			draw_block(self.color, block.x, block.y, con, g);
		}
	}

	pub fn change_color(&mut self) {
		self.color = Snake::get_snake_color();
	}

	fn get_snake_color() -> Color {
		let mut rng = thread_rng();

		let red = rng.gen_range(0.0f32, 1.0f32);
		let green = rng.gen_range(0.0f32, 1.0f32);
		let blue = rng.gen_range(0.0f32, 1.0f32);

		[red, green, blue, OPACITY]
	}

	pub fn head_position(&self) -> (i32, i32) {
		let head_block = self.body.front().unwrap();
		(head_block.x, head_block.y)
	}

	pub fn move_forward(&mut self, dir: Option<Direction>) {
		match dir {
			Some(d) => self.direction = d,
			None => (),
		}

		let (last_x, last_y) = self.head_position();

		let new_block = match self.direction {
			Direction::Up => Block {
				x: last_x,
				y: last_y - 1,
			},
			Direction::Down => Block {
				x: last_x,
				y: last_y + 1,
			},
			Direction::Left => Block {
				x: last_x - 1,
				y: last_y,
			},
			Direction::Right => Block {
				x: last_x + 1,
				y: last_y,
			},
		};
		self.body.push_front(new_block);
		let removed_block = self.body.pop_back().unwrap();
		self.tail = Some(removed_block);
	}

	pub fn head_direction(&self) -> Direction {
		self.direction
	}

	pub fn next_head(&self, dir: Option<Direction>) -> (i32, i32) {
		let (head_x, head_y) = self.head_position();

		let mut moving_dir = self.direction;
		match dir {
			Some(d) => moving_dir = d,
			None => {}
		}

		match moving_dir {
			Direction::Up => (head_x, head_y - 1),
			Direction::Down => (head_x, head_y + 1),
			Direction::Left => (head_x - 1, head_y),
			Direction::Right => (head_x + 1, head_y),
		}
	}

	pub fn restore_tail(&mut self) {
		let block = self.tail.take();
		self.body.push_back(block.unwrap());
	}

	pub fn overlap_tail(&self, x: i32, y: i32) -> bool {
		let mut ch = 0;
		for block in &self.body {
			if x == block.x && y == block.y {
				return true;
			}

			ch += 1;
			if ch == self.body.len() - 1 {
				break;
			}
		}
		false
	}
}
