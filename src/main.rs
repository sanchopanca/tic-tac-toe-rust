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

#[derive(Copy, Clone)]
struct Game {
    board: [[BoardSpace; 3]; 3],
    turn: Player,
    state: GameState,
}

pub struct EmptySpacesIter<'a> {
    game: &'a Game,
    x: usize,
    y: usize,
}

impl<'a> Iterator for EmptySpacesIter<'a> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        while self.x < self.game.board.len() {
            while self.y < self.game.board[self.x].len() {
                let space = &self.game.board[self.x][self.y];
                self.y += 1;
                if *space == BoardSpace::Empty {
                    return Some((self.x, self.y - 1));
                }
            }
            self.x += 1;
            self.y = 0;
        }
        None
    }
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
        // println!("{:?}", self.state);
    }

    pub fn empty_spaces(&self) -> EmptySpacesIter {
        EmptySpacesIter {
            game: self,
            x: 0,
            y: 0,
        }
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

fn ai_move(game: &Game) -> Option<(usize, usize)> {
    if game.state != GameState::Playing {
        None
    } else {
        let ((x, y), _) = minimax(game);
        Some((x, y))
    }
}

fn minimax(game: &Game) -> ((usize, usize), i32) {
    if game.state != GameState::Playing {
        return ((0, 0), evaluate_position(game));
    }

    let mut best_move = (0, 0);
    let mut best_score;
    match game.turn {
        Player::X => {
            best_score = -10;
            for (x, y) in game.empty_spaces() {
                let mut clonned_game = game.clone();
                clonned_game.play(x, y);
                let (_, score) = minimax(&clonned_game);
                if score > best_score {
                    best_score = score;
                    best_move = (x, y);
                }
            }
        }
        Player::O => {
            best_score = 10;
            for (x, y) in game.empty_spaces() {
                let mut clonned_game = game.clone();
                clonned_game.play(x, y);
                let (_, score) = minimax(&clonned_game);
                if score < best_score {
                    best_score = score;
                    best_move = (x, y);
                }
            }
        }
    }

    (best_move, best_score)
}

fn evaluate_position(game: &Game) -> i32 {
    match game.state {
        GameState::XWon => 1,
        GameState::OWon => -1,
        GameState::Tie => 0,
        GameState::Playing => panic!("shouldn't be called for board that still in game"),
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
    if app.game.turn == Player::O {
        if let Some((x, y)) = ai_move(&app.game) {
            app.game.play(x, y);
            return;
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use BoardSpace::*;
    use Player::*;

    #[test]
    fn test_minimax_last_step() {
        // tie
        let board = [
            [Occupied(X), Occupied(O), Occupied(O)],
            [Occupied(O), Occupied(X), Occupied(X)],
            [Occupied(X), Empty, Occupied(O)],
        ];
        let game = Game {
            board,
            state: GameState::Playing,
            turn: X,
        };
        let (_, score) = minimax(&game);
        assert_eq!(score, 0);

        // x win
        let board = [
            [Occupied(X), Occupied(O), Occupied(O)],
            [Empty, Occupied(X), Occupied(X)],
            [Occupied(X), Occupied(O), Occupied(O)],
        ];
        let game = Game {
            board,
            state: GameState::Playing,
            turn: X,
        };
        let (_, score) = minimax(&game);
        assert_eq!(score, 1);
    }

    #[test]
    fn test_minimax_depth2() {
        // x win
        let board = [
            [Occupied(X), Occupied(O), Occupied(O)],
            [Empty, Occupied(X), Occupied(X)],
            [Occupied(X), Occupied(O), Empty],
        ];
        let game = Game {
            board,
            state: GameState::Playing,
            turn: O,
        };
        let (_, score) = minimax(&game);
        assert_eq!(score, 1);

        // tie
        let board = [
            [Occupied(X), Occupied(O), Occupied(O)],
            [Empty, Occupied(X), Occupied(X)],
            [Occupied(X), Empty, Occupied(O)],
        ];
        let game = Game {
            board,
            state: GameState::Playing,
            turn: O,
        };
        let (_, score) = minimax(&game);
        assert_eq!(score, 0);

        // o win
        let board = [
            [Empty, Occupied(O), Occupied(O)],
            [Empty, Occupied(X), Occupied(X)],
            [Occupied(X), Occupied(X), Occupied(O)],
        ];
        let game = Game {
            board,
            state: GameState::Playing,
            turn: O,
        };
        let (_, score) = minimax(&game);
        assert_eq!(score, -1);
    }

    #[test]
    fn test_minimax_depth3() {
        // x win
        let board = [
            [Empty, Occupied(O), Occupied(O)],
            [Empty, Occupied(X), Occupied(X)],
            [Occupied(X), Occupied(O), Empty],
        ];
        let game = Game {
            board,
            state: GameState::Playing,
            turn: X,
        };
        let (_, score) = minimax(&game);
        assert_eq!(score, 1);

        // tie
        let board = [
            [Empty, Occupied(O), Occupied(O)],
            [Empty, Empty, Occupied(X)],
            [Occupied(X), Occupied(X), Occupied(O)],
        ];
        let game = Game {
            board,
            state: GameState::Playing,
            turn: X,
        };
        let (_, score) = minimax(&game);
        assert_eq!(score, 0);

        // o win
        let board = [
            [Occupied(O), Empty, Occupied(O)],
            [Empty, Empty, Occupied(X)],
            [Occupied(O), Occupied(X), Occupied(X)],
        ];
        let game = Game {
            board,
            state: GameState::Playing,
            turn: X,
        };
        let (_, score) = minimax(&game);
        assert_eq!(score, -1);
    }

    #[test]
    // #[ignore]
    fn test_minimax_the_game_is_a_tie() {
        let board = [
            [Empty, Empty, Empty],
            [Empty, Empty, Empty],
            [Empty, Empty, Empty],
        ];
        let game = Game {
            board,
            state: GameState::Playing,
            turn: X,
        };
        let (_, score) = minimax(&game);
        assert_eq!(score, 0);
    }

    #[test]
    fn test_iterator() {
        let board = [
            [Occupied(X), Occupied(O), Empty],
            [Occupied(O), Occupied(X), Occupied(X)],
            [Empty, Empty, Occupied(O)],
        ];
        let game = Game {
            board,
            state: GameState::Playing,
            turn: X,
        };
        let empty = game.empty_spaces().count();
        assert_eq!(empty, 3);
    }
}
