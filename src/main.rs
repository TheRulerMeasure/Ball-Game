use bevy::prelude::*;
use rand::{seq::SliceRandom, thread_rng};

const CARD_AMMOUNT: u8 = 10;
const MAX_COLUMN: u8 = 5;

#[derive(Component)]
struct Card {
    id: u8,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(mouse_events_system)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let ball_handle = asset_server.load("rball.png");
    commands.spawn_bundle(Camera2dBundle::default());

    let mut arr1: Vec<u8> = (0..=CARD_AMMOUNT / 2).collect::<Vec<_>>();
    let mut card_ids: Vec<u8> = (0..=CARD_AMMOUNT / 2).collect::<Vec<_>>();
    card_ids.append(&mut arr1);

    let mut rng = thread_rng();
    card_ids.shuffle(&mut rng);

    for i in 0..CARD_AMMOUNT {
        let floor_max_column = (i as f32 / MAX_COLUMN as f32).floor();
        let x = (100.0 * (i as f32)) - (floor_max_column * (100.0 * (MAX_COLUMN as f32)));
        let y = floor_max_column * -100.0;

        commands
            .spawn_bundle(SpriteBundle {
                texture: ball_handle.clone(),
                transform: Transform {
                    translation: Vec3 { x: x, y: y, z: 0.0 },
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Card {
                id: card_ids[i as usize],
            });
    }
}

fn mouse_events_system(
    mut cursor_evr: EventReader<CursorMoved>,
) {
    for ev in cursor_evr.iter() {
        print!("x: {}, y: {}", ev.position.x, ev.position.y);
    }
}
