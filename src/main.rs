use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    core::transform::{Transform, TransformBundle},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
        Camera,
        SpriteSheet,
        SpriteSheetFormat,
        Texture,
        ImageFormat,
    },
    input::{InputBundle, StringBindings},
    utils::application_root_dir,
    ui::{RenderUi, UiBundle},
};

mod components;
mod systems;
mod resources;
use systems::{
    commands::CommandSystem,
    movement::MovementSystem,
    combat::CombatSystem,
    enemy_targeting::EnemyTargetingSystem,
    animation::IdleAnimationSystem,
};
use resources::map::MapBuilder;

// These are px dimensions used to
// calc our tile dimensions.
// pub const ARENA_HEIGHT: f32 = 320.0;
// pub const ARENA_WIDTH: f32 = 640.0;
pub const ARENA_HEIGHT: f32 = 640.0;
pub const ARENA_WIDTH: f32 = 1280.0;
pub const TILE_WIDTH: f32 = 16.0;

struct MainState;

impl MainState {
    fn new() -> MainState {
        MainState {}
    }
}

impl SimpleState for MainState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let mut builder = MapBuilder::initialize(world);

        builder.build_map(world);

        initialize_camera(world);
    }
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let assets_dir = app_root.join("assets");
    let config_dir = app_root.join("config");
    let display_config_path = config_dir.join("display.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderUi::default()),
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(InputBundle::<StringBindings>::new())?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with(CommandSystem::default(), "command_system", &["input_system"])
        .with(MovementSystem, "movement_system", &["command_system"])
        .with(EnemyTargetingSystem, "enemy_targeting_system", &["movement_system"])
        .with(CombatSystem, "combat_system", &["movement_system", "enemy_targeting_system"])
        .with(IdleAnimationSystem::default(), "anim_system", &["movement_system", "combat_system", "enemy_targeting_system"]);

    let mut game = Application::new(assets_dir, MainState::new(), game_data)?;
    game.run();

    Ok(())
}

fn initialize_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(transform)
        .build();
}

pub fn load_sprite_sheet(world: &mut World, texture_file: &str, ron_file: &str) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            texture_file,
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        ron_file,
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}

fn calc_tile_center(x: usize) -> f32 {
    (x as f32 * TILE_WIDTH) + (0.5 * TILE_WIDTH)
}
