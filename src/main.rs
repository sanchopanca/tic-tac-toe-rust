use macroquad::{prelude::*, ui::root_ui};

#[allow(dead_code)]
const YANKEES_BLUE: Color = color_u8!(13, 43, 69, 255);
#[allow(dead_code)]
const JAPANESE_INDIGO: Color = color_u8!(32, 60, 86, 255);
const INDEPENDENCE: Color = color_u8!(84, 78, 104, 255);
#[allow(dead_code)]
const ANTIQUE_FUCHSIA: Color = color_u8!(141, 105, 122, 255);
const RAW_SIENNA: Color = color_u8!(208, 129, 89, 255);
#[allow(dead_code)]
const RAJAH: Color = color_u8!(255, 170, 94, 255);
const PAPAYA_WHIP: Color = color_u8!(255, 236, 214, 255);

const BACKGROUND_COLOR: Color = PAPAYA_WHIP;
const BARS_COLOR: Color = INDEPENDENCE;
const MARKS_COLOR: Color = RAW_SIENNA;

fn conf() -> Conf {
    Conf {
        window_title: "Tic Tac Toe".to_owned(),
        window_width: 800,
        window_height: 800,
        ..Default::default()
    }
}

#[derive(Copy, Clone, PartialEq)]
enum Player {
    X,
    O,
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum GameState {
    Playing,
    XWon,
    OWon,
    Tie,
}

#[derive(Copy, Clone, PartialEq)]
enum BoardSpace {
    Occupied(Player),
    Empty,
}

struct Game {
    board: [[BoardSpace; 3]; 3],
    turn: Player,
    state: GameState,
}

#[derive(Copy, Clone, PartialEq)]
enum UIPhase {
    NewGame,
    Playing,
    GameOver(GameState),
}

struct App {
    game: Game,
    ui_phase: UIPhase,
}

impl Game {
    fn new() -> Game {
        Game {
            board: [[BoardSpace::Empty; 3]; 3],
            turn: Player::X,
            state: GameState::Playing,
        }
    }

    fn play(&mut self, x: usize, y: usize) {
        if let BoardSpace::Empty = self.board[x][y] {
            self.board[x][y] = BoardSpace::Occupied(self.turn);
            self.turn = match self.turn {
                Player::X => Player::O,
                Player::O => Player::X,
            }
        }
        self.state = self.check_win();
        println!("{:?}", self.state);
    }

    fn check_win(&self) -> GameState {
        let mut x_won = false;
        let mut o_won = false;
        let mut tie = true;
        for row in self.board {
            if row.iter().all(|&x| x == BoardSpace::Occupied(Player::X)) {
                x_won = true;
            }
            if row.iter().all(|&x| x == BoardSpace::Occupied(Player::O)) {
                o_won = true;
            }
            if row.iter().any(|&x| x == BoardSpace::Empty) {
                tie = false;
            }
        }

        for i in 0..self.board[0].len() {
            if self.board[0][i] == self.board[1][i]
                && self.board[1][i] == self.board[2][i]
                && self.board[0][i] != BoardSpace::Empty
            {
                match self.board[0][i] {
                    BoardSpace::Occupied(Player::X) => x_won = true,
                    BoardSpace::Occupied(Player::O) => o_won = true,
                    _ => (),
                }
            }
        }

        // hardcode, sorry about that
        if self.board[0][0] == self.board[1][1]
            && self.board[1][1] == self.board[2][2]
            && self.board[0][0] != BoardSpace::Empty
        {
            match self.board[0][0] {
                BoardSpace::Occupied(Player::X) => x_won = true,
                BoardSpace::Occupied(Player::O) => o_won = true,
                _ => (),
            }
        }

        if self.board[0][2] == self.board[1][1]
            && self.board[1][1] == self.board[2][0]
            && self.board[0][2] != BoardSpace::Empty
        {
            match self.board[0][2] {
                BoardSpace::Occupied(Player::X) => x_won = true,
                BoardSpace::Occupied(Player::O) => o_won = true,
                _ => (),
            }
        }

        if x_won {
            GameState::XWon
        } else if o_won {
            GameState::OWon
        } else if tie {
            GameState::Tie
        } else {
            GameState::Playing
        }
    }
}

#[macroquad::main(conf)]
async fn main() {
    let mut app = App {
        game: Game::new(),
        ui_phase: UIPhase::NewGame,
    };
    loop {
        process(&mut app);
        draw(&app);
        draw_ui(&mut app);
        next_frame().await
    }
}

fn process(app: &mut App) {
    if app.ui_phase != UIPhase::Playing {
        return;
    }
    if app.game.state != GameState::Playing {
        app.ui_phase = UIPhase::GameOver(app.game.state);
        return;
    }
    if let Some((x, y)) = get_input() {
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

fn draw(app: &App) {
    clear_background(BACKGROUND_COLOR);
    draw_bars();
    draw_board(&app.game);
}

fn draw_ui(app: &mut App) {
    match &app.ui_phase.clone() {
        UIPhase::NewGame => {
            root_ui().window(1, Vec2::new(1., 1.), Vec2::new(100., 100.), |ui| {
                ui.label(None, "New Game");
                if ui.button(None, "Start") {
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
            match game.board[i][j] {
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
