use amethyst::SimpleState;
use amethyst::StateData;
use amethyst::GameData;
use amethyst::prelude::*;
use amethyst::{
    assets::{AssetStorage, Loader},
    core::transform::Transform,
    ecs::prelude::World,
    prelude::*,
    renderer::{
        Camera, Flipped, PngFormat, Projection, SpriteRender, SpriteSheet, SpriteSheetFormat,
        SpriteSheetHandle, Texture, TextureMetadata,
    },
    ui::{Anchor, TtfFormat, UiText, UiTransform},
};
use crate::components::{Player, Movement};
use crate::components::player::{PLAYER_WIDTH, PLAYER_HEIGHT, PLAYER_VELOCITY};


pub struct SeekingWarmth;

impl SimpleState for SeekingWarmth {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let  world = data.world;
        let spritesheet_handle = load_sprite_sheet(world);

        world.register::<Player>();
        initialise_player(world, spritesheet_handle);
        initialise_camera(world)
    }
}

pub const CAMERA_WIDTH: f32 = 400.0;
pub const CAMERA_HEIGHT: f32 = 300.0;

fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_z(1.0);
    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            0.0,
            CAMERA_WIDTH,
            0.0,
            CAMERA_HEIGHT,
        )))
        .with(transform)
        .build();
}

fn load_sprite_sheet(world: &mut World) -> SpriteSheetHandle {
    // Load the sprite sheet necessary to render the graphics.
    // The texture is the pixel data
    // `sprite_sheet` is the layout of the sprites on the image
    // `texture_handle` is a cloneable reference to the texture
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "player_sprite.png",
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "player_spritesheet.ron", // Here we load the associated ron file
        SpriteSheetFormat,
        texture_handle, // We pass it the texture we want it to use
        (),
        &sprite_sheet_store,
    )
}

fn initialise_player(world: &mut World, sprite_sheet_handle: SpriteSheetHandle) {

    let mut player_transform = Transform::default();

    let y = PLAYER_HEIGHT * 0.5;
    player_transform.set_xyz(PLAYER_WIDTH * 0.5, y, 0.0);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: 0, // paddle is the first sprite in the sprite_sheet
    };

    world
        .create_entity()
        .with(sprite_render.clone())
        .with(Movement::new())
        .with(Player {
            width: PLAYER_WIDTH,
            height: PLAYER_HEIGHT,
            velocity: PLAYER_VELOCITY,
        })
        .with(player_transform)
        .build();
}
