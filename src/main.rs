use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup)
        .add_systems(Update, animate_sprite)
        .run();
}

#[derive(Component)]
struct AnimationIndicies {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndicies, &mut AnimationTimer, &mut TextureAtlas)>,
) {
    for (indicies, mut timer, mut atlas) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            atlas.index = if atlas.index == indicies.last {
                indicies.first
            } else {
                atlas.index + 1
            };
        }
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("textures/knight_idle.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(100), 6, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let animation_indices = AnimationIndicies { first: 0, last: 5 };

    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_scale(Vec3::splat(6.0)),
            texture,
            ..default()
        },
        TextureAtlas {
            layout: texture_atlas_layout,
            index: animation_indices.first,
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ));
}
