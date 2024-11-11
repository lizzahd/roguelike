use macroquad::prelude::*;

use crate::structures::*;
use crate::primitives::*;

const TEXT_PADDING: Vec2 = vec2(T_SIZE, T_SIZE);

pub struct Menu;
impl Menu {
	pub async fn run(&mut self) -> BlueprintType {
		let option_names = vec![
			DieselGenerator::NAME,
			Crusher::NAME,
			ArcFurnace::NAME,
			SteelWall::NAME,
		];

		let options = vec![
			BlueprintType::DieselGenerator,
			BlueprintType::Crusher,
			BlueprintType::ArcFurnace,
			BlueprintType::SteelWall,
		];

		let mut cursor_index = 0;
		let mut selected_index = 0;
		let mut current_option = options[selected_index];

		set_default_camera();

		loop {
			clear_background(BLACK);

			if is_key_pressed(KeyCode::Escape) {
				return current_option;
			}

			if is_key_pressed(KeyCode::Kp2) || is_key_pressed(KeyCode::Down) {
				if cursor_index == options.len() - 1 {
					cursor_index = 0;
				} else {
					cursor_index += 1;
				}
			}

			if is_key_pressed(KeyCode::Kp8) || is_key_pressed(KeyCode::Up) {
				if cursor_index == 0 {
					cursor_index = options.len() - 1;
				} else {
					cursor_index -= 1;
				}
			}

			if is_key_pressed(KeyCode::Enter) {
				selected_index = cursor_index;
				current_option = options[selected_index];
			}

			for i in 0..option_names.len() {
				let color = if i == selected_index {
					GREEN
				} else if i == cursor_index {
					WHITE
				} else {
					LIGHTGRAY
				};

				draw_text(option_names[i], TEXT_PADDING.x * 2., TEXT_PADDING.y + (i as f32 * T_SIZE), T_SIZE, color);
			}

			draw_text(">", TEXT_PADDING.x, TEXT_PADDING.y + (cursor_index as f32 * T_SIZE), T_SIZE, WHITE);

			next_frame().await;
		}

		current_option
	}
}