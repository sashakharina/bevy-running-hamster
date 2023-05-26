use std::time::Duration;

use bevy::{prelude::*, window::PrimaryWindow};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_startup_systems((
            setup_camera,
            setup_hamster,
        ))
        .add_systems(
            (animate_sprite,
        ))
        .run();
}

#[derive(Component)]
struct Speed (f32);

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn animate_sprite(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        let max_time = 800;
        let decrease_time_step = 40 as u64;
        let increase_time_step = 1;
        let mut duration = timer.duration().as_millis() as u64;

        duration = duration + increase_time_step;

        if duration >= max_time {
            timer.pause();
            sprite.index = indices.first;
            duration = max_time;
        }

        if keyboard_input.just_pressed(KeyCode::Space) {
            if duration > decrease_time_step {
                duration = duration - decrease_time_step;
                timer.unpause();
            }
        }

        timer.set_duration(Duration::from_millis(duration));

        // if !timer.paused() {
            timer.tick(time.delta());
            if timer.just_finished() {
                sprite.index = if sprite.index == indices.last {
                    indices.first
                } else {
                    sprite.index + 1
                };
            }
        }
    // }
}

fn setup_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    let window = window_query.get_single().unwrap();
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..Default::default()
    });
}

fn setup_hamster(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let window = window_query.get_single().unwrap();

    let texture_handle = asset_server.load("textures/walking_humster.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(33.0, 40.0), 1, 6, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
  
    let animation_indices = AnimationIndices { first: 1, last: 4 };

    let hamster_speed = Speed(1.0);
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(animation_indices.first),
            transform: Transform::from_xyz(
                window.width() / 2.0, 
                window.height() / 2.0, 
                0.0)
                .with_scale(Vec3::splat(5.0)),
            ..default()
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(hamster_speed.0, TimerMode::Repeating)),
    ));
}
