// src/main.rs
mod intersection;
mod road;
mod traffic_light;
mod vehicle;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::collections::HashMap;
use std::time::Duration;
use rand::Rng;

use traffic_light::{LightState, TrafficLight};
use vehicle::{Direction, Vehicle};

use road::Road;

const SAFE_DISTANCE: i32 = 50;
const SINGLE_ROAD_PART: i32 = 350;
const VEHICULE_LENGTH: i32 = 40;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Road Intersection", 800, 800)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::RGB(20, 40, 20));
    canvas.clear();
    canvas.present();

    let mut light_s = TrafficLight::new(460, 460, 20, 20, LightState::Red);
    let mut light_w = TrafficLight::new(320, 460, 20, 20, LightState::Red);
    let mut light_n = TrafficLight::new(320, 320, 20, 20, LightState::Green);
    let mut light_e = TrafficLight::new(460, 320, 20, 20, LightState::Red);
    let mut vehicles = Vec::new();

    let mut last_spawn = HashMap::new();
    last_spawn.insert(Direction::North, 0);
    last_spawn.insert(Direction::South, 0);
    last_spawn.insert(Direction::East, 0);
    last_spawn.insert(Direction::West, 0);

    let mut event_pump = sdl_context.event_pump()?;
    let mut n = 0;
    let mut green_timer = 0.;
    let mut frame_count = 0;
    let mut current_light: u8 = 1;

    'running: loop {
        frame_count += 1;

        // Input handling
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    let direction = match keycode {
                        Keycode::Up => Some(Direction::North),
                        Keycode::Down => Some(Direction::South),
                        Keycode::Left => Some(Direction::West),
                        Keycode::Right => Some(Direction::East),
                        Keycode::R => {
                            let mut rng = rand::rng();
                            let val = rng.random_range(0..4);
                            let res = match val {
                                0 => Some(Direction::North),
                                1 => Some(Direction::South),
                                2 => Some(Direction::West),
                                3 => Some(Direction::East),
                                _ => unreachable!(),
                            };
                            res
                        },
                        Keycode::Escape => break 'running,
                        _ => None,
                    };

                    if let Some(dir) = direction {
                        if is_safe_to_spawn(&vehicles, dir, &last_spawn, frame_count) {
                            vehicles.push(Vehicle::new(dir));
                            last_spawn.insert(dir, frame_count);
                        }
                    }
                }
                _ => {}
            }
        }

        n += 1;
        if n as f32 > green_timer * 150. {
            n = 0;
             let direction = match current_light {
                0 => Direction::South,
                1 => Direction::West,
                2 => Direction::North,
                3 => Direction::East,
                _ => unreachable!(),
            };
            // println!("{} {} {} {} {:?}", green_timer, green_timer * 150., n, current_light, direction);

            green_timer = get_timer(&vehicles, direction);
            // Turn all lights red
            light_n.update(false);
            light_s.update(false);
            light_e.update(false);
            light_w.update(false);

            if green_timer != 0. {
                match current_light {
                    0 => light_n.update(true),
                    1 => light_e.update(true),
                    2 => light_s.update(true),
                    3 => light_w.update(true),
                    _ => unreachable!(),
                };
            }

            current_light = (current_light + 1) % 4;
        }

        // Compute tentative positions (with traffic light checks)
        let mut tentatives: Vec<Vehicle> = vehicles
            .iter()
            .map(|v| {
                let mut tentative_v = v.clone();
                let light_state = match tentative_v.direction {
                    Direction::North => light_s.state,
                    Direction::South => light_n.state,
                    Direction::East => light_w.state,
                    Direction::West => light_e.state,
                };

                tentative_v.update(light_state);
                tentative_v
            })
            .collect();

        let mut safe_to_move = vec![true; tentatives.len()];
        for i in 0..tentatives.len() {
            let (before, rest) = tentatives.split_at_mut(i);
            let (current, after) = rest.split_first_mut().unwrap();
            let dir = current.direction;

            let closest_ahead = before
                .iter()
                .chain(after.iter())
                .filter(|other| other.direction == dir)
                .filter_map(|other| {
                    let distance = match dir {
                        Direction::North => {
                            current.rect.y() - (other.rect.y() + other.rect.height() as i32)
                        }
                        Direction::South => {
                            other.rect.y() - (current.rect.y() + current.rect.height() as i32)
                        }
                        Direction::East => {
                            other.rect.x() - (current.rect.x() + current.rect.width() as i32)
                        }
                        Direction::West => {
                            current.rect.x() - (other.rect.x() + other.rect.width() as i32)
                        }
                    };
                    if distance >= 0 {
                        Some(distance)
                    } else {
                        None
                    }
                })
                .min();

            if let Some(distance) = closest_ahead {
                if distance < SAFE_DISTANCE {
                    safe_to_move[i] = false;
                }
            }
        }


        let mut collisions = vec![false; tentatives.len()];
        for i in 0..tentatives.len() {
            for j in (i + 1)..tentatives.len() {
                let a = &tentatives[i];
                let b = &tentatives[j];

                if (a.in_intersection || b.in_intersection) && !a.has_turned && !b.has_turned {
                    continue;
                }

                if a.rect.has_intersection(b.rect) {
                    collisions[i] = true;
                    collisions[j] = true;
                }
            }
        }

        // Update original vehicles only if safe
        for (i, vehicle) in vehicles.iter_mut().enumerate() {
            if safe_to_move[i] && !collisions[i] {
                *vehicle = tentatives[i].clone();
            }
        }
        vehicles.retain(|v| is_on_screen(v));

        canvas.set_draw_color(Color::RGB(20, 40, 20));
        canvas.clear();

        let road_ns = Road::new(350, 0, 100, 800, true); // North‑South road 
        let road_ew = Road::new(0, 350, 800, 100, false); // East‑West road 
        road_ns.draw(&mut canvas);
        road_ew.draw(&mut canvas);
        intersection::draw(&mut canvas);
        light_n.draw(&mut canvas);
        light_s.draw(&mut canvas);
        light_e.draw(&mut canvas);
        light_w.draw(&mut canvas);
        for vehicle in &vehicles {
            vehicle.draw(&mut canvas);
        }
        canvas.present();

        std::thread::sleep(Duration::from_millis(16));
    }
    Ok(())
}

fn is_safe_to_spawn(
    vehicles: &[Vehicle],
    direction: Direction,
    last_spawn: &HashMap<Direction, i32>,
    current_frame: i32,
) -> bool {
    if let Some(last_frame) = last_spawn.get(&direction) {
        if current_frame - last_frame < SAFE_DISTANCE / 2 {
            return false;
        }
    }

    let (spawn_coord, is_vertical) = match direction {
        Direction::North => (760, true),
        Direction::South => (80, true), 
        Direction::East => (80, false), 
        Direction::West => (760, false),
    };

    // Check distance from existing vehicles in the same direction
    for vehicle in vehicles.iter().filter(|v| v.direction == direction) {
        let vehicle_pos = if is_vertical {
            vehicle.rect.y() + vehicle.rect.height() as i32
        } else {
            vehicle.rect.x() + vehicle.rect.width() as i32
        };

        let distance = (vehicle_pos - spawn_coord).abs();
        if distance < SAFE_DISTANCE {
            return false;
        }
    }

    true
}

// Function to check if a vehicle is still on screen
fn is_on_screen(vehicle: &Vehicle) -> bool {
    let rect = vehicle.rect;
    match vehicle.direction {
        Direction::North => rect.y() > -50,
        Direction::South => rect.y() < 850,
        Direction::East => rect.x() < 850,
        Direction::West => rect.x() > -50,
    }
}

fn get_timer(vehicles: &[Vehicle], direction: Direction) -> f32 {
    let mut cars_count = 0;
    for car in vehicles {
        if !car.has_turned && !car.in_intersection && car.direction == direction {
            cars_count += 1;
        }
    }
    (cars_count as f32 * 1.55) / (SINGLE_ROAD_PART / (VEHICULE_LENGTH + SAFE_DISTANCE)) as f32
}
