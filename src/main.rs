use rand::{Rng, seq::IteratorRandom};
use rusty_engine::prelude::*;

#[derive(Resource)]
struct GameState {
    marble_labels: Vec<String>,
    cars_left: u32,
    spawn_timer: Timer,
}

const MARBLE_SPEED: f32 = 600.0;
const CAR_PRESETS: [SpritePreset; 5] = [
    SpritePreset::RacingCarBlack,
    SpritePreset::RacingCarBlue,
    SpritePreset::RacingCarGreen,
    SpritePreset::RacingCarRed,
    SpritePreset::RacingCarYellow,
];
const CAR_SPEED: f32 = 250.0;

fn main() {
    let mut game = Game::new();

    game.window_settings(Window {
        title: "Rusty Car Shoot".to_string(),
        ..Default::default()
    });

    game.audio_manager.play_music(MusicPreset::Classy8Bit, 0.15);

    let player1 = game.add_sprite("player1", "sprite/spacerage/player_b_m.png");
    player1.layer = 10.0;
    player1.translation.y = -325.0;

    let cars_left_text = game.add_text("cars_left", "Cars left: 25");
    cars_left_text.translation = Vec2::new(540.0, -320.0);

    game.add_logic(game_logic);
    game.run(GameState {
        marble_labels: vec![
            "marble1".to_string(),
            "marble2".to_string(),
            "marble3".to_string(),
        ],
        cars_left: 25,
        spawn_timer: Timer::from_seconds(0.0, TimerMode::Once),
    });
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    let player1 = engine.sprites.get_mut("player1").unwrap();
    if let Some(mouse_location) = engine.mouse_state.location() {
        player1.translation.x = mouse_location.x;
    }
    let player_x = player1.translation.x;

    if engine.mouse_state.just_pressed(MouseButton::Left) && !game_state.marble_labels.is_empty() {
        let marble_sprite = engine.add_sprite(
            game_state.marble_labels.pop().unwrap(),
            SpritePreset::RollingBallBlue,
        );
        marble_sprite.translation.x = player_x;
        marble_sprite.translation.y = -275.0;
        marble_sprite.layer = 5.0;
        marble_sprite.collision = true;
        engine.audio_manager.play_sfx(SfxPreset::Impact2, 0.4);
    }

    let mut labels_to_delete = Vec::new();

    for sprite in engine.sprites.values_mut() {
        if sprite.label.starts_with("marble") {
            sprite.translation.y += MARBLE_SPEED * engine.delta_f32;
        }
        if sprite.translation.y > 400.0 || sprite.translation.x > 750.0 {
            labels_to_delete.push(sprite.label.clone());
        }
    }

    for label in labels_to_delete {
        engine.sprites.remove(&label);
        if label.starts_with("marble") {
            game_state.marble_labels.push(label);
        }
    }

    if game_state.spawn_timer.tick(engine.delta).just_finished() {
        game_state.spawn_timer =
            Timer::from_seconds(rand::rng().random_range(0.1..1.25), TimerMode::Once);
        if game_state.cars_left > 0 {
            game_state.cars_left -= 1;
            let cars_left_text = engine.texts.get_mut("cars_left").unwrap();
            cars_left_text.value = format!("Cars left: {}", game_state.cars_left);
            let car_sprite = engine.add_sprite(
                format!("car{}", game_state.cars_left),
                *CAR_PRESETS.iter().choose(&mut rand::rng()).unwrap(),
            );
            car_sprite.translation.x = -740.0;
            car_sprite.translation.y = rand::rng().random_range(-100.0..325.0);
            car_sprite.collision = true;
        }
    }

    for sprite in engine.sprites.values_mut() {
        if sprite.label.starts_with("car") {
            sprite.translation.x += CAR_SPEED * engine.delta_f32;
        }
    }

    for event in engine.collision_events.drain(..) {
        if event.state.is_end() || !event.pair.one_starts_with("marble") {
            continue;
        }
        for label in event.pair {
            if label.starts_with("marble") {
                game_state.marble_labels.push(label.clone());
            }
            engine.sprites.remove(&label);
        }
        engine.audio_manager.play_sfx(SfxPreset::Confirmation1, 0.2);
    }
}
