use macroquad::prelude::*;

use crate::types::*;
use crate::primitives::*;
use crate::level::*;

use hot_assets::*;

#[derive(Copy, Clone, Debug)]
pub enum BlueprintType {
	DieselGenerator,
	Crusher,
	ArcFurnace,
	SteelWall,
}

impl BlueprintType {
	pub const DEFAULT: Self = Self::DieselGenerator;
}

pub struct Blueprint {
	pub rect: Rect,
	pub valid: bool,

	pub rotation: f32,

	blueprint_type: BlueprintType,

	tex: *const Texture2D,
	assets: *const AssetManager,
}

impl Blueprint {
	pub fn new(pos: Vec2, assets: &AssetManager, t: BlueprintType) -> Self {
		let rect: Rect;
		let tex: *const Texture2D;

		match t {
			BlueprintType::DieselGenerator => {
				rect = Rect::new(pos.x + T_SIZE + 1., pos.y + 1., DieselGenerator::SIZE.x - 2., DieselGenerator::SIZE.y - 2.);
				tex = &assets.images["diesel_generator"] as *const Texture2D;
			}
			BlueprintType::Crusher => {
				rect = Rect::new(pos.x + T_SIZE + 1., pos.y + 1., Crusher::SIZE.x - 2., Crusher::SIZE.y - 2.);
				tex = &assets.images["crusher"] as *const Texture2D;
			}
			BlueprintType::ArcFurnace => {
				rect = Rect::new(pos.x + T_SIZE + 1., pos.y + 1., ArcFurnace::SIZE.x - 2., ArcFurnace::SIZE.y - 2.);
				tex = &assets.images["arc_furnace"] as *const Texture2D;
			}
			BlueprintType::SteelWall => {
				rect = Rect::new(pos.x + T_SIZE + 1., pos.y + 1., SteelWall::SIZE.x - 2., SteelWall::SIZE.y - 2.);
				tex = &assets.images["steel_plate_wall"] as *const Texture2D;
			}
		}

		Self {
			// rect: Rect::new(rect.x + 1., rect.y + 1., rect.w - 2., rect.h - 2.),
			rect,
			valid: false,

			rotation: 0.,

			blueprint_type: t,

			tex,
			assets: assets as *const AssetManager,
		}
	}

	pub fn place(&mut self, world: &mut Chunk) {
		let structure: Box<dyn Structure>;

		unsafe {
			match self.blueprint_type {
				BlueprintType::DieselGenerator => {
					structure = Box::new(DieselGenerator::new(self.rect, &*self.assets, self.rotation));
				}
				BlueprintType::Crusher => {
					structure = Box::new(Crusher::new(self.rect, &*self.assets, self.rotation));
				}
				BlueprintType::ArcFurnace => {
					structure = Box::new(ArcFurnace::new(self.rect, &*self.assets, self.rotation));
				}
				BlueprintType::SteelWall => {
					structure = Box::new(SteelWall::new(self.rect, &*self.assets, self.rotation));
				}
			}

			world.structures.push(
				structure
			);
		}

		self.update_valid(world);
	}

	pub fn update_valid(&mut self, world: &Chunk) {
		self.valid = true;

		// let check_rect = Rect::new(self.rect.x + 1., self.rect.y + 1., self.rect.w - 2., self.rect.h - 2.);

		for wall in &world.colliders {
			if wall.rect.overlaps(&self.rect) {
				self.valid = false;
				break;
			}	
		}

		for structure in &world.structures {
			if structure.rect().overlaps(&self.rect) {
				self.valid = false;
				break;
			}
		}
	}

	pub fn move_toward(&mut self, rhs: Vec2, world: &Chunk) {
		let new_rect = self.rect.offset(rhs);
		
		self.rect = new_rect;

		self.update_valid(world);
	}

	pub fn rotate_left(&mut self, world: &Chunk) {
		self.rotation -= PI_H;
		let new_rect = Rect::new(self.rect.x, self.rect.y, self.rect.h, self.rect.w);
		self.rect = new_rect;
		self.update_valid(world);
	}

	pub fn rotate_right(&mut self, world: &Chunk) {
		self.rotation += PI_H;
		let new_rect = Rect::new(self.rect.x, self.rect.y, self.rect.h, self.rect.w);
		self.rect = new_rect;
		self.update_valid(world);
	}

	pub fn draw(&mut self, _world: &Chunk) {
		let color = if self.valid {
			GREEN
		} else {
			RED
		};


		unsafe {
			draw_texture_ex(
				&*self.tex,
				self.rect.x, self.rect.y,
				color,
				DrawTextureParams {
					rotation: self.rotation,
					..Default::default()
				}
			)
		}
		draw_rectangle_lines(self.rect.x, self.rect.y, self.rect.w, self.rect.h, 3., BLUE);
	}
}

pub struct DieselGenerator {
	hp: i32,
	dead: bool,
	rect: Rect,
	collides: bool,
	rotation: f32,

	tex: *const Texture2D,
}

impl DieselGenerator {
	pub const SIZE: Vec2 = vec2(T_SIZE * 5., T_SIZE * 3.);
	pub const NAME: &str = "Diesel Generator";

	pub fn new(rect: Rect, assets: &AssetManager, rotation: f32) -> Self {
		Self {
			hp: 30,
			dead: false,
			// rect: Rect::new(pos.x + 1., pos.y + 1., (T_SIZE * 5.) - 2., (T_SIZE * 3.) - 2.),
			rect,
			collides: true,
			rotation,

			tex: &assets.images["diesel_generator"] as *const Texture2D,
		}
	}

	fn draw(&mut self) {
		unsafe {
			draw_texture_ex(
				&*self.tex,
				self.rect.x, self.rect.y,
				WHITE,
				DrawTextureParams {
					rotation: self.rotation,
					..Default::default()
				}
			)
		}
	}
}

lazy_derive!(Structure, DieselGenerator);

pub struct Crusher {
	hp: i32,
	dead: bool,
	rect: Rect,
	collides: bool,
	rotation: f32,

	tex: *const Texture2D,
}

impl Crusher {
	pub const SIZE: Vec2 = vec2(T_SIZE * 5., T_SIZE * 3.);
	pub const NAME: &str = "Crusher";

	pub fn new(rect: Rect, assets: &AssetManager, rotation: f32) -> Self {
		Self {
			hp: 30,
			dead: false,
			rect,
			collides: true,
			rotation,

			tex: &assets.images["crusher"] as *const Texture2D,
		}
	}

	fn draw(&mut self) {
		unsafe {
			draw_texture_ex(
				&*self.tex,
				self.rect.x, self.rect.y,
				WHITE,
				DrawTextureParams {
					rotation: self.rotation,
					..Default::default()
				}
			)
		}
	}
}

lazy_derive!(Structure, Crusher);

pub struct ArcFurnace {
	hp: i32,
	dead: bool,
	rect: Rect,
	collides: bool,
	rotation: f32,

	tex: *const Texture2D,
}

impl ArcFurnace {
	pub const SIZE: Vec2 = vec2(T_SIZE * 2., T_SIZE * 2.);
	pub const NAME: &str = "Arc Furnace";

	pub fn new(rect: Rect, assets: &AssetManager, rotation: f32) -> Self {
		Self {
			hp: 30,
			dead: false,
			rect,
			collides: true,
			rotation,

			tex: &assets.images["arc_furnace"] as *const Texture2D,
		}
	}

	fn draw(&mut self) {
		unsafe {
			draw_texture_ex(
				&*self.tex,
				self.rect.x, self.rect.y,
				WHITE,
				DrawTextureParams {
					rotation: self.rotation,
					..Default::default()
				}
			)
		}
	}
}

lazy_derive!(Structure, ArcFurnace);

pub struct SteelWall {
	hp: i32,
	dead: bool,
	rect: Rect,
	collides: bool,
	rotation: f32,

	tex: *const Texture2D,
}

impl SteelWall {
	pub const SIZE: Vec2 = vec2(T_SIZE, T_SIZE);
	pub const NAME: &str = "Steel Wall";

	pub fn new(rect: Rect, assets: &AssetManager, rotation: f32) -> Self {
		Self {
			hp: 10,
			dead: false,
			rect,
			collides: true,
			rotation,

			tex: &assets.images["steel_plate_wall"] as *const Texture2D,
		}
	}

	fn draw(&mut self) {
		unsafe {
			draw_texture_ex(
				&*self.tex,
				self.rect.x, self.rect.y,
				WHITE,
				DrawTextureParams {
					rotation: self.rotation,
					..Default::default()
				}
			)
		}
	}
}

lazy_derive!(Structure, SteelWall);