use macroquad::prelude::*;

use hot_assets::*;

use crate::primitives::*;
use crate::level::*;
use crate::a_star::*;
use crate::types::*;

pub struct Kobold {
	pub id: usize,
	rect: Rect,
	target: Option<*mut Box<dyn Entity>>,
	path: Option<Vec<Vec2>>,

	tex: *const Texture2D,

	hp: i32,
	dead: bool,
}

impl Kobold {
	pub const MAX_HP: i32 = 10;

	pub fn new(pos: Vec2, assets: &AssetManager) -> Self {
		Self {
			id: 0,

			rect: Rect::new(pos.x, pos.y, T_SIZE, T_SIZE),
			target: None,
			path: None,
			
			tex: &assets.images["kobold"] as *const Texture2D,

			hp: Self::MAX_HP,
			dead: false,
		}
	}

	fn move_toward(&mut self, direction: Vec2, world: &mut Chunk) -> bool {
		let d_pos = self.rect.point() + direction;
		let check_wall = Wall::new(d_pos, WallData::Basic);

		if world.colliders.iter().position(|w: &Wall| w.rect == check_wall.rect).is_some() {
			return true;
		}

		self.rect.move_to(d_pos);

		false
	}

	fn move_to(&mut self, new_pos: Vec2, world: &mut Chunk) -> bool {
		if world.colliders.iter().position(|w: &Wall| w.rect.point() == new_pos).is_some() {
			return true;
		}

		self.rect.move_to(new_pos);

		false
	}
}

lazy_derive!(Damageable, Kobold);

impl Entity for Kobold {
    fn update(&mut self, entities: *mut Vec<Box<dyn Entity>>, world: &mut Chunk) {
    	unsafe {
    		if let Some(target) = &self.target {
	    		if let Some(path) = &self.path {
	    			dbg!(path);
	    		} else {
	    			match (**target).data() {
	    				EntityData::Player {rect, ..} => {
			    			// self.path = Some(astar(world, self.rect.point(), rect.point()));
			    			if let Some(new_pos) = shite_step(world, self.rect.point(), rect.point()) {
			    				self.move_to(new_pos, world);
			    			}
	    				}
	    				_ => {}
	    			}
	    		}
	    	} else {
				for entity in &mut *entities {
					match entity.data() {
						EntityData::Player {rect, ..} => {
							if self.rect.center().distance(rect.center()) <= 3. * T_SIZE {
								self.target = Some(entity as *mut Box<dyn Entity>);
								break;
							}
						}
						_ => {}
					}
				}
	    	}
    	}
    }

    fn draw(&mut self, _entities: *mut Vec<Box<dyn Entity>>, _world: &mut Chunk) {
    	unsafe {
    		draw_texture_ex(
	    		&*self.tex as &Texture2D, self.rect.x, self.rect.y, WHITE,
	    		DrawTextureParams {
	    			..Default::default()
	    		}
	    	);
    	}
    }

    fn data(&mut self) -> EntityData {
    	EntityData::Kobold {
    		rect: self.rect,
    	}
    }

    fn id(&self) -> usize {
    	self.id
    }
}