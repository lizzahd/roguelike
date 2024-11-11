use ::rand::Rng;
use macroquad::prelude::*;

use hot_assets::*;

use crate::primitives::*;
use crate::structures::*;
use crate::types::*;

pub enum WallData {
	Basic,
	CaveWall {
		hardness: f32,
	},
	IronOre {
		hardness: f32,
		iron_amount: i32,
	},
}

pub struct Wall {
	pub rect: Rect,
	pub data: WallData,
}

impl Wall {
	pub fn new(pos: Vec2, data: WallData) -> Self {
		Self {
			rect: Rect::new(pos.x, pos.y, T_SIZE, T_SIZE),
			data,
		}
	}

	pub fn damage(&mut self, amount: f32) -> bool {
		match self.data {
			WallData::CaveWall {ref mut hardness, ..} => {
				*hardness -= amount;
				if *hardness <= 0. {
					true
				} else {
					false
				}
			}
			WallData::IronOre {ref mut hardness, ..} => {
				*hardness -= amount;
				if *hardness <= 0. {
					true
				} else {
					false
				}
			}
			_ => false
		}
	}
}

struct Decal {
	pos: Vec2,
	tex: Texture2D,
	r_orient: bool,
}

impl Decal {
	pub fn new(pos: Vec2, tex: Texture2D, r_orient: bool) -> Self {
		Self {
			pos, tex, r_orient
		}
	}
}

pub struct Chunk {
	pub render_target: RenderTarget,
	pub colliders: Vec<Wall>,
	pub structures: Vec<Box<dyn Structure>>,
	assets: *const AssetManager,
	decals: Vec<Decal>,
}

pub fn get_adj(x: i32, y: i32) -> Vec<(i32, i32)> {
	vec![
		(x, y - 1),
		(x + 1, y - 1),
		(x + 1, y),
		(x + 1, y + 1),
		(x, y + 1),
		(x - 1, y + 1),
		(x - 1, y),
		(x - 1, y - 1),
	]
}

impl Chunk {
	pub const SIZE: usize = 64;

	fn get_wall_at(&self, _pos: Vec2) -> Texture2D {
		unsafe {
			(&*self.assets).images["stone_wall"].clone()
		}
	}

	fn get_floor_at(&self, pos: Vec2) -> Texture2D {
		unsafe {
			if self.get_moisture_at(pos) > 0.1 {
				(&*self.assets).images["cave_soil_floor"].clone()	
			} else {
				(&*self.assets).images["stone_floor"].clone()
			}
		}
	}

	fn get_rubble_at(&self, _pos: Vec2) -> Texture2D {
		unsafe {
			(&*self.assets).images["stone_rubble"].clone()
		}
	}

	fn get_moisture_at(&self, _pos: Vec2) -> f64 {
		0.
	}

	pub fn damage_terrain(&mut self, index: usize, amount: f32) {
		let mut rng = ::rand::thread_rng();

		unsafe {
			if self.colliders[index].damage(amount) {
				self.add_decal(self.colliders[index].rect.point(), self.get_rubble_at(self.colliders[index].rect.point()), false);
				self.colliders.remove(index);
			} else {
				self.add_decal(self.colliders[index].rect.point(), (&*self.assets).images[&format!("crack.{}", rng.gen_range(0..=3))].clone(), true);
			}
		}
	}

	pub fn add_decal(&mut self, pos: Vec2, tex: Texture2D, r_orient: bool) {
		self.decals.push(Decal::new(pos, tex, r_orient));
	}

	pub fn new(assets: &AssetManager) -> Self {
		let target = render_target(Self::SIZE as u32 * T_SIZE as u32, Self::SIZE as u32 * T_SIZE as u32);
		target.texture.set_filter(FilterMode::Nearest);

		Self {
			render_target: target,
			colliders: Vec::new(),
			structures: Vec::new(),
			assets: assets as *const AssetManager,
			decals: Vec::new(),
		}
	}

	pub fn generate(&mut self) {
		self.colliders = Vec::new();
		self.decals = Vec::new();
		self.render_target = render_target(Self::SIZE as u32 * T_SIZE as u32, Self::SIZE as u32 * T_SIZE as u32);
		self.render_target.texture.set_filter(FilterMode::Nearest);

		let mut terrain = [[false; Self::SIZE]; Self::SIZE];
		let percent = 45;
		let smooth_iterations = 2;

		let mut rng = ::rand::thread_rng();

		for x in 0..Self::SIZE {
			for y in 0..Self::SIZE {
				terrain[y][x] = if rng.gen_range(0..100) >= percent {
					false
				} else {
					true
				};
			}
		}

		let cam_scale_factor = (Self::SIZE / 2) as f32 * T_SIZE;
		set_camera(&Camera2D {
            zoom: vec2(1. / cam_scale_factor, 1. / cam_scale_factor),
            target: vec2(Self::SIZE as f32 * T_SIZE / 2., Self::SIZE as f32 * T_SIZE / 2.),
            render_target: Some(self.render_target.clone()),
            ..Default::default()
        });

		for _ in 0..smooth_iterations {
			for x in 0..Self::SIZE {
				for y in 0..Self::SIZE {
					let t_pos = vec2(x as f32 * T_SIZE, y as f32 * T_SIZE);
					if x == 0 && y == 0 {
						let comp_wall = Wall::new(t_pos, WallData::Basic);
						if self.colliders.iter().position(|w: &Wall| w.rect == comp_wall.rect).is_none() {
							draw_texture(&self.get_floor_at(t_pos), t_pos.x, t_pos.y, WHITE);							
						}
						continue;
					}

					let mut n = 0;

					for (ax, ay) in get_adj(x as i32, y as i32) {
						// TODO: Change this when factoring in other chunks
						if ax < 0 || ax >= Self::SIZE as i32 || ay < 0 || ay >= Self::SIZE as i32 {
							n += rng.gen_range(0..=3);
							continue;
						}

						if !terrain[ay as usize][ax as usize] {
							n += 1;
						}
					}

					if n >= 4 {
						// Floor
						let comp_wall = Wall::new(t_pos, WallData::Basic);
						if self.colliders.iter().position(|w: &Wall| w.rect == comp_wall.rect).is_none() {
							draw_texture(&self.get_floor_at(t_pos), t_pos.x, t_pos.y, WHITE);							
						}
					} else if n < 4 {
						// Wall
						let comp_wall = Wall::new(t_pos, WallData::Basic);
						draw_texture(&self.get_wall_at(t_pos), t_pos.x, t_pos.y, WHITE);
						if self.colliders.iter().position(|w: &Wall| w.rect == comp_wall.rect).is_none() {
							self.colliders.push(Wall::new(t_pos, WallData::CaveWall {
								hardness: 3.,
							}));
						}
					}
				}
			}
		}

		set_default_camera();
	}

	pub fn draw(&mut self) {
		draw_texture(&self.render_target.texture, 0., 0., WHITE);
		// for collider in &self.colliders {
		// 	draw_rectangle_lines(collider.rect.x, collider.rect.y, collider.rect.w, collider.rect.h, 1., RED);
		// 	match collider.data {
		// 		WallData::CaveWall {hardness, ..} => {
		// 			draw_text(&format!("{}", hardness as i32), collider.rect.x, collider.rect.y + T_SIZE, T_SIZE / 2., BLUE);
		// 		}
		// 		_ => {}
		// 	}
		// }
		for structure in &mut self.structures {
			structure.draw();
		}
	}

	pub fn update(&mut self) {
		let cam_scale_factor = (Self::SIZE / 2) as f32 * T_SIZE;

		set_camera(&Camera2D {
            zoom: vec2(1. / cam_scale_factor, 1. / cam_scale_factor),
            target: vec2(Self::SIZE as f32 * T_SIZE / 2., Self::SIZE as f32 * T_SIZE / 2.),
            render_target: Some(self.render_target.clone()),
            ..Default::default()
        });

        let mut rng = ::rand::thread_rng();

		for decal in &self.decals {
			let params = if decal.r_orient {
				DrawTextureParams {
					flip_x: rng.gen(),
					flip_y: rng.gen(),
					rotation: rng.gen_range(0..4) as f32 * PI_H,
					..Default::default()
				}
			} else {
				DrawTextureParams {
					..Default::default()
				}
			};

			draw_texture_ex(
				&decal.tex, decal.pos.x, decal.pos.y, WHITE,
				params
			);
		}

		self.decals = Vec::new();

        set_default_camera();
	}
}