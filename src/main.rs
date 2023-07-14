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

// Enums
pub enum Team {
    WHITE,
    BLACK
}
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

#[derive(Component)]
pub struct Piece {
    team: Team,
    kind: Kind,
    position: u8
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
        spawn_board
    ))
    .add_systems(Update, select_square)
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

fn select_square(
    buttons: Res<Input<MouseButton>>,
    window: Query<&Window>,
    camera: Query<(&Camera, &GlobalTransform)>,
    mut squares: Query<(&Square, &mut Sprite)>
) {
    if buttons.just_pressed(MouseButton::Left) {
        let (camera, camera_transform) = camera.single();
        if let Some(position) = window
            .single()
            .cursor_position()
            .and_then(|cursor| camera
                .viewport_to_world(camera_transform, cursor)
            )
            .map(|ray| ray.origin.truncate()) 
        {
            if position.x >= BOARD_CENTRE.x - 0.5*BOARD_SIZE &&
               position.x <= BOARD_CENTRE.x + 0.5*BOARD_SIZE &&
               position.y >= BOARD_CENTRE.y - 0.5*BOARD_SIZE &&
               position.y <= BOARD_CENTRE.y + 0.5*BOARD_SIZE
            {
                let (file, rank) = world_to_board(position);
                for (square, mut sprite) in squares.iter_mut() {
                    if (square.rank + square.file) % 2 == 0 {
                        if square.file == file && square.rank == rank {
                            sprite.color = BOARD_SQUARE_WHITE_SELECT_COLOUR;
                        } else {
                            sprite.color = BOARD_SQUARE_WHITE_COLOUR;
                        }
                    } else {
                        if square.file == file && square.rank == rank {
                            sprite.color = BOARD_SQUARE_BLACK_SELECT_COLOUR;
                        } else {
                            sprite.color = BOARD_SQUARE_BLACK_COLOUR;
                        }
                    }
                }
            } else {
                for (square, mut sprite) in squares.iter_mut() {
                    if (square.rank + square.file) % 2 == 0 {
                        sprite.color = BOARD_SQUARE_WHITE_COLOUR;
                    } else {
                        sprite.color = BOARD_SQUARE_BLACK_COLOUR;
                    }
                }
            }
        }
    }
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

fn world_to_board(coord: Vec2) -> (u8, u8) {
    let file = ((coord.x - BOARD_CENTRE.x + 0.5*BOARD_SIZE) / SQUARE_SIZE).ceil() as u8;
    let rank = ((coord.y - BOARD_CENTRE.y + 0.5*BOARD_SIZE) / SQUARE_SIZE).ceil() as u8;
    return (file, rank);
}