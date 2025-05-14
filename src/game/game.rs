use bevy::prelude::*;
use bevy_ascii_terminal::{Tile as AsciiTile, *};

use crate::get_map_1;

#[derive(Component)]
pub struct Player {
    name: String,
}

#[derive(Component)]
pub struct Board {
    pub map: GameMap,
    tile_stacks: Vec<Vec<TileStack>>,
}

impl Board {
    pub fn new() -> Self {
        let map = get_map_1();

        let tile_stacks = map.get_tile_stacks();

        Self { map, tile_stacks }
    }

    pub fn render_board_tiles(
        &self,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) -> Vec<(Mesh3d, MeshMaterial3d<StandardMaterial>, Transform)> {
        self.tile_stacks
            .iter()
            .flat_map(|row| {
                row.iter()
                    .map(|tile_stack| tile_stack.render(&mut meshes, &mut materials))
                    .collect::<Vec<_>>()
            })
            .collect()
    }
}

pub struct TileStack {
    game_tile: GameTile,
    board_tile: BoardTile,
}

impl TileStack {
    pub fn new(game_tile: GameTile, board_tile: BoardTile) -> Self {
        Self {
            game_tile,
            board_tile,
        }
    }

    pub fn game_tile(&self) -> &GameTile {
        &self.game_tile
    }

    pub fn board_tile(&self) -> &BoardTile {
        &self.board_tile
    }

    pub fn render(
        &self,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) -> (Mesh3d, MeshMaterial3d<StandardMaterial>, Transform) {
        self.board_tile.render(meshes, materials)
    }
}

pub trait Tile {
    fn x(&self) -> usize;
    fn y(&self) -> usize;
}

pub struct GameTile {
    x: usize,
    y: usize,
    is_occupied: bool,
}

impl Tile for GameTile {
    fn x(&self) -> usize {
        self.x
    }

    fn y(&self) -> usize {
        self.y
    }
}

impl GameTile {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            is_occupied: false,
        }
    }

    pub fn is_occupied(&self) -> bool {
        self.is_occupied
    }

    pub fn set_occupied(&mut self, occupied: bool) {
        self.is_occupied = occupied;
    }
}

#[derive(Component, Clone)]
pub struct BoardTile {
    x: usize,
    y: usize,
    tile_type: TileType,
}

impl BoardTile {
    pub fn new(x: usize, y: usize, tile_type: TileType) -> Self {
        Self { x, y, tile_type }
    }

    const FLAT_TILE_HEIGHT: f32 = 0.5;
    const STAIRS_TILE_HEIGHT: f32 = 0.75;
    const ELEVATED_TILE_HEIGHT: f32 = 1.0;
    const TRANSFORM_TILE_HEIGHT: f32 = 0.5;
    const WALL_TILE_HEIGHT: f32 = 2.0;
    pub fn render(
        &self,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) -> (Mesh3d, MeshMaterial3d<StandardMaterial>, Transform) {
        match self.tile_type {
            TileType::Normal => (
                Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, Self::FLAT_TILE_HEIGHT))),
                MeshMaterial3d(materials.add(Color::srgb(0.23, 0.23, 0.22))),
                Transform::from_xyz(
                    self.x as f32,
                    self.y as f32,
                    Self::TRANSFORM_TILE_HEIGHT as f32,
                ),
            ),
            TileType::Elevation => (
                Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, Self::ELEVATED_TILE_HEIGHT))),
                MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6))),
                Transform::from_xyz(
                    self.x as f32,
                    self.y as f32,
                    Self::TRANSFORM_TILE_HEIGHT as f32,
                ),
            ),
            TileType::SpecialZone => (
                Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, Self::FLAT_TILE_HEIGHT))),
                MeshMaterial3d(materials.add(Color::srgb_u8(137, 171, 162))),
                Transform::from_xyz(
                    self.x as f32,
                    self.y as f32,
                    Self::TRANSFORM_TILE_HEIGHT as f32,
                ),
            ),
            TileType::AdverseTerrain => (
                Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, Self::FLAT_TILE_HEIGHT))),
                MeshMaterial3d(materials.add(Color::srgb_u8(46, 19, 71))),
                Transform::from_xyz(
                    self.x as f32,
                    self.y as f32,
                    Self::TRANSFORM_TILE_HEIGHT as f32,
                ),
            ),
            TileType::Objective => (
                Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, Self::FLAT_TILE_HEIGHT))),
                MeshMaterial3d(materials.add(Color::srgb_u8(219, 215, 81))),
                Transform::from_xyz(
                    self.x as f32,
                    self.y as f32,
                    Self::TRANSFORM_TILE_HEIGHT as f32,
                ),
            ),
            TileType::Stair => (
                Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, Self::STAIRS_TILE_HEIGHT))),
                MeshMaterial3d(materials.add(Color::srgb_u8(14, 14, 14))),
                Transform::from_xyz(
                    self.x as f32,
                    self.y as f32,
                    Self::TRANSFORM_TILE_HEIGHT as f32,
                ),
            ),
            TileType::Wall => (
                Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, Self::WALL_TILE_HEIGHT))),
                MeshMaterial3d(materials.add(Color::srgb(0.0, 0.0, 0.0))),
                Transform::from_xyz(
                    self.x as f32,
                    self.y as f32,
                    Self::TRANSFORM_TILE_HEIGHT as f32,
                ),
            ),
            TileType::Hazard => (
                Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, Self::FLAT_TILE_HEIGHT))),
                MeshMaterial3d(materials.add(Color::srgb(0.219, 0.164, 0.81))),
                Transform::from_xyz(
                    self.x as f32,
                    self.y as f32,
                    Self::TRANSFORM_TILE_HEIGHT as f32,
                ),
            ),
        }
    }
}

impl Tile for BoardTile {
    fn x(&self) -> usize {
        self.x
    }

    fn y(&self) -> usize {
        self.y
    }
}

#[derive(Component, Clone)]
pub struct GameMap {
    pub name: String,
    pub tiles: Vec<Vec<TileType>>,
    pub flavor_text: String,
    pub description: String,
}

impl GameMap {
    pub fn new(
        name: String,
        tiles: Vec<Vec<TileType>>,
        flavor_text: String,
        description: String,
    ) -> Self {
        Self {
            name,
            tiles,
            flavor_text,
            description,
        }
    }

    pub fn get_tile_stacks(&self) -> Vec<Vec<TileStack>> {
        let mut tile_stacks = Vec::new();

        for (y, row) in self.tiles.iter().enumerate() {
            let mut tile_stack = Vec::new();
            for (x, tile_type) in row.iter().enumerate() {
                let game_tile = GameTile::new(x, y);
                let board_tile = BoardTile::new(x, y, tile_type.clone());
                tile_stack.push(TileStack::new(game_tile, board_tile));
            }
            tile_stacks.push(tile_stack);
        }

        tile_stacks
    }
}

#[derive(Component, Clone, Copy)]
pub enum TileType {
    Normal,
    Elevation,
    SpecialZone,
    AdverseTerrain,
    Objective,
    Stair,
    Wall,
    Hazard,
}
