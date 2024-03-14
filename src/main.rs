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
enum IAPosition {
    P1,
    P2,
}

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

#[derive(Resource, Debug, Component, PartialEq, Clone, Copy)]
struct zfighting(f32);

//struct bevyGame(Game);
fn main() {
   
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
    .insert_resource(IAPosition::P1)
    .insert_resource(Player::P1)
    .insert_resource(game::Game::new())
    .insert_resource(Mode::IAP1)
    .insert_resource(zfighting(0.0))
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
