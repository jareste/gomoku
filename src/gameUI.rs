use bevy::prelude::*;
use bevy::transform::commands;
use bevy_prototype_lyon::prelude::*;
use game::Game;
use game::Piece;
use rand::Rng;

use super::{despawn_screen,Mode, game, GameState, Player, TEXT_COLOR};
use bevy::prelude::Sprite;
use crate::ia;
use crate::menu::MenuButtonAction;
use crate::zfighting;
use crate::IAPosition;

use std::process::exit;
use std::thread::sleep;
use std::time::Instant;

// This plugin will contain the game. In this case, it's just be a screen that will
// display the current settings for 5 seconds before returning to the menu
pub fn gameUI_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Game), gameUI_setup)
        .add_systems(Update, gameUI.run_if(in_state(GameState::Game)))
        .add_systems(Update, mouse_click_system.run_if(in_state(GameState::Game)))
        .add_systems(Update, countdown.run_if(in_state(GameState::Game)))
        .add_systems(Update, button_system.run_if(in_state(GameState::Game)))
        .add_systems(Update, captures.run_if(in_state(GameState::Game)))
        .add_systems(Update, IA_move.run_if(in_state(GameState::Game)))
        .add_systems(Update, game_ended.run_if(in_state(GameState::Game)))
        .add_systems(OnExit(GameState::Game), despawn_screen::<OnGameScreen>)
        .add_systems(OnExit(GameState::Game), despawn_screen::<OnHintScreen>)
        .add_systems(OnExit(GameState::Game), despawn_screen::<TimeText>)
        .add_systems(OnExit(GameState::Game), despawn_screen::<CaptureText>);
}

// Tag component used to tag entities added on the game screen
#[derive(Component)]
struct OnGameScreen;

#[derive(Component)]
struct OnHintScreen;

#[derive(Component)]
struct TimeText;

#[derive(Component)]
struct CaptureText;


#[derive(Resource, Deref, DerefMut)]
    struct GameTimer(Timer);

#[derive(Resource)]
struct PlayerTimes(pub u32, pub u32);


#[derive(Resource, PartialEq, Eq, Clone, Copy, Debug)]
struct Finished(pub bool);

pub struct Board;

/// An identifier for the empty-tiles' entities.
pub struct EmptyTile;

#[derive(Copy, Clone, Debug)]
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
    fn to_vec3(self, zf: f32) -> Vec3 {
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
            zf,
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
    iapos: Res<IAPosition>,
    mut player: ResMut<Player>,
    mut zf: ResMut<zfighting>,
 ) {
    let board = 500.0 + (20.0 * 10.0);
    let tile_size = 500.0 /19.0;
    let tile_spacing = 10.0;

    commands.insert_resource(GameTimer(Timer::from_seconds(0.1, TimerMode::Repeating)));
    commands.insert_resource(PlayerTimes(0, 0));
    commands.insert_resource(Finished(false));

    commands
        .spawn((SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(board, board)),
                color: Color::rgb_u8(80, 80, 80).into(),
                ..default()
            },
            ..Default::default()
        },
        OnGameScreen,),
        );

    commands
        .spawn(( TextBundle::from_section(
                "PLAYER ONE",
                TextStyle {
                    font: asset_server.load("fonts/MontserratExtrabold-VGO60.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.1, 0.1),
                    ..default()
                },
            )
            .with_style(Style {
                margin: UiRect {
                    top: Val::Px(600.0),
                    left: Val::Px(10.0),
                    bottom: Val::Px(10.0),
                    right: Val::Px(10.0),},
                ..default()
            }),
            OnGameScreen,),
        );
    commands
        .spawn(( TextBundle::from_section(
                "PLAYERTWO",
                TextStyle {
                    font: asset_server.load("fonts/MontserratExtrabold-VGO60.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.1, 0.1, 0.9),
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
            OnGameScreen,),
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
        font_size: 25.0,
        color: TEXT_COLOR,
        ..default()
    };
    
    commands
        .spawn((ButtonBundle {
            style: Style {
                width: Val::Px(110.0),
                height: Val::Px(45.0),
                border: UiRect::all(Val::Px(4.0)),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                margin: UiRect {
                    top: Val::Px(705.0),
                    left: Val::Px(1020.0),
                    bottom: Val::Px(10.0),
                    right: Val::Px(0.0),},
                ..default()
            },
            border_color: BorderColor(Color::BLACK),
            background_color: NORMAL_BUTTON.into(),
            ..default()
        },
        OnGameScreen,),
        )           
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section("Quit", button_text_style.clone()));
        });
        
        if (*mode != Mode::IAP2 && *mode != Mode::IAP1P2) {
            commands
            .spawn((ButtonBundle {
                style: Style {
                    width: Val::Px(110.0),
                    height: Val::Px(45.0),
                    border: UiRect::all(Val::Px(0.0)),
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    margin: UiRect {
                        top: Val::Px(160.0),
                        left: Val::Px(1020.0),
                        bottom: Val::Px(10.0),
                        right: Val::Px(0.0),},
                    ..default()
                },
                border_color: BorderColor(Color::BLACK),
                background_color: NORMAL_BUTTON.into(),
                ..default()
            },
            OnGameScreen,),
            )           
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section("Hint P2", button_text_style.clone()));
            });
        }
        if (*mode != Mode::IAP1 && *mode != Mode::IAP1P2) {
            commands
            .spawn((ButtonBundle {
                style: Style {
                    width: Val::Px(110.0),
                    height: Val::Px(45.0),
                    border: UiRect::all(Val::Px(0.0)),
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    margin: UiRect {
                        top: Val::Px(705.0),
                        left: Val::Px(60.0),
                        bottom: Val::Px(10.0),
                        right: Val::Px(0.0),},
                    ..default()
                },
                border_color: BorderColor(Color::BLACK),
                background_color: NORMAL_BUTTON.into(),
                ..default()
            },
            OnGameScreen,),
            )           
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section("Hint P1", button_text_style));
            });
        }

        commands.spawn((
            TextBundle::from_section(
                "Time: ",
                TextStyle {
                    font: asset_server.load("fonts/MontserratExtrabold-VGO60.ttf"),
                    font_size: 20.0,
                    ..default()
                },
            ) 
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: Val::Px(650.0),
                left: Val::Px(75.0),
                ..default()
            }),
            TimeText,
        ));
        
        commands.spawn((
            TextBundle::from_section(
                "Time: ",
                TextStyle {
                    font: asset_server.load("fonts/MontserratExtrabold-VGO60.ttf"),
                    font_size: 20.0,
                    ..default()
                },
            ) 
            .with_style(Style {
                position_type: PositionType::Absolute,
                left: Val::Px(1035.0),
                top: Val::Px(105.0),
                ..default()
            }),
            TimeText,
        ));

        commands.spawn((
            TextBundle::from_section(
                "Captures: 0",
                TextStyle {
                    font: asset_server.load("fonts/MontserratExtrabold-VGO60.ttf"),
                    font_size: 20.0,
                    ..default()
                },
            ) 
            .with_style(Style {
                position_type: PositionType::Absolute,
                left: Val::Px(65.0),
                top: Val::Px(670.0),
                ..default()
            }),
            CaptureText,
        ));

        commands.spawn((
            TextBundle::from_section(
                "Captures: 0",
                TextStyle {
                    font: asset_server.load("fonts/MontserratExtrabold-VGO60.ttf"),
                    font_size: 20.0,
                    ..default()
                },
            ) 
            .with_style(Style {
                position_type: PositionType::Absolute,
                left: Val::Px(1025.0),
                top: Val::Px(125.0),
                ..default()
            }),
            CaptureText,
        ));

    // Creating a grid of empty tiles.
    if *mode != Mode::Normal && *iapos == IAPosition::P1 {
        game.start_ia();
        *player = Player::P2;
    }
    game.print_map();
    print_ui_map(&game, &mut commands, tile_size, &mut zf);
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

fn print_ui_tile(position: Position, tile_size: f32, commands: &mut Commands, color: Color, zf: f32) {
    commands
    .spawn((SpriteBundle {
        //material: materials.add(Color::rgba_u8(238, 228, 218, 90).into()),
        sprite: Sprite {
            custom_size: Some(Vec2::new(tile_size, tile_size)),
            color: color.into(),
            ..default()
        },
        transform: Transform::from_translation(position.to_vec3(zf)),
        ..Default::default()
    },
    OnGameScreen),
    );
}

fn print_ui_hint(position: Position, tile_size: f32, commands: &mut Commands, zf: &mut zfighting) {
    let color = Color::rgba_u8(80, 80, 80, 250); 
    commands
    .spawn((SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(tile_size/2.0, tile_size/2.0)),
            color: color.into(),

            ..default()
        },
        transform: Transform::from_translation(position.to_vec3((zf.0))),
        ..Default::default()
    },
    OnHintScreen),
    );
    println!("zf: {:?}", zf.0);
    *zf = zfighting(zf.0 + 0.0001);
}

fn print_ui_map(game: &Game, commands: &mut Commands, tile_size: f32, zf: &mut zfighting) {
    let empty_tile : Color = Color::rgba_u8(238, 228, 218, 250);
    let p1_tile : Color = Color::rgba_u8(238, 50, 50, 250);
    let p2_tile : Color = Color::rgba_u8(50, 50, 218, 255);
    for i in 0..19 {
        for j in 0..19 {
            let position = Position { row: 18 - i, col: j};
            match game.map[i][j] {
                Piece::Empty => print_ui_tile(position, tile_size, commands, empty_tile, zf.0),
                Piece::Player1 => print_ui_tile(position, tile_size, commands, p1_tile, zf.0),
                Piece::Player2 => print_ui_tile(position, tile_size, commands, p2_tile, zf.0)
            }
            *zf = zfighting(zf.0 + 0.0001);
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
    query: Query<Entity, With<OnHintScreen>>,
    finished: Res<Finished>,
    mut zf: ResMut<zfighting>,
    ) {

    if *finished == Finished(false){
        let center = Vec2::new(600.0, 400.0);
        let board = 500.0 + (20.0 * 10.0);
        let tile_size = 500.0 /19.0;
        let tile_spacing = 10.0;



        if mouse_button_input.just_pressed(MouseButton::Left) {
            let mouse: Vec2 = windows.single().cursor_position().unwrap() - Vec2::new(260.0, 60.0);
            if mouse.x < 0.0 || mouse.x > 680.0 || mouse.y < 0.0 || mouse.y > 680.0 {
                let abs = windows.single().cursor_position().unwrap();
                return;
            }
            let row = (19.0 - mouse.y / (tile_size + 10.0)) as usize;
            let col = ( mouse.x / (tile_size + 10.0)) as usize;
        
            let position = Position { row, col};

            info!("{}", windows.single().cursor_position().unwrap());
            info!("row: {}, col: {}, pl: {:?}", row, col, *player);

            for entity in query.iter() {
                commands.entity(entity).despawn_recursive();
            }
            
            match *player {
                Player::P1 => {
                    let p_back = position.clone().to_backend();
                    info!("click on coordinates: {} {}", p_back.0, p_back.1);
                    if !game.update_game(p_back.0, p_back.1, Piece::Player1) {
                        info!("Invalid move");
                        return;
                    }
                    *player = Player::P2;
                    print_ui_map(&game, &mut commands, tile_size, &mut zf);
                },
                Player::P2 => {
                    let p_back = position.clone().to_backend();
                    info!("click on coordinates: {} {}", p_back.0, p_back.1);
                    if !game.update_game(p_back.0, p_back.1, Piece::Player2) {
                        info!("Invalid move");
                        return;
                    }
                    print_ui_map(&game, &mut commands, tile_size, &mut zf);
                    *player = Player::P1;
                }
            }
            game.print_map();
        }

        if mouse_button_input.just_released(MouseButton::Left) {
            info!("left mouse just released");
        }
    }
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const HOVERED_PRESSED_BUTTON: Color = Color::rgb(0.25, 0.65, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut game_state: ResMut<NextState<GameState>>,
    mut game: ResMut<Game>,
    mut commands: Commands,
    mut player: ResMut<Player>, 
    mut playerTimes: ResMut<PlayerTimes>,
    finished: Res<Finished>,
    mut zf: ResMut<zfighting>,
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        let text = text_query.get_mut(children[0]).unwrap().sections[0].value.clone();
        let tile_size = 500.0 /19.0;
        match (*interaction) {
            Interaction::Pressed => {
                *color = HOVERED_BUTTON.into();
                println!("Player {:?}", *player);
                match (text.as_str(), *player){
                    ("Quit", _) => {
                        game_state.set(GameState::Menu);
                        game.restart();
                        *playerTimes = PlayerTimes(0, 0);
                    },
                    ("Hint P1", Player::P1) => {
                        if *finished == Finished(false){
                            let hint = game.hint(1);
                            let position = Position { 
                                row: 18 - hint.0 as usize, 
                                col: hint.1 as usize
                            };
                            println!("Hint P1 {:?}", position);
                            print_ui_hint(position, tile_size, &mut commands, &mut zf)
                        }
                    },
                    ("Hint P2", Player::P2) => {
                        if *finished == Finished(false){
                            let hint = game.hint(2);
                            let position = Position { 
                                row: 18 - hint.0 as usize, 
                                col: hint.1 as usize
                            };
                            println!("Hint P1 {:?}", position);
                            print_ui_hint(position, tile_size, &mut commands, &mut zf)
                        }
                    },
                    _ => {},
                }
            }
            Interaction::Hovered => *color = HOVERED_BUTTON.into(),
            Interaction::None => *color = NORMAL_BUTTON.into(),
        };
    }
}


fn countdown(
    mut game_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
    mut timer: ResMut<GameTimer>,
    player: Res<Player>,
    mut player_times: ResMut<PlayerTimes>,
    mut query: Query<&mut Text, With<TimeText>>,
) {
    if timer.tick(time.delta()).finished() {
        match *player {
            Player::P1 => player_times.0 += 1,
            Player::P2 => player_times.1 += 1,
        }
        let mut i = 0;
        for mut entity in query.iter_mut() {
            match i {  
                0 => entity.sections[0].value = format!("Time: {}.{}", player_times.0/10, player_times.0%10),
                1 => entity.sections[0].value = format!("Time: {}.{}", player_times.1/10, player_times.1%10),
                _ => {}
            
            }
            i += 1;
        }
    }
}


fn captures(
    mut game_state: ResMut<NextState<GameState>>,
    game: Res<Game>,
    mut query: Query<&mut Text, With<CaptureText>>,
) {
    let mut i = 0;
    for mut entity in query.iter_mut() {
        match i {  
            0 => entity.sections[0].value = format!("Captures: {}", game.captured2),
            1 => entity.sections[0].value = format!("Captures: {}", game.captured1),
            _ => {}
        
        }
        i += 1;
    }
}


fn IA_move(
    mut game: ResMut<Game>,
    mut player: ResMut<Player>,
    mut commands: Commands,
    mode: Res<Mode>,
    mut player_times: ResMut<PlayerTimes>,
    pl: Res<IAPosition>,
    finished: Res<Finished>,
    mut zf: ResMut<zfighting>,
) {
    if *finished == Finished(false){
        let tile_size = 500.0 /19.0;
        match *mode {
            Mode::IAP1 => {
                if *player == Player::P1 {
                    let start = Instant::now();
                    game.place_ia(1);
                    let time = (start.elapsed().as_secs_f64() * 10.0) as u32;
                    player_times.0 += time;
                    print_ui_map(&game, &mut commands, tile_size, &mut zf);
                    *player = Player::P2;
                }
            },
            Mode::IAP2 => {
                if *player == Player::P2 {
                    let start = Instant::now();
                    game.place_ia(2);
                    let time = (start.elapsed().as_secs_f64() * 10.0) as u32;
                    player_times.1 += time;
                    print_ui_map(&game, &mut commands, tile_size, &mut zf);
                    *player = Player::P1;
                }
            },
            Mode::IAP1P2 => {
                match *player {
                    Player::P1 => game.place_ia(1),
                    Player::P2 => game.place_ia(2),
                };
                print_ui_map(&game, &mut commands, tile_size, &mut zf);
                *player = match *player {
                    Player::P1 => Player::P2,
                    Player::P2 => Player::P1,
                };
            },
            _ => {}
        }
    }
}


fn game_ended(
    mut game_state: ResMut<NextState<GameState>>,
    game: Res<Game>,
    mut player: ResMut<Player>,
    mut player_times: ResMut<PlayerTimes>,
    mut finished: ResMut<Finished>,
    mut timer: ResMut<GameTimer>,
) {
    if *finished == Finished(false){
        if (game.check_win() == (true, Piece::Player1)) || (game.check_win() == (true, Piece::Player2)) {
            *finished = Finished(true);
            timer.pause();
            println!("Player {:?} wins", game.check_win().1);
            println!("Segmentation Fault (core dumped)");
            // ia::store_transposition_table();
        }
    }
}