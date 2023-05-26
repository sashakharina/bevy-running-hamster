use std::time::Duration;

use bevy::{prelude::*, window::PrimaryWindow};

const MAX_ANIMATION_DURATION_IN_MILLIS: u64 = 800;
const SCOREBOARD_FONT_SIZE: f32 = 40.0;
const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);
const TEXT_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);
const SCORE_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(Scoreboard { score: 0 })
        .add_startup_systems((
            setup_camera,
            setup_hamster,
            setup_score,
        ))
        .add_systems((
            animate_sprite,
            update_scoreboard,
        ))
        .run();
}

#[derive(Resource)]
struct Scoreboard {
    score: u64,
}

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
    mut scoreboard: ResMut<Scoreboard>,
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        let decrease_time_step = 40 as u64;
        let increase_time_step = 1;
        let mut duration = timer.duration().as_millis() as u64;

        duration = duration + increase_time_step;

        if duration >= MAX_ANIMATION_DURATION_IN_MILLIS {
            timer.pause();
            sprite.index = indices.first;
            duration = MAX_ANIMATION_DURATION_IN_MILLIS;
        }

        if keyboard_input.just_pressed(KeyCode::Space) {
            if duration > decrease_time_step {
                duration = duration - decrease_time_step;
                timer.unpause();
            }
        }

        if 1000/duration > scoreboard.score && duration < MAX_ANIMATION_DURATION_IN_MILLIS {
            scoreboard.score = 1000/duration;
        }

        timer.set_duration(Duration::from_millis(duration));

            timer.tick(time.delta());
            if timer.just_finished() {
                sprite.index = if sprite.index == indices.last {
                    indices.first
                } else {
                    sprite.index + 1
                };
            }
        }
}

fn update_scoreboard(scoreboard: Res<Scoreboard>, mut query: Query<&mut Text>) {
    let mut text = query.single_mut();
    text.sections[1].value = scoreboard.score.to_string();
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

    let hamster_speed = MAX_ANIMATION_DURATION_IN_MILLIS as f32 / 1000.0;
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
        AnimationTimer(Timer::from_seconds(hamster_speed, TimerMode::Repeating)),
    ));
}

fn setup_score(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    scoreboard: Res<Scoreboard>,
) {
    commands.spawn(
        TextBundle::from_sections([
            TextSection::new(
                "Score: ",
                TextStyle {
                    font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                    font_size: SCOREBOARD_FONT_SIZE,
                    color: TEXT_COLOR,
                },
            ),
            TextSection::new(
                scoreboard.score.to_string(),
                TextStyle {
                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                font_size: SCOREBOARD_FONT_SIZE,
                color: SCORE_COLOR,
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: SCOREBOARD_TEXT_PADDING,
                left: SCOREBOARD_TEXT_PADDING,
                ..default()
            },
            ..default()
        }),
    );
}
