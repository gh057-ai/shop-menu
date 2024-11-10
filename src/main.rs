use bevy::prelude::*;
use log::debug;

mod shop;
use shop::{GameState, ShopPlugin};

/* RUN WITH BASH CMD */
// RUST_LOG=warn,shop_menu=debug,your_crate_name=debug cargo run
fn main() {
    // Initialize logging with filtered output
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug"))
        .filter_module("wgpu_core", log::LevelFilter::Warn)
        .filter_module("wgpu_hal", log::LevelFilter::Warn)
        .filter_module("naga", log::LevelFilter::Warn)
        .filter_module("bevy_render", log::LevelFilter::Warn)
        .filter_module("bevy_window", log::LevelFilter::Warn)
        .filter_module("bevy_winit", log::LevelFilter::Warn)
        .filter_module("calloop", log::LevelFilter::Warn)
        .init();

    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .add_plugins(ShopPlugin)
        .add_systems(Startup, setup_camera)
        .add_systems(Update, handle_exit)
        .run();
}

fn setup_camera(mut commands: Commands) {
    debug!("Setting up camera");
    commands.spawn(Camera2dBundle::default());
}

fn handle_exit(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut exit: EventWriter<AppExit>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        debug!("Exit requested via escape key");
        exit.send(AppExit::Success);
    }
}