// bevy namespace uses
use bevy::prelude::*;

// Window constants
pub const ASPECT_RATIO: f32 = 16.0/9.0;
pub const WINDOW_HEIGHT: f32 =  720.0;
pub const WINDOW_WIDTH: f32 = WINDOW_HEIGHT*ASPECT_RATIO;
pub const BACKGROUND_COLOUR: Color = Color::rgb(0.2, 0.2, 0.2);

// Other constants
pub const BOARD_SIZE: f32 = 500.0;
pub const SQUARE_SIZE: f32 = BOARD_SIZE / 8.0;
pub const BOARD_CENTRE: Vec2 = Vec2::new(0.0, 0.0);
pub const BOARD_SQUARE_WHITE_COLOUR: Color = Color::rgb(0.93, 0.93, 0.82);
pub const BOARD_SQUARE_BLACK_COLOUR: Color = Color::rgb(0.46, 0.59, 0.34);
pub const BOARD_SQUARE_WHITE_SELECT_COLOUR: Color = Color::rgb(0.96, 0.96, 0.41);
pub const BOARD_SQUARE_BLACK_SELECT_COLOUR: Color = Color::rgb(0.73, 0.79, 0.16);
pub const STARTING_FEN_STRING: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";

// Enums
#[derive(Debug)]
pub enum Team {
    WHITE,
    BLACK
}
#[derive(Debug)]
pub enum Kind {
    PAWN,
    KNIGHT,
    BISHOP,
    ROOK,
    QUEEN,
    KING
}


////////////////////////////////////////////////////////////////
// Components
////////////////////////////////////////////////////////////////
#[derive(Component)]
pub struct Square {
    file: u8,
    rank: u8
}

#[derive(Component, Debug)]
pub struct Piece {
    team: Team,
    kind: Kind,
    coords: (u8, u8)
}

////////////////////////////////////////////////////////////////
// App
////////////////////////////////////////////////////////////////
fn main() {
    App::new()
    .insert_resource(ClearColor(BACKGROUND_COLOUR))
    .add_plugins(DefaultPlugins.set(
        WindowPlugin { 
            primary_window: Some(Window {
                title: "Bevy Chess Implementation".to_string(),
                resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }
    ))
    .add_systems(Startup, (
        spawn_camera, 
        spawn_board,
        spawn_pieces
    ))
    .add_systems(Update, select_piece)
    .run();
}

////////////////////////////////////////////////////////////////
// Systems
////////////////////////////////////////////////////////////////

// Camera systems
fn spawn_camera(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle::default()).insert(Name::new("Camera"));
}

fn spawn_board(
    mut commands: Commands,
) {
    for rank in 1..9 {
        for file in 1..9 {
            let square_colour: Color;
            if (rank + file) % 2 == 0 {
                square_colour = BOARD_SQUARE_WHITE_COLOUR;
            } else {
                square_colour = BOARD_SQUARE_BLACK_COLOUR;
            }
            commands.spawn(SpriteBundle {
                sprite: Sprite {
                    color: square_colour,
                    custom_size: Some(Vec2::new(SQUARE_SIZE, SQUARE_SIZE)),
                    ..default()
                },
                transform: Transform::from_xyz(
                    BOARD_CENTRE.x - (4.5 - file as f32) * SQUARE_SIZE, 
                    BOARD_CENTRE.y - (4.5 - rank as f32) * SQUARE_SIZE, 
                    0.0
                ),
                ..default()
            })
            .insert(Name::new(format!("{}{}", file_to_char(file), rank_to_char(rank))))
            .insert(Square {rank, file});
        }
    }
}

fn spawn_pieces(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let pieces: Vec<Piece> = fen_string_to_piece_vec(STARTING_FEN_STRING).unwrap();
    for piece in pieces {
        let world_position = board_to_world(piece.coords);
        let piece_name = get_piece_name(&piece.team, &piece.kind);
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                ..default()
            },
            transform: Transform::from_xyz(
                world_position.x,
                world_position.y,
                1.0
            ),
            texture: asset_server.load(format!("pieces/{}.png", piece_name)),
            ..default()
        })
        .insert(piece);
    }
}

fn select_piece(
    buttons: Res<Input<MouseButton>>,
    window: Query<&Window>,
    camera: Query<(&Camera, &GlobalTransform)>,
    squares: Query<(&Square, &mut Sprite)>,
    pieces: Query<&Piece>
) {
    if !buttons.just_pressed(MouseButton::Left) {return}

    let (camera, camera_transform) = camera.single();
    let position = window.single().cursor_position().and_then(|cursor|camera.viewport_to_world(camera_transform, cursor)).map(|ray| ray.origin.truncate());
    if position.is_none() {return}
    
    let position = position.unwrap();
    if !is_position_on_board(&position) {clear_square_selections(squares); return}

    let selected_coords: (u8, u8) = world_to_board(position);
    for piece in pieces.iter() {
        if piece.coords == selected_coords {
            colour_selected_square(squares, selected_coords);
            return;
        }
    }

    clear_square_selections(squares);
}

////////////////////////////////////////////////////////////////
// Helper functions
////////////////////////////////////////////////////////////////

fn file_to_char(file: u8) -> char {
    match file {
        1 => 'a',
        2 => 'b',
        3 => 'c',
        4 => 'd',
        5 => 'e',
        6 => 'f',
        7 => 'g',
        8 => 'h',
        _ => panic!()
    }
}

fn rank_to_char(rank: u8) -> char {
    match rank {
        1 => '1',
        2 => '2',
        3 => '3',
        4 => '4',
        5 => '5',
        6 => '6',
        7 => '7',
        8 => '8',
        _ => panic!()
    }
}

fn world_to_board(position: Vec2) -> (u8, u8) {
    let file = ((position.x - BOARD_CENTRE.x + 0.5*BOARD_SIZE) / SQUARE_SIZE).ceil() as u8;
    let rank = ((position.y - BOARD_CENTRE.y + 0.5*BOARD_SIZE) / SQUARE_SIZE).ceil() as u8;
    return (file, rank);
}

fn board_to_world(coords: (u8, u8)) -> Vec2 {
    let (file, rank) = coords;
    let x: f32 = BOARD_CENTRE.x - (4.5 - file as f32) * SQUARE_SIZE;
    let y: f32 = BOARD_CENTRE.y - (4.5 - rank as f32) * SQUARE_SIZE;
    return Vec2::new(x, y);
}

fn is_position_on_board(&position: &Vec2) -> bool {
    position.x >= BOARD_CENTRE.x - 0.5*BOARD_SIZE &&
    position.x <= BOARD_CENTRE.x + 0.5*BOARD_SIZE &&
    position.y >= BOARD_CENTRE.y - 0.5*BOARD_SIZE &&
    position.y <= BOARD_CENTRE.y + 0.5*BOARD_SIZE
}

fn get_piece_name(team: &Team, kind: &Kind) -> String {
    let team_str = match team {
        Team::WHITE => "white", 
        Team::BLACK => "black" 
    };
    
    let kind_str = match kind {
        Kind::PAWN    => "pawn",
        Kind::KNIGHT  => "knight",
        Kind::BISHOP  => "bishop",
        Kind::ROOK    => "rook",
        Kind::QUEEN   => "queen",
        Kind::KING    => "king"
    };

    return format!("{}_{}", team_str, kind_str);
}

fn fen_string_to_piece_vec(fen: &str) -> Result<Vec<Piece>, String> {
    let mut piece_vec: Vec<Piece> = Vec::new();
    let mut current_rank: u8 = 8;  // TODO: Improve this with assertions they are in the range [1:8]
    let mut current_file: u8 = 1;  // TODO: Improve this with assertions they are in the range [1:8]

    for char in fen.chars() {
        match char {
            'P'       => {piece_vec.push(Piece {team: Team::WHITE, kind: Kind::PAWN,   coords: (current_file, current_rank)}); current_file += 1},
            'N'       => {piece_vec.push(Piece {team: Team::WHITE, kind: Kind::KNIGHT, coords: (current_file, current_rank)}); current_file += 1},
            'B'       => {piece_vec.push(Piece {team: Team::WHITE, kind: Kind::BISHOP, coords: (current_file, current_rank)}); current_file += 1},
            'R'       => {piece_vec.push(Piece {team: Team::WHITE, kind: Kind::ROOK,   coords: (current_file, current_rank)}); current_file += 1},
            'Q'       => {piece_vec.push(Piece {team: Team::WHITE, kind: Kind::QUEEN,  coords: (current_file, current_rank)}); current_file += 1},
            'K'       => {piece_vec.push(Piece {team: Team::WHITE, kind: Kind::KING,   coords: (current_file, current_rank)}); current_file += 1},
            'p'       => {piece_vec.push(Piece {team: Team::BLACK, kind: Kind::PAWN,   coords: (current_file, current_rank)}); current_file += 1},
            'n'       => {piece_vec.push(Piece {team: Team::BLACK, kind: Kind::KNIGHT, coords: (current_file, current_rank)}); current_file += 1},
            'b'       => {piece_vec.push(Piece {team: Team::BLACK, kind: Kind::BISHOP, coords: (current_file, current_rank)}); current_file += 1},
            'r'       => {piece_vec.push(Piece {team: Team::BLACK, kind: Kind::ROOK,   coords: (current_file, current_rank)}); current_file += 1},
            'q'       => {piece_vec.push(Piece {team: Team::BLACK, kind: Kind::QUEEN,  coords: (current_file, current_rank)}); current_file += 1},
            'k'       => {piece_vec.push(Piece {team: Team::BLACK, kind: Kind::KING,   coords: (current_file, current_rank)}); current_file += 1},
            '/'       => {current_file = 1; current_rank -= 1},
            '1'..='8' => {current_file += char.to_digit(10).unwrap() as u8},
            _         => return Result::Err(format!("Non-fen char '{}' found in fen string", char))  // TODO: Improve this with proper error type
        }
    }

    return Result::Ok(piece_vec);
}

fn clear_square_selections(mut squares: Query<(&Square, &mut Sprite)>) {
    for (square, mut sprite) in squares.iter_mut() {
        if (square.rank + square.file) % 2 == 0 {
            sprite.color = BOARD_SQUARE_WHITE_COLOUR;
        } else {
            sprite.color = BOARD_SQUARE_BLACK_COLOUR;
        }
    }
}

fn colour_selected_square(mut squares: Query<(&Square, &mut Sprite)>, selected_coords: (u8, u8)) {
    for (square, mut sprite) in squares.iter_mut() {
        if (square.rank + square.file) % 2 == 0 {
            if (square.file, square.rank) == selected_coords {
                sprite.color = BOARD_SQUARE_WHITE_SELECT_COLOUR;
            } else {
                sprite.color = BOARD_SQUARE_WHITE_COLOUR;
            }
        } else {
            if (square.file, square.rank) == selected_coords {
                sprite.color = BOARD_SQUARE_BLACK_SELECT_COLOUR;
            } else {
                sprite.color = BOARD_SQUARE_BLACK_COLOUR;
            }
        }
    }
}