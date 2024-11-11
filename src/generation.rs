use std::collections::HashMap;
use ::rand::Rng;
use maplit::hashmap;
use macroquad::prelude::*;

use crate::components::*;

fn noise(x: f32, y: f32) -> f32 {
	let mut rng = ::rand::thread_rng();

	(x as f32 + rng.gen_range(0_f32..5_f32)).cos() * rng.gen_range(1_f32..10_f32) - (y as f32 + rng.gen_range(0_f32..5_f32)).sin() * rng.gen_range(1_f32..10_f32)
}

#[derive(Clone)]
pub struct Tile {
	hardness: i16,
	sprite: CharSprite,
}

#[derive(Clone)]
pub struct Chunk {
	pos: I16Vec2,
	tiles: HashMap<I16Vec2, Tile>,
}

impl Chunk {
	fn generate(pos: I16Vec2, size: I16Vec2) -> Self {
		let mut tiles = HashMap::<I16Vec2, Tile>::new();

		for local_x in (0..size.x * T_SIZE).step_by(USIZE_T_SIZE) {
			for local_y in (0..size.y * T_SIZE).step_by(USIZE_T_SIZE) {
				let x = local_x + pos.x;
				let y = local_y + pos.y;

				let mut n = 0;

				for adj in ADJACENT_8 {
					let adj_tile = adj.as_vec2() + vec2(x as f32, y as f32);

					if noise(adj_tile.x, adj_tile.y) > 0.01 {
						n += 1;
					}
				}

				if n > 4 {
					tiles.insert(i16vec2(x, y), Tile::new(n, '#', WHITE));
				} else {
					tiles.remove(&i16vec2(x, y));
				}
			}
		}

		Self {
			pos,
			tiles,
		}
	}

	pub fn draw(&self) {
		draw_rectangle_lines(self.pos.x as f32, self.pos.y as f32, Level::CHUNK_PIXEL_SIZE as f32, Level::CHUNK_PIXEL_SIZE as f32, 5., RED);

		for (pos, tile) in self.tiles.iter() {
			tile.sprite.draw((*pos).as_vec2());
		}
	}
}

impl Tile {
	pub fn new(hardness: i16, sprite: char, color: Color) -> Self {
		Self {
			hardness,
			sprite: CharSprite::new(sprite, color),
		}
	}
}

pub struct Level {
	chunks: HashMap<I16Vec2, Chunk>,
}

impl Level {
	pub const CHUNK_SIZE: i16 = 64;
	pub const CHUNK_PIXEL_SIZE: i16 = Self::CHUNK_SIZE * T_SIZE;

	pub fn new() -> Self {
		Self {
			chunks: hashmap!{
				i16vec2(0, 0) => Chunk::generate(i16vec2(0, 0), i16vec2(Self::CHUNK_SIZE, Self::CHUNK_SIZE)),
			},
		}
	}

	pub fn generate_chunk(&mut self, pos: I16Vec2) {
		let snap_pos = (pos / Self::CHUNK_PIXEL_SIZE) * Self::CHUNK_PIXEL_SIZE;
		if self.chunks.contains_key(&snap_pos) {

		} else {
			self.chunks.insert(snap_pos, Chunk::generate(snap_pos, i16vec2(Self::CHUNK_SIZE, Self::CHUNK_SIZE)));
		}
	}

	pub fn draw(&self, camera: &Camera2D) {
		for (_, chunk) in self.chunks.iter() {
			// draw if it is in the viewport
			let center = (chunk.pos + Self::CHUNK_PIXEL_SIZE).as_vec2();
			
			if center.distance(camera.target) <= screen_width() {
				chunk.draw();
			}
		}
	}
}