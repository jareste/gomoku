
mod gameUI;
mod menu;
mod game;

use bevy::{prelude::*, window::WindowResolution};
use bevy_prototype_lyon::prelude::*;



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
    IA,
}

//struct bevyGame(Game);

fn main() {
    App::new()

        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(1200., 800.).with_scale_factor_override(1.0),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(ShapePlugin)
        // Insert as resource the initial value for the settings resources
        .insert_resource(IAQuality::Medium)
        .insert_resource(MinMaxProf(7))
        .insert_resource(Player::P1)
        .insert_resource(game::Game::new())
        .insert_resource(Mode::Normal)
        //.insert_resource(bevyGame(Game::new()))
        // Declare the game state, whose starting value is determined by the `Default` trait
        .init_state::<GameState>()
        .add_systems(Startup, setup)
        // Adds the plugins for each state
        .add_plugins((menu::menu_plugin, gameUI::gameUI_plugin))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}


// Generic system that takes a component as a parameter, and will despawn all entities with that component
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

