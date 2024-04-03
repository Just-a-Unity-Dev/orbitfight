use bevy::prelude::*;

#[derive(Component)]
struct Position {x: f32, y: f32}
struct Entity(u64);

fn hello_world() {
    println!("hello world");
}

fn main() {
    App::new()
    .add_systems(Update, hello_world)
    .run();
}
