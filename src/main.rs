use rusty_engine::prelude::*;

#[derive(Resource)]
struct GameState {
    marble_labels: Vec<String>,
    cars_left: u32,
    spawn_timer: Timer,
}

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

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {}
