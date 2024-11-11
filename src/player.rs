use macroquad::prelude::*;

use hot_assets::*;

use crate::primitives::*;
use crate::level::*;
use crate::types::*;
use crate::structures::*;
use crate::menus::*;

enum ControlMode {
	Build,
	Move,
}

pub struct Player {
	pub id: usize,
	end_turn: bool,
	pub rect: Rect,

	mining_speed: f32,

	assets: *const AssetManager,
	tex: *const Texture2D,

	control_mode: ControlMode,
	pub blueprint: Option<Blueprint>,
	pub selected_blueprint: BlueprintType,

	pub menu: Option<Menu>,

	hp: i32,
	dead: bool,
}

impl Player {
	pub fn new(pos: Vec2, assets: &AssetManager) -> Self {
		Self {
			id: 0,
			end_turn: false,

			rect: Rect::new(pos.x, pos.y, T_SIZE, T_SIZE),
			mining_speed: 1.,
			
			assets: assets as *const AssetManager,
			tex: &assets.images["dwarf"] as *const Texture2D,

			control_mode: ControlMode::Move,
			selected_blueprint: BlueprintType::DEFAULT,
			blueprint: None,

			menu: None,

			hp: 10,
			dead: false,
		}
	}

	fn move_to(&mut self, direction: Vec2, world: &mut Chunk, entities: *mut Vec<Box<dyn Entity>>) -> bool {
		let d_pos = self.rect.point() + direction;
		let check_wall = Wall::new(d_pos, WallData::Basic);

		if let Some(i) = world.colliders.iter().position(|w: &Wall| w.rect == check_wall.rect) {
			world.damage_terrain(i, self.mining_speed);
			return true;
		}

		for structure in &mut world.structures {
			if structure.rect().contains(check_wall.rect.center()) {
				if structure.collides() {
					return true;
				}
			}
		}

		unsafe {
			for entity in &mut *entities {
			}
		}

		self.rect.move_to(d_pos);

		false
	}
}

lazy_derive!(Damageable, Player);

const DIRECTION_CONTROLS: [Vec2; 9] = [
	vec2(-T_SIZE, T_SIZE),
	vec2(0., T_SIZE),
	vec2(T_SIZE, T_SIZE),
	vec2(-T_SIZE, 0.),
	vec2(0., 0.),
	vec2(T_SIZE, 0.),
	vec2(-T_SIZE, -T_SIZE),
	vec2(0., -T_SIZE),
	vec2(T_SIZE, -T_SIZE),
];

impl Entity for Player {
    fn update(&mut self, _entities: *mut Vec<Box<dyn Entity>>, _world: &mut Chunk) {
    }

    fn draw(&mut self, entities: *mut Vec<Box<dyn Entity>>, world: &mut Chunk) {
    	self.end_turn = false;

    	match self.control_mode {
    		ControlMode::Build => {
    			if is_key_pressed(KeyCode::Escape) {
    				self.control_mode = ControlMode::Move;
    				self.blueprint = None;
    			}

    			if let Some(ref mut blueprint) = &mut self.blueprint {
    				blueprint.draw(world);

    				if is_key_pressed(KeyCode::Kp1) {
			    		blueprint.move_toward(DIRECTION_CONTROLS[0], world);
					} else if is_key_pressed(KeyCode::Kp2) {
			    		blueprint.move_toward(DIRECTION_CONTROLS[1], world);
					} else if is_key_pressed(KeyCode::Kp3) {
			    		blueprint.move_toward(DIRECTION_CONTROLS[2], world);
					} else if is_key_pressed(KeyCode::Kp4) {
			    		blueprint.move_toward(DIRECTION_CONTROLS[3], world);
					} else if is_key_pressed(KeyCode::Kp6) {
			    		blueprint.move_toward(DIRECTION_CONTROLS[5], world);
					} else if is_key_pressed(KeyCode::Kp7) {
			    		blueprint.move_toward(DIRECTION_CONTROLS[6], world);
					} else if is_key_pressed(KeyCode::Kp8) {
			    		blueprint.move_toward(DIRECTION_CONTROLS[7], world);
					} else if is_key_pressed(KeyCode::Kp9) {
			    		blueprint.move_toward(DIRECTION_CONTROLS[8], world);
					}

					if is_key_pressed(KeyCode::Tab) {
						self.menu = Some(Menu {});
					}

					if is_key_pressed(KeyCode::Q) {
						blueprint.rotate_left(world);
					} else if is_key_pressed(KeyCode::E) {
						blueprint.rotate_right(world);
					}

					if is_key_pressed(KeyCode::Enter) {
						blueprint.update_valid(world);

						if blueprint.valid {
							blueprint.place(world);
						}

						blueprint.update_valid(world);
					}
    			}
    		}
    		ControlMode::Move => {
    			if is_key_pressed(KeyCode::B) {
    				self.control_mode = ControlMode::Build;
    				unsafe {
    					self.blueprint = Some(
	    					Blueprint::new(
	    						vec2(self.rect.x + T_SIZE, self.rect.y),
	    						&*self.assets,
	    						self.selected_blueprint,
	    					)
	    				);
    				}
    			}

    			if is_key_pressed(KeyCode::Kp1) {
		    		self.move_to(DIRECTION_CONTROLS[0], world, entities);
		    		self.end_turn = true;
				} else if is_key_pressed(KeyCode::Kp2) {
		    		self.move_to(DIRECTION_CONTROLS[1], world, entities);
		    		self.end_turn = true;
				} else if is_key_pressed(KeyCode::Kp3) {
		    		self.move_to(DIRECTION_CONTROLS[2], world, entities);
		    		self.end_turn = true;
				} else if is_key_pressed(KeyCode::Kp4) {
		    		self.move_to(DIRECTION_CONTROLS[3], world, entities);
		    		self.end_turn = true;
				} else if is_key_pressed(KeyCode::Kp5) {
		    		self.move_to(DIRECTION_CONTROLS[4], world, entities);
		    		self.end_turn = true;
				} else if is_key_pressed(KeyCode::Kp6) {
		    		self.move_to(DIRECTION_CONTROLS[5], world, entities);
		    		self.end_turn = true;
				} else if is_key_pressed(KeyCode::Kp7) {
		    		self.move_to(DIRECTION_CONTROLS[6], world, entities);
		    		self.end_turn = true;
				} else if is_key_pressed(KeyCode::Kp8) {
		    		self.move_to(DIRECTION_CONTROLS[7], world, entities);
		    		self.end_turn = true;
				} else if is_key_pressed(KeyCode::Kp9) {
		    		self.move_to(DIRECTION_CONTROLS[8], world, entities);
		    		self.end_turn = true;
				}
    		}
    	}

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
    	EntityData::Player {
    		rect: self.rect,
    		end_turn: self.end_turn,
    		obj: self as *mut Self,
    	}
    }

    fn id(&self) -> usize {
    	self.id
    }
}