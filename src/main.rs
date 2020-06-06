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

use components::npc::{NpcVariant, initialise_npc};
use components::tile::{TileVariant, initialise_tile};
use systems::{
    commands::CommandSystem,
    movement::MovementSystem,
    combat::CombatSystem,
    enemy_targeting::EnemyTargetingSystem,
    animation::IdleAnimationSystem,
};

pub const ARENA_HEIGHT: f32 = 320.0;
pub const ARENA_WIDTH: f32 = 640.0;
pub const TILE_WIDTH: f32 = 16.0;
pub const UNIT_WIDTH: usize = (ARENA_WIDTH / TILE_WIDTH) as usize;
pub const UNIT_HEIGHT: usize = (ARENA_HEIGHT / TILE_WIDTH) as usize;

struct Rect {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

impl Rect {
    pub fn new(x: usize, y: usize, w: usize, h: usize) -> Self {
        Rect {
            x: x,
            y: y,
            width: w,
            height: h,
        }
    }
}

// Converts the map tile matrix into rendered entities.
fn initialise_map(
    world: &mut World, 
    map: &Map,
    ceiling_sheet_handle: Handle<SpriteSheet>,
    floor_sheet_handle: Handle<SpriteSheet>,
    wall_sheet_handle: Handle<SpriteSheet>,
) {
    for (x, row) in map.iter().enumerate() {
        for (y, tile) in row.iter().enumerate() {
            let x_coords = calc_tile_center(x);
            let y_coords = calc_tile_center(y);
            let handler = match tile {
                TileVariant::Ceiling => ceiling_sheet_handle.clone(),
                TileVariant::Floor => floor_sheet_handle.clone(),
                TileVariant::Wall => wall_sheet_handle.clone(),
                _ => ceiling_sheet_handle.clone(),
            };

            initialise_tile(world, *tile, handler, [x_coords, y_coords], 0);
        }
    }
}

fn initialise_room(
    map: &mut Map,
    rect: Rect, 
) {
    let max_y = rect.y + rect.height;
    let max_x = rect.x + rect.width;

    // Init floor
    for y in rect.y..=max_y {
        for x in rect.x..=max_x {
            map[x][y] = TileVariant::Floor;
        }
    }

    // Init walls
    for x in rect.x..=max_x {
        map[x][max_y + 1] = TileVariant::Wall;
    }
}

type Map = [[TileVariant; UNIT_HEIGHT]; UNIT_WIDTH];

struct MainState {
    floor_sheet_handle: Option<Handle<SpriteSheet>>,
    ceiling_sheet_handle: Option<Handle<SpriteSheet>>,
    wall_sheet_handle: Option<Handle<SpriteSheet>>,
    npc_sheet_handle: Option<Handle<SpriteSheet>>,
    map: Map,
}

impl MainState {
    fn new() -> MainState {
        MainState {
            floor_sheet_handle: None,
            ceiling_sheet_handle: None,
            wall_sheet_handle: None,
            npc_sheet_handle: None,
            map: [[TileVariant::Empty; UNIT_HEIGHT]; UNIT_WIDTH],
        }
    }

    fn load_sheet_handles(&mut self, world: &mut World) {
        self.ceiling_sheet_handle.replace(load_sprite_sheet(world, "texture/ceiling.png", "texture/ceiling.ron"));
        self.floor_sheet_handle.replace(load_sprite_sheet(world, "texture/floor.png", "texture/floor.ron"));
        self.wall_sheet_handle.replace(load_sprite_sheet(world, "texture/walls.png", "texture/walls.ron"));
        self.npc_sheet_handle.replace(load_sprite_sheet(world, "texture/warrior.png", "texture/warrior.ron"));
    }
}

impl SimpleState for MainState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        self.load_sheet_handles(world);

        // Theres gotta be a better way than creating a new map 
        // and replacing. Attempted replacing in place but
        // ran into mutability issues...
        let mut new_map_base = self.map.clone();
        for (x, row) in self.map.iter_mut().enumerate() {
            for (y, _tile) in row.iter_mut().enumerate() {
                new_map_base[x][y] = TileVariant::Ceiling;
            }
        }
        self.map = new_map_base;

        let room1 = Rect::new(6, 3, 10, 5);
        let room2 = Rect::new(15, 8, 20, 8);

        initialise_room(&mut self.map, room1);
        initialise_room(&mut self.map, room2);

        initialise_map(
            world, 
            &self.map, 
            self.ceiling_sheet_handle.clone().unwrap(), 
            self.floor_sheet_handle.clone().unwrap(), 
            self.wall_sheet_handle.clone().unwrap()
        );

        // TODO make npc spawning smart
        initialise_npc(world, NpcVariant::Normal, self.npc_sheet_handle.clone().unwrap(), [ARENA_WIDTH / 2.0, ARENA_HEIGHT / 2.0]);
        initialise_npc(world, NpcVariant::Orc, self.npc_sheet_handle.clone().unwrap(), [(ARENA_WIDTH / 2.0) + 150.0, (ARENA_HEIGHT / 2.0) + 50.0]);
        initialise_camera(world);
    }
}

fn calc_tile_center(x: usize) -> f32 {
    x as f32 * TILE_WIDTH + 0.5 * TILE_WIDTH
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

fn initialise_camera(world: &mut World) {
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
