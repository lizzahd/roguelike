use macroquad::prelude::*;

use hot_assets::*;

use crate::level::*;
use crate::primitives::*;
use crate::player::*;
use crate::kobold::*;
use crate::structures::*;
use crate::menus::*;

mod level;
mod primitives;
mod player;
mod a_star;
mod kobold;
mod types;
mod structures;
mod menus;

fn conf() -> Conf {
    Conf {
        window_width: 1152,
        window_height: 648,
        ..Default::default()
    }
}

const MIN_FRAME_TIME: f32 = 1. / 60.;

#[macroquad::main(conf)]
async fn main () {
    let assets = AssetManager::new().await;

    let mut chunk = Chunk::new(&assets);
    chunk.generate();

    let mut last_mouse_position = mouse_position();
    let mut camera_target = vec2(0., 0.);
    let mut zoom = 0.001;

    let mut global_id = 0;
    let mut entities = Vec::<Box<dyn Entity>>::new();

    macro_rules! add_entity {
        ($entity:expr) => {
            $entity.id = global_id;
            global_id += 1;
            entities.push(Box::new($entity));
        }
    }

    add_entity!(Player::new(vec2(0., 0.), &assets));

    loop {
        clear_background(BLACK);

        let camera = Camera2D {
            target: camera_target,
            zoom: vec2(zoom, zoom * screen_width() / screen_height()),
            offset: vec2(0., 0.),
            ..Default::default()
        };

        set_camera(&camera);

        chunk.draw();

        let mut update = false;
        let entities_ptr = &mut entities as *mut Vec<Box<dyn Entity>>;
        for i in 0..entities.len() {
            entities[i].draw(entities_ptr, &mut chunk);

            unsafe {
                match entities[i].data() {
                    EntityData::Player {end_turn, rect, ref mut obj, ..} => {
                        if let Some(ref mut menu) = (**obj).menu {
                            let result = menu.run().await;
                            set_camera(&camera);

                            (**obj).selected_blueprint = result;
                            let blueprint = Blueprint::new((**obj).rect.point(), &assets, result);
                            (**obj).blueprint = Some(blueprint);

                            (**obj).menu = None;
                        }

                        camera_target = rect.center();
                        if end_turn {
                            update = true;
                        }
                    }
                    _ => {}
                }
            }
        }

        if update {
            for i in 0..entities.len() {
                entities[i].update(entities_ptr, &mut chunk);
            }
        }

        let mouse_rel = Vec2::from_array(last_mouse_position.into()) - Vec2::from_array(mouse_position().into());
        let mouse_pos = camera.screen_to_world(Vec2::from_array(mouse_position().into()));

        if is_mouse_button_down(MouseButton::Middle) {
            camera_target += mouse_rel + zoom;
        }

        zoom += mouse_wheel().1 / 1000000.;
        zoom = zoom.max(0.);

        last_mouse_position = mouse_position();

        chunk.update();

        set_default_camera();

        let frame_time = get_frame_time();
        if frame_time < MIN_FRAME_TIME {
            std::thread::sleep(std::time::Duration::from_secs_f32(MIN_FRAME_TIME - frame_time));
        }

        next_frame().await;
    }
}