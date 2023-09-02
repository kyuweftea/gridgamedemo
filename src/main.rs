use bevy::prelude::*;


#[derive(Resource)]
struct HelloTimer(Timer);


#[derive(Component)]
struct GridPosition { x: u8, y: u8 }


fn add_things(mut commands: Commands) {
    commands.spawn(GridPosition { x: 0, y: 0 });
    commands.spawn(GridPosition { x: 1, y: 2 });
}


fn hello_world(time: Res<Time>, mut timer: ResMut<HelloTimer>, query: Query<&GridPosition>) {
    if timer.0.tick(time.delta()).just_finished() {
        for position in &query {
            println!("Thing at: ({}, {})", position.x, position.y);
        }
    }
}


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(HelloTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
        .add_systems(Startup, add_things)
        .add_systems(Update, hello_world)
        .run();
}
