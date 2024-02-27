use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use game::Game;
use game::Piece;

use super::{despawn_screen,Mode, game, GameState, Player, TEXT_COLOR};
use bevy::prelude::Sprite;

use std::process::exit;

// This plugin will contain the game. In this case, it's just be a screen that will
// display the current settings for 5 seconds before returning to the menu
pub fn gameUI_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Game), gameUI_setup)
        .add_systems(Update, gameUI.run_if(in_state(GameState::Game)))
        .add_systems(Update, mouse_click_system.run_if(in_state(GameState::Game)))
        .add_systems(OnExit(GameState::Game), despawn_screen::<OnGameScreen>);
}

// Tag component used to tag entities added on the game screen
#[derive(Component)]
struct OnGameScreen;

/// An identifier for the board background's entity.
pub struct Board;

/// An identifier for the empty-tiles' entities.
pub struct EmptyTile;

#[derive(Resource, Deref, DerefMut)]
struct GameTimer(Timer);

#[derive(Copy, Clone)]
struct Position {
    pub row: usize,
    pub col: usize,
}

impl Position {
    /// Calculates the index of the position on a board
    /// represented by a 1D array.
    fn index(&self) -> usize {
        self.row * 4 + self.col
    }

    /// Transforms a position into a world point according to the board's size.
    fn to_vec3(self) -> Vec3 {
        // Offset from the bottom left point of the board.
        let board = 500.0 + (20.0 * 10.0);
        let tile_size = 500.0 /19.0;
        let tile_spacing = 10.0;

        let offset = Vec3::new(
            -(board - tile_size) / 2.0 + tile_spacing,
            -(board - tile_size) / 2.0 + tile_spacing,
            0.0,
        );

        Vec3::new(
            (tile_size + tile_spacing) * self.col as f32,
            (tile_size + tile_spacing) * self.row as f32,
            0.0,
        ) + offset
    }

    fn to_backend(self) -> (usize, usize) {
        (18 - self.row, self.col)
    }

}

fn gameUI_setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mode: Res<Mode>,
    mut game: ResMut<Game>,
) {
    let board = 500.0 + (20.0 * 10.0);
    let tile_size = 500.0 /19.0;
    let tile_spacing = 10.0;

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(board, board)),
                color: Color::rgb_u8(80, 80, 80).into(),
                ..default()
            },
            ..Default::default()
        });

    // Creating a grid of empty tiles.
    for row in 0..19 {
        for col in 0..19 {
            let position = Position { row, col };

            commands
                .spawn(SpriteBundle {
                    //material: materials.add(Color::rgba_u8(238, 228, 218, 90).into()),
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(tile_size, tile_size)),
                        color: Color::rgba_u8(238, 228, 218, 90).into(),
                        ..default()
                    },
                    transform: Transform::from_translation(position.to_vec3()),
                    ..Default::default()
                });
        }
    }
    if *mode == Mode::IA {
        let position = Position { row: 9, col: 9};
        let tile_size = 500.0 /19.0;
        game.start_ia();
        commands
        .spawn(SpriteBundle {
            //material: materials.add(Color::rgba_u8(238, 228, 218, 90).into()),
            sprite: Sprite {
                custom_size: Some(Vec2::new(tile_size, tile_size)),
                color: Color::rgba_u8(238, 50, 50, 250).into(),
                ..default()
            },
            transform: Transform::from_translation(position.to_vec3()),
            ..Default::default()
        });
    }
    // Spawn a 5 seconds timer to trigger going back to the menu

    
}

// Tick the timer, and change state when finished
fn gameUI(
    time: Res<Time>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    //if timer.tick(time.delta()).finished() {
    //    game_state.set(GameState::Menu);
    //}
}

fn mouse_click_system(mut commands: Commands, mouse_button_input: Res<ButtonInput<MouseButton>>,  windows: Query<&Window>, mut player: ResMut<Player>, mut game : ResMut<Game> , mode : Res<Mode>) {

    let center = Vec2::new(600.0, 400.0);
    let board = 500.0 + (20.0 * 10.0);
    let tile_size = 500.0 /19.0;
    let tile_spacing = 10.0;

    


    if mouse_button_input.just_pressed(MouseButton::Left) {
        let mouse: Vec2 = windows.single().cursor_position().unwrap() - Vec2::new(260.0, 60.0);
        if mouse.x < 0.0 || mouse.x > 680.0 || mouse.y < 0.0 || mouse.y > 680.0 {
            return;
        }
        let row = (19.0 - mouse.y / (tile_size + 10.0)) as usize;
        let col = ( mouse.x / (tile_size + 10.0)) as usize;
    
        let position = Position { row, col};

        info!("{}", windows.single().cursor_position().unwrap());
        info!("row: {}, col: {}, pl: {:?}", row, col, *player);
        
        if *mode == Mode::IA {
            let p_back = position.clone().to_backend();
            info!("click on coordinates: {} {}", p_back.0, p_back.1);
            if !game.place(p_back.0, p_back.1, Piece::Player2) {
                info!("Invalid move");
                return;
            }
            commands
                .spawn(SpriteBundle {
                    //material: materials.add(Color::rgba_u8(238, 228, 218, 90).into()),
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(tile_size, tile_size)),
                        color: Color::rgba_u8(50, 50, 218, 255).into(),
                        ..default()
                    },
                    transform: Transform::from_translation(position.to_vec3()),
                    ..Default::default()
            });
            let ia_move = game.place_ia();
            let position_ia = Position { row: 18 - ia_move.0 , col: ia_move.1};
            commands
            .spawn(SpriteBundle {
                //material: materials.add(Color::rgba_u8(238, 228, 218, 90).into()),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(tile_size, tile_size)),
                    color: Color::rgba_u8(238, 50, 50, 250).into(),
                    ..default()
                },
                transform: Transform::from_translation(position_ia.to_vec3()),
                ..Default::default()
            });

        }

        // if *player == Player::P1 {
        //     match *mode {
        //         Mode::Normal => {
        //             let p_back = position.clone().to_backend();
        //             if !game.place(p_back.0, p_back.1, Piece::Player1) {
        //                 info!("Invalid move");
        //                 return;
        //             }
        //             commands
        //                 .spawn(SpriteBundle {
        //                     //material: materials.add(Color::rgba_u8(238, 228, 218, 90).into()),
        //                     sprite: Sprite {
        //                         custom_size: Some(Vec2::new(tile_size, tile_size)),
        //                         color: Color::rgba_u8(238, 50, 50, 250).into(),
        //                         ..default()
        //                     },
        //                     transform: Transform::from_translation(position.to_vec3()),
        //                     ..Default::default()
        //                 });
        //             *player = Player::P2;
        //         },
        //         Mode::IA => {
        //         let ia_move = game.place_ia();
        //         let position_ia = Position { row: ia_move.0 , col: ia_move.1};
        //         commands
        //         .spawn(SpriteBundle {
        //             //material: materials.add(Color::rgba_u8(238, 228, 218, 90).into()),
        //             sprite: Sprite {
        //                 custom_size: Some(Vec2::new(tile_size, tile_size)),
        //                 color: Color::rgba_u8(238, 50, 50, 250).into(),
        //                 ..default()
        //             },
        //             transform: Transform::from_translation(position_ia.to_vec3()),
        //             ..Default::default()
        //         });
        //         *player = Player::P2;
        //         },
        //     }
        // }
        // else{
        //     let p_back = position.clone().to_backend();
        //     if !game.place(p_back.0, p_back.1, Piece::Player2) {
        //         info!("Invalid move");
        //         return;
        //     }
        //     commands
        //         .spawn(SpriteBundle {
        //             //material: materials.add(Color::rgba_u8(238, 228, 218, 90).into()),
        //             sprite: Sprite {
        //                 custom_size: Some(Vec2::new(tile_size, tile_size)),
        //                 color: Color::rgba_u8(50, 50, 218, 255).into(),
        //                 ..default()
        //             },
        //             transform: Transform::from_translation(position.to_vec3()),
        //             ..Default::default()
        //         });
        //     *player = Player::P1;
        // }
        if game.check_win() {
            info!("Winner");
            exit(0);
        }
            
    }

    if mouse_button_input.just_released(MouseButton::Left) {
        info!("left mouse just released");
    }
}