use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use game::Game;
use game::Piece;

use super::{despawn_screen,Mode, game, GameState, Player, TEXT_COLOR};
use bevy::prelude::Sprite;
use crate::menu::MenuButtonAction;

use std::process::exit;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);

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
    asset_server: Res<AssetServer>,
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

    commands
        .spawn( TextBundle::from_section(
                "PLAYER ONE",
                TextStyle {
                    font: asset_server.load("fonts/MontserratExtrabold-VGO60.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.1, 0.1, 0.9),
                    ..default()
                },
            )
            .with_style(Style {
                margin: UiRect {
                    top: Val::Px(710.0),
                    left: Val::Px(10.0),
                    bottom: Val::Px(10.0),
                    right: Val::Px(10.0),},
                ..default()
            }),
        );
    commands
        .spawn( TextBundle::from_section(
                "PLAYERTWO",
                TextStyle {
                    font: asset_server.load("fonts/MontserratExtrabold-VGO60.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.1, 0.1),
                    ..default()
                },
            )
            .with_style(Style {
                margin: UiRect {
                    top: Val::Px(50.0),
                    left: Val::Px(960.0),
                    bottom: Val::Px(10.0),
                    right: Val::Px(0.0),},
                position_type: PositionType::Relative,
                ..default()
            }),
        );
        
    let button_style = Style {
        width: Val::Px(250.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_text_style = TextStyle {
        font_size: 40.0,
        color: TEXT_COLOR,
        ..default()
    };
    
    commands
        .spawn((
            ButtonBundle {
                style: button_style,
                background_color: NORMAL_BUTTON.into(),
                ..default()
            },
            MenuButtonAction::BackToSettings,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section("Back", button_text_style));
        });

    // Creating a grid of empty tiles.
    if *mode == Mode::IA {
        game.start_ia();
    }
    print_ui_map(&game, &mut commands, tile_size);
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

fn print_ui_tile(position: Position, tile_size: f32, commands: &mut Commands, color: Color) {
    commands
    .spawn(SpriteBundle {
        //material: materials.add(Color::rgba_u8(238, 228, 218, 90).into()),
        sprite: Sprite {
            custom_size: Some(Vec2::new(tile_size, tile_size)),
            color: color.into(),
            ..default()
        },
        transform: Transform::from_translation(position.to_vec3()),
        ..Default::default()
    });
}

fn print_ui_map(game: &Game, commands: &mut Commands, tile_size: f32) {
    let empty_tile : Color = Color::rgba_u8(238, 228, 218, 250);
    let p1_tile : Color = Color::rgba_u8(238, 50, 50, 250);
    let p2_tile : Color = Color::rgba_u8(50, 50, 218, 255);
    for i in 0..19 {
        for j in 0..19 {
            let position = Position { row: 18 - i, col: j};
            match game.map[i][j] {
                Piece::Empty => print_ui_tile(position, tile_size, commands, empty_tile),
                Piece::Player1 => print_ui_tile(position, tile_size, commands, p1_tile),
                Piece::Player2 => print_ui_tile(position, tile_size, commands, p2_tile)
            }
        }
    }
}

fn mouse_click_system(
    mut commands: Commands, 
    mouse_button_input: Res<ButtonInput<MouseButton>>,  
    windows: Query<&Window>, 
    mut player: ResMut<Player>, 
    mut game : ResMut<Game> , 
    mode : Res<Mode>,
    mut game_state: ResMut<NextState<GameState>>,
    ) {

    let center = Vec2::new(600.0, 400.0);
    let board = 500.0 + (20.0 * 10.0);
    let tile_size = 500.0 /19.0;
    let tile_spacing = 10.0;

    if mouse_button_input.just_pressed(MouseButton::Left) {
        let mouse: Vec2 = windows.single().cursor_position().unwrap() - Vec2::new(260.0, 60.0);
        if mouse.x < 0.0 || mouse.x > 680.0 || mouse.y < 0.0 || mouse.y > 680.0 {
            let abs = windows.single().cursor_position().unwrap();
            if (abs.x > 1100.0 && abs.y > 700.0) {
                game_state.set(GameState::Menu);
            }
            return;
        }
        let row = (19.0 - mouse.y / (tile_size + 10.0)) as usize;
        let col = ( mouse.x / (tile_size + 10.0)) as usize;
    
        let position = Position { row, col};

        info!("{}", windows.single().cursor_position().unwrap());
        info!("row: {}, col: {}, pl: {:?}", row, col, *player);
        
        match *mode {
            Mode::Normal => {
                let p_back = position.clone().to_backend();
                info!("click on coordinates: {} {}", p_back.0, p_back.1);
                if !game.update_game(p_back.0, p_back.1, if *player == Player::P1 {Piece::Player1} else {Piece::Player2}) {
                    info!("Invalid move");
                    return;
                }
                if *player == Player::P1 { *player = Player::P2 } else { *player = Player::P1 };
                print_ui_map(&game, &mut commands, tile_size);
            },
            Mode::IA => {
                let p_back = position.clone().to_backend();
                info!("click on coordinates: {} {}", p_back.0, p_back.1);
                if !game.update_game_ia(p_back.0, p_back.1) {
                    info!("Invalid move");
                    return;
                }
                print_ui_map(&game, &mut commands, tile_size);
            }
        }
        game.print_map();
        if (game.check_win() == (true, Piece::Player1)) || (game.check_win() == (true, Piece::Player2)) {
            println!("Segmentation Fault (core dumped)");
            exit(0);
        }
            
    }

    if mouse_button_input.just_released(MouseButton::Left) {
        info!("left mouse just released");
    }
}