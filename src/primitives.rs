use std::f32::consts::PI;
use macroquad::prelude::*;

use crate::level::*;
use crate::player::*;

pub const T_SIZE: f32 = 48.;
pub const PI_H: f32 = PI / 2.;

pub use crate::deref;
#[macro_export]
macro_rules! deref {
	($r:expr) => {
		unsafe {
			*$r
		}
	}
}

pub fn adj_8_t() -> Vec<Vec2> {
	vec![
		vec2(0., -T_SIZE),
		vec2(T_SIZE, -T_SIZE),
		vec2(T_SIZE, 0.),
		vec2(T_SIZE, T_SIZE),
		vec2(0., T_SIZE),
		vec2(-T_SIZE, T_SIZE),
		vec2(-T_SIZE, 0.),
		vec2(-T_SIZE, -T_SIZE),
	]
}

pub enum EntityData {
	Player {
		rect: Rect,
		end_turn: bool,
		obj: *mut Player,
	},
	Kobold {
		rect: Rect,
	},
}

pub trait Entity {
    fn update(&mut self, _entities: *mut Vec<Box<dyn Entity>>, _world: &mut Chunk);
    fn draw(&mut self, _entities: *mut Vec<Box<dyn Entity>>, _world: &mut Chunk);

    fn data(&mut self) -> EntityData;
    fn id(&self) -> usize;
}