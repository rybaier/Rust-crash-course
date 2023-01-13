use rusty_engine::prelude::*;
use rand::prelude::*;

struct GameState {
    high_score: u32,
    score:u32,
    // enemy_label: Vec<String>,
    ferris_index: i32,
    spawn_timer: Timer,

}

impl Default for GameState {
    fn default () -> Self {
        Self {
            high_score: 0,
            score: 0,
            // enemy_label: Vec::new(),
            ferris_index: 0,
            spawn_timer: Timer::from_seconds(2.0, true),

        }
    }
}


fn main() {
    let mut game = Game::new();
//    set up game here 
    game.window_settings(WindowDescriptor{
        title: "Crab Squish!".to_string(),
        // width: 1400.0,
        // height: 500.0,
        ..Default::default()
    });

    game.audio_manager.play_music(MusicPreset::MysteriousMagic, 0.1);

    let player = game.add_sprite("player", SpritePreset::RacingCarBlue);
    player.translation = Vec2::new(0.0, 0.0);
    player.rotation = WEST;
    player.scale = 1.0; 
    player.layer = 1.0;
    player.collision = true;

    // let car1 = game.add_sprite("car1", SpritePreset::RacingCarYellow);
    // car1.translation = Vec2::new(300.0, 0.0);
    // car1.collision = true;

    let score = game.add_text("score", "Score: 0");
    score.translation = Vec2::new(520.0, 320.0);

    let high_score = game.add_text("high_score", "High Score: 0");
    high_score.translation = Vec2::new(-520.0, 320.0);
    // let ferris = game.add_sprite("ferris", "cute-ferris.png");
    // ferris.translation = Vec2::new(-200.0, 100.0);
    // ferris.scale = 0.1;
    // ferris.collision = true;

    game.add_logic(game_logic);
    game.run(GameState::default());
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
// actual game logic goes here
// quit if q is pushed
if engine.keyboard_state.just_pressed(KeyCode::Q) {
    engine.should_exit = true;
}
 
// Keep text near the edges of screen
let offset = ((engine.time_since_startup_f64 * 4.0).cos() * 5.0) as f32;
let score = engine.texts.get_mut("score").unwrap();
score.translation.x = engine.window_dimensions.x / 2.0 - 80.0;
score.translation.y = engine.window_dimensions.y / 2.0 - 30.0 + offset;
let high_score = engine.texts.get_mut("high_score").unwrap();
high_score.translation.x = -engine.window_dimensions.x / 2.0 + 100.0;
high_score.translation.y = engine.window_dimensions.y / 2.0 - 30.0 + offset;

// Handle collisions
    // engine.show_coliiders = true
    for event in engine.collision_events.drain(..){
        // println!("{:?}", event);
        if event.state == CollisionState::Begin && event.pair.one_starts_with("player") {
            // remove the sprite the player collides with 
            for label in [event.pair.0, event.pair.1]{
                if label != "player"{
                    engine.sprites.remove(&label);
                }
            }
            game_state.score +=1;
            let score = engine.texts.get_mut("score").unwrap();
            score.value = format!("Score: {}", game_state.score);  
            if game_state.score > game_state.high_score {
                game_state.high_score = game_state.score;
                let high_score = engine.texts.get_mut("high_score").unwrap();
                high_score.value = format!("High Score: {}", game_state.high_score);  
            }
            engine.audio_manager.play_sfx(SfxPreset::Minimize2, 0.3)
        }
    }
//  Handle Movement
    let player = engine.sprites.get_mut("player").unwrap();
    // player.translation.x += 100.0 * engine.delta_f32;
    const MOVEMENT_SPEED: f32 = 100.0;
    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::Up, KeyCode::W]) {
        player.translation.y += MOVEMENT_SPEED * engine.delta_f32;
    }
    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::Down, KeyCode::S]) {
        player.translation.y -= MOVEMENT_SPEED * engine.delta_f32;
    }
    if engine
    .keyboard_state
    .pressed_any(&[KeyCode::Right, KeyCode::D]) {
    player.translation.x += MOVEMENT_SPEED * engine.delta_f32;
    }
    if engine
        .keyboard_state
        .pressed_any(&[ KeyCode::Left, KeyCode::A]) {
        player.translation.x -= MOVEMENT_SPEED * engine.delta_f32;
    }
    //  handle mouse input
    if engine.mouse_state.just_pressed(MouseButton::Left){
        if let Some(mouse_location) = engine.mouse_state.location(){
            let label = format!("Feris {}", game_state.ferris_index);
            game_state.ferris_index +=1;
            let ferris = engine.add_sprite(label.clone(), "cute-ferris.png");
            ferris.translation = mouse_location;
            ferris.scale = 0.1;
            ferris.collision = true;
        }
    }

    if game_state.spawn_timer.tick(engine.delta).just_finished(){
        let label = format!("Feris {}", game_state.ferris_index);
        game_state.ferris_index +=1;
        let ferris = engine.add_sprite(label.clone(), "cute-ferris.png");
        ferris.translation.x = thread_rng().gen_range(-550.0..550.0);
        ferris.translation.y = thread_rng().gen_range(-350.0..350.0);
        ferris.scale = 0.1;
        ferris.collision = true;
    }
    // Reset score
     if engine.keyboard_state.just_pressed(KeyCode::R){
        game_state.score = 0;
        let score = engine.texts.get_mut("score").unwrap();
        score.value = "Score: 0".to_string();
     }
     
}