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
pub const BOARD_SQUARE_WHITE_COLOUR: Color = Color::rgb(0.9, 0.9, 0.9);
pub const BOARD_SQUARE_BLACK_COLOUR: Color = Color::rgb(0.15, 0.15, 0.15);

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
pub struct Piece{
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
    .add_systems(Startup, spawn_camera)
    .add_systems(Startup, spawn_board)
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
    for rank in 0..8 {
        for file in 0..8 {
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
                    BOARD_CENTRE.x - (3.5 - file as f32) * SQUARE_SIZE, 
                    BOARD_CENTRE.y + (3.5 - rank as f32) * SQUARE_SIZE, 
                    0.0
                ),
                ..default()
            });
        }
    }
}
