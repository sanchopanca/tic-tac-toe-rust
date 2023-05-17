mod colors;
use crate::game::{BoardSpace, Game, GameState, Player};
use crate::App;
use colors::*;
use macroquad::{prelude::*, ui::root_ui};

const GOLDEN_RATIO: f32 = 1.618;

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
    let popup_size = Vec2::new(
        screen_width() / GOLDEN_RATIO,
        screen_height() / GOLDEN_RATIO / GOLDEN_RATIO,
    );
    let popup_position = Vec2::new(
        (screen_width() - popup_size.x) / 2.0,
        (screen_height() - popup_size.y) / 2.0,
    );

    let title_position = Vec2::new(popup_size.x / 2.0 - 15.0, 0.0);
    let text_position = Vec2::new(popup_size.x / 2.0, 50.0);
    let single_button_position = Vec2::new(popup_size.x / 2.0 - 15.0, 100.0);
    let left_button_position = Vec2::new(popup_size.x / 2.0 - 100.0, 100.0);
    let right_button_position = Vec2::new(popup_size.x / 2.0 + 100.0 - 30.0, 100.0);
    match &app.ui_phase.clone() {
        UIPhase::NewGame => {
            root_ui().window(1, popup_position, popup_size, |ui| {
                ui.label(title_position, "New Game");
                if ui.button(left_button_position, "Play as X") {
                    app.human_player = Player::X;
                    app.ui_phase = UIPhase::Playing;
                }
                if ui.button(right_button_position, "Play as O") {
                    app.human_player = Player::O;
                    app.ui_phase = UIPhase::Playing;
                }
                super::init_random();
            });
        }
        UIPhase::GameOver(state) => {
            root_ui().window(1, popup_position, popup_size, |ui| {
                ui.label(title_position, "Game Over");
                match state {
                    GameState::XWon => ui.label(text_position, "X won"),
                    GameState::OWon => ui.label(text_position, "O won"),
                    GameState::Tie => ui.label(text_position, "Tie"),
                    _ => (),
                }
                if ui.button(single_button_position, "New Game") {
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
        draw_line(
            0.0,
            i as f32 * screen_height() / 3.0,
            screen_width(),
            i as f32 * screen_height() / 3.0,
            10.0,
            BARS_COLOR,
        );
        // vertical lines
        draw_line(
            i as f32 * screen_width() / 3.0,
            0.0,
            i as f32 * screen_width() / 3.0,
            screen_height(),
            10.0,
            BARS_COLOR,
        );
    }
}

fn draw_board(game: &Game) {
    for i in 0..3 {
        for j in 0..3 {
            match game.get_board()[i][j] {
                BoardSpace::Occupied(Player::X) => draw_x(i, j),
                BoardSpace::Occupied(Player::O) => draw_o(i, j),
                BoardSpace::Empty => (),
            }
        }
    }
}

fn padding() -> f32 {
    let size = screen_width() / 3.0;
    size / GOLDEN_RATIO / 4.0
}

fn thikness() -> f32 {
    let size = screen_width() / 3.0;
    size / GOLDEN_RATIO / 4.0
}

fn draw_x(x: usize, y: usize) {
    let top_left_x = x as f32 * screen_width() / 3.0;
    let top_left_y = y as f32 * screen_height() / 3.0;
    let padding = padding() + thikness() / (2.0 * 2.0_f32.sqrt());
    draw_line(
        top_left_x + padding,
        top_left_y + padding,
        top_left_x + screen_width() / 3.0 - padding,
        top_left_y + screen_height() / 3.0 - padding,
        thikness(),
        MARKS_COLOR,
    );
    draw_line(
        top_left_x + screen_width() / 3.0 - padding,
        top_left_y + padding,
        top_left_x + padding,
        top_left_y + screen_height() / 3.0 - padding,
        thikness(),
        MARKS_COLOR,
    );
}

fn draw_o(x: usize, y: usize) {
    let top_left_x = x as f32 * screen_width() / 3.0;
    let top_left_y = y as f32 * screen_height() / 3.0;
    let half_space_width = screen_width() / 6.0;
    let half_space_height = screen_height() / 6.0;
    draw_poly(
        top_left_x + half_space_width,
        top_left_y + half_space_height,
        40,
        half_space_width - padding(),
        0.0,
        MARKS_COLOR,
    );
    draw_poly(
        top_left_x + half_space_width,
        top_left_y + half_space_height,
        40,
        half_space_width - padding() - thikness(),
        0.0,
        BACKGROUND_COLOR,
    );
}
