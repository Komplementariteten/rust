use bevy::prelude::*;
use bevy_learn::

fn main() {
    App::new().add_plugins(DefaultPlugins).add_systems(Update, on_update).add_systems(Startup, on_startup).run();
}

fn on_update() {

}

fn hallo_welt() {
    println!("Hallo Welt");
}


pub fn print_all_cards(query: Query<&CompId, With<Card>>) {
    for cardId in &query {
        println!("Card Id:{:?}!", cardId.0);
    }
}

fn on_startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    commands.spawn(Sprite {
        image: asset_server.load("examples/elfen_koenig.png"),
        // Flip the logo to the left
        flip_x: true,
        // And don't flip it upside-down ( the default )
        flip_y: false,
        ..Default::default()
    });
    
}