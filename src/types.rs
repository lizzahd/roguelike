use macroquad::prelude::*;

pub use crate::lazy_derive;

#[macro_export]
macro_rules! lazy_derive {
	(Damageable, $name:ident) => {
		impl Damageable for $name {
			fn hp(&self) -> i32 {
				self.hp
			}

			fn hurt(&mut self, damage: i32) -> bool {
				self.hp -= damage;

				if self.hp <= 0 {
					self.dead = true;
					return true;
				}

				false
			}
		}
	};
	(Structure, $name:ident) => {
		impl Structure for $name {
			fn hp(&self) -> i32 {
				self.hp
			}

			fn hurt(&mut self, damage: i32) -> bool {
				self.hp -= damage;

				if self.hp <= 0 {
					self.dead = true;
					return true;
				}

				false
			}

			fn rect(&self) -> Rect {
				self.rect
			}

			fn collides(&self) -> bool {
				self.collides
			}

			fn draw(&mut self) {
				self.draw();
			}

			fn draw_blueprint(&mut self, valid: bool) {
				unsafe {
					let color = if valid {
						GREEN
					} else {
						RED
					};

					draw_texture_ex(
						&*self.tex,
						self.rect.x, self.rect.y,
						color,
						DrawTextureParams {
							..Default::default()
						}
					)
				}
			}

			fn move_toward(&mut self, rhs: Vec2) {
				self.rect.offset(rhs);
			}
		}
	};
}

pub trait Damageable {
	fn hp(&self) -> i32;
	fn hurt(&mut self, _damage: i32) -> bool;
}

pub trait Structure {
	fn hp(&self) -> i32;
	fn rect(&self) -> Rect;
	fn collides(&self) -> bool;
	fn hurt(&mut self, _damage: i32) -> bool;
	fn draw(&mut self);
	fn draw_blueprint(&mut self, valid: bool);
	fn move_toward(&mut self, rhs: Vec2);
}