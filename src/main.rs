#![allow(warnings)]
mod game;
mod ia;
//use game::terminal_game;
//use game::terminal_game_ia;
mod gameUI;
mod menu;
mod constants;
mod heuristic;
use bevy::{prelude::*, window::WindowResolution};
use bevy_prototype_lyon::prelude::*;
use crate::game::Piece;
use crate::ia::IA;
use rand::prelude::SliceRandom;
use std::process;
use ctrlc::set_handler;

const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    #[default]
    Menu,
    Game,
}

#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
enum IAQuality {
    Low,
    Medium,
    High,
}

#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
struct MinMaxProf(u32);

#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
enum Player {
    P1,
    P2,
}

#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
enum Mode {
    Normal,
    IAP1,
    IAP2,
    IAP1P2,
}

//struct bevyGame(Game);
fn main() {
    set_handler(move || {
        println!("Detected Ctrl+C signal. Saving data...");
        ia::store_transposition_table();
        println!("Data saved. Exiting...");
        process::exit(0);
    }).expect("Error setting Ctrl-C handler");
//     let game = 0;
//     if game == 1 {
//         terminal_game();
//         return;
//     }
//     else if game == 0 {
//         terminal_game_ia();
//         return;
//     }
// }
    // let map = game::string_to_map("-  -  -  -  -  -  -  -  -  -  -  -  -  -  -  -  -  -  -  
    // -  -  -  -  -  -  -  -  -  -  -  -  -  -  -  -  -  -  -  
    // -  -  -  -  -  -  -  -  -  -  -  -  -  -  -  -  -  -  -  
    // -  -  -  -  -  -  -  -  -  -  -  -  -  -  -  -  -  -  -  
    // -  -  -  -  -  -  -  -  -  -  -  -  -  -  -  -  -  -  -  
    // -  -  -  -  -  -  -  -  -  -  -  -  -  -  -  -  -  -  -  
    // -  -  -  -  -  -  -  -  -  -  -  -  -  -  -  -  -  -  -  
    // -  -  -  -  -  -  -  -  -  -  O  -  -  -  -  -  -  -  -  
    // -  -  -  -  -  -  -  -  O  O  O  X  -  -  -  -  -  -  -  
    // -  -  -  -  -  -  -  -  O  X  O  -  -  -  -  -  -  -  -  
    // -  -  -  -  -  -  -  -  -  X  X  -  -  -  -  -  -  -  -  
    // -  -  -  -  -  -  X  -  X  X  -  -  -  -  -  -  -  -  -  
    // -  -  -  -  -  -  -  X  -  -  -  -  -  -  -  -  -  -  -  
    // -  -  -  -  -  -  O  -  -  -  -  -  -  -  -  -  -  -  -  
    // -  -  -  -  -  X  -  -  -  -  -  -  -  -  -  -  -  -  -  
    // -  -  -  -  -  -  -  -  -  -  -  -  -  -  -  -  -  -  -  
    // -  -  -  -  -  -  -  -  -  -  -  -  -  -  -  -  -  -  -  
    // -  -  -  -  -  -  -  -  -  -  -  -  -  -  -  -  -  -  -  
    // -  -  -  -  -  -  -  -  -  -  -  -  -  -  -  -  -  -  -  ");

    // let mut game = game::Game::new();
    // game.map = map;
    // // game.place(7,8, Piece::Player2);

    // let piece = game.map[9][8];
    // // game.map[8][8] = Piece::Player1;
    // game.map[7][8] = Piece::Player2;
    //                         game.print_map();
    // // game.map[9][8] = piece;

    // if game.find_free_threes((7, 8), 1, Piece::Player2) {
    //     println!("holaaaaa");
    // } else {
    //     println!("adeeeeu");
    // }

    ia::load_transposition_table();


    // geme.place(9,9, Piece::Player1);

    for i in 0..60000 {
        let mut geme = game::Game::new();
        geme.start_ia();            
        'test: loop {
            // println!("game {}", i);
            let moves = geme.get_possible_moves(false, 1);
            let mut rng = rand::thread_rng();
            if let Some(random_move) = moves.choose(&mut rng) {
                geme.place(random_move.0 as usize, random_move.1 as usize, Piece::Player2);
                // println!("game321 {}", i);
                
                if geme.check_win() == (true, Piece::Player1) {
                    println!("Player 1 wins game {}", i);
                    break 'test;
                }
                if geme.check_win() == (true, Piece::Player2) {
                    println!("Player 2 wins game {}", i);
                    break 'test;
                }

                geme.update_heat_map((random_move.0, random_move.1));
                
                geme.place_ia();
                if geme.check_win() == (true, Piece::Player1) {
                    println!("Player 1 wins game {}", i);
                    break 'test;
                }
                if geme.check_win() == (true, Piece::Player2) {
                    println!("Player 2 wins game {}", i);
                    break 'test;
                }
            }
            else {
                println!("No more moves");
                break 'test;
            }

        }
        geme.print_map();
    }
    ia::store_transposition_table();


    // let mut geme = game::Game::new();
    App::new()

    .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resolution: WindowResolution::new(1200., 800.).with_scale_factor_override(1.0),
            title: "Gomoku".to_string(),
            resize_constraints: WindowResizeConstraints {
                min_width: 1200.,
                min_height: 800.,
                max_width: 1200.,
                max_height: 800.,
                ..default()
            },
            ..default()
        }),
        ..default()
    }))
    .add_plugins(ShapePlugin)
    .insert_resource(IAQuality::Medium)
    .insert_resource(MinMaxProf(7))
    .insert_resource(Player::P2)
    .insert_resource(game::Game::new())
    .insert_resource(Mode::IAP1)
    .init_state::<GameState>()
    .add_systems(Startup, setup)
    .add_plugins((menu::menu_plugin, gameUI::gameUI_plugin))
    .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

