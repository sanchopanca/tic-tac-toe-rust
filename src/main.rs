mod game;
mod random;
mod ui;
use game::{GameState, Player};
use macroquad::prelude::*;
use ui::{draw, draw_ui, UIPhase};

fn conf() -> Conf {
    Conf {
        window_title: "Tic Tac Toe".to_owned(),
        window_width: 800,
        window_height: 800,
        ..Default::default()
    }
}

pub struct App {
    game: game::Game,
    ui_phase: UIPhase,
    human_player: game::Player,
}

#[macroquad::main(conf)]
async fn main() {
    let mut app = create_app();
    loop {
        process(&mut app);
        draw(&app);
        draw_ui(&mut app);
        next_frame().await
    }
}

fn create_app() -> App {
    init_random();
    App {
        game: game::Game::new(),
        ui_phase: UIPhase::NewGame,
        human_player: Player::O,
    }
}

fn init_random() {
    let time = get_time() * 10_000_000.0;
    quad_rand::srand(time as u64);
}

fn process(app: &mut App) {
    if app.ui_phase != UIPhase::Playing {
        return;
    }
    if app.game.get_state() != GameState::Playing {
        app.ui_phase = UIPhase::GameOver(app.game.get_state());
        return;
    }
    if app.game.get_turn() == app.human_player {
        if let Some((x, y)) = get_input() {
            app.game.play(x, y);
        }
        return;
    }
    if let Some((x, y)) = game::ai_move(&app.game) {
        app.game.play(x, y);
    }
}

fn get_input() -> Option<(usize, usize)> {
    if is_mouse_button_pressed(MouseButton::Left) {
        let x = mouse_position().0 as usize / (screen_width() / 3.0) as usize;
        let y = mouse_position().1 as usize / (screen_height() / 3.0) as usize;
        Some((x, y))
    } else {
        None
    }
}
