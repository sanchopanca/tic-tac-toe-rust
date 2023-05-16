mod colors;
use crate::game::{BoardSpace, Game, GameState, Player};
use crate::App;
use colors::*;
use macroquad::{prelude::*, ui::root_ui};

#[derive(Copy, Clone, PartialEq)]
pub enum UIPhase {
    NewGame,
    Playing,
    GameOver(GameState),
}

pub fn draw(app: &App) {
    clear_background(BACKGROUND_COLOR);
    draw_bars();
    draw_board(&app.game);
}

pub fn draw_ui(app: &mut App) {
    match &app.ui_phase.clone() {
        UIPhase::NewGame => {
            root_ui().window(1, Vec2::new(1., 1.), Vec2::new(100., 100.), |ui| {
                ui.label(None, "New Game");
                if ui.button(None, "Play as X") {
                    app.human_player = Player::X;
                    app.ui_phase = UIPhase::Playing;
                }
                if ui.button(None, "Play as O") {
                    app.human_player = Player::O;
                    app.ui_phase = UIPhase::Playing;
                }
            });
        }
        UIPhase::GameOver(state) => {
            root_ui().window(1, Vec2::new(1., 1.), Vec2::new(100., 100.), |ui| {
                ui.label(None, "Game Over");
                match state {
                    GameState::XWon => ui.label(None, "X won"),
                    GameState::OWon => ui.label(None, "O won"),
                    GameState::Tie => ui.label(None, "Tie"),
                    _ => (),
                }
                if ui.button(None, "New Game") {
                    app.ui_phase = UIPhase::NewGame;
                    app.game = Game::new();
                }
            });
        }
        _ => (),
    }
}

fn draw_bars() {
    for i in 1..3 {
        // horizontal lines
        draw_rectangle(
            0.0,
            i as f32 * screen_height() / 3.0,
            screen_width(),
            10.0,
            BARS_COLOR,
        );
        // vertical lines
        draw_rectangle(
            i as f32 * screen_width() / 3.0,
            0.0,
            10.0,
            screen_height(),
            BARS_COLOR,
        );
    }
}

fn draw_board(game: &Game) {
    for i in 0..3 {
        for j in 0..3 {
            match game.get_board()[i][j] {
                BoardSpace::Occupied(Player::X) => draw_x(
                    i as f32 * screen_width() / 3.0,
                    j as f32 * screen_height() / 3.0,
                ),
                BoardSpace::Occupied(Player::O) => draw_o(
                    i as f32 * screen_width() / 3.0,
                    j as f32 * screen_height() / 3.0,
                ),
                BoardSpace::Empty => (),
            }
        }
    }
}

fn draw_x(x: f32, y: f32) {
    draw_line(
        x,
        y,
        x + screen_width() / 3.0,
        y + screen_height() / 3.0,
        15.0,
        MARKS_COLOR,
    );
    draw_line(
        x + screen_width() / 3.0,
        y,
        x,
        y + screen_height() / 3.0,
        15.0,
        MARKS_COLOR,
    );
}

fn draw_o(x: f32, y: f32) {
    draw_poly(
        x + screen_width() / 6.0,
        y + screen_height() / 6.0,
        40,
        screen_width() / 6.0,
        0.0,
        MARKS_COLOR,
    );
    draw_poly(
        x + screen_width() / 6.0,
        y + screen_height() / 6.0,
        40,
        screen_width() / 6.0 - 10.0,
        0.0,
        BACKGROUND_COLOR,
    );
}
