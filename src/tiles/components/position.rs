use crate::prelude::*;

#[derive(Component, Reflect, Default, Eq, PartialEq, Hash, Copy, Clone, Debug)]
#[reflect(Component)]
pub struct Position(pub IVec2);

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self(IVec2::new(x, y))
    }

    pub fn to_world(&self) -> Vec3 {
        Vec3::new(
            self.0.x as f32 * TILE_SIZE as f32,
            self.0.y as f32 * TILE_SIZE as f32,
            0.0,
        )
    }

    pub fn distance(&self, other: &Position) -> i32 {
        let Position(IVec2 { x: x1, y: y1 }) = self;
        let Position(IVec2 { x: x2, y: y2 }) = other;

        let dx = (x2 - x1).abs();
        let dy = (y2 - y1).abs();

        if dx > dy {
            14 * dy + 10 * (dx - dy)
        } else {
            14 * dx + 10 * (dy - dx)
        }
    }

    pub fn get_neighbors_and_costs(
        &self,
        query: &Query<(Entity, &Position, &Walkable), (With<Tile>, With<Floor>)>,
        tile_storage: &Query<&TileLookup>,
    ) -> Vec<(Position, u32)> {
        let mut neighbors = Vec::new();
        // let &GridNode { x, y } = node;
        let &Position(IVec2 { x, y }) = self;

        const STRAIGHT_COST: u32 = 10;
        const DIAGONAL_COST: u32 = 14;

        // TODO: (LOW PRIORITY) Refactor and clean up this code
        let up = Position::new(x, y + 1);
        if Position::check_neighbor_walkable(&up, &query, &tile_storage) {
            neighbors.push((up, STRAIGHT_COST));
        }

        let right = Position::new(x + 1, y);
        if Position::check_neighbor_walkable(&right, &query, &tile_storage) {
            neighbors.push((right, STRAIGHT_COST));
        }

        let down = Position::new(x, y - 1);
        if Position::check_neighbor_walkable(&down, &query, &tile_storage) {
            neighbors.push((down, STRAIGHT_COST));
        }

        let left = Position::new(x - 1, y);
        if Position::check_neighbor_walkable(&left, &query, &tile_storage) {
            neighbors.push((left, STRAIGHT_COST));
        }

        let up_left = Position::new(x - 1, y + 1);
        if Position::check_neighbor_walkable(&up_left, &query, &tile_storage) {
            if neighbors.contains(&(up, STRAIGHT_COST))
                && neighbors.contains(&(left, STRAIGHT_COST))
            {
                neighbors.push((up_left, DIAGONAL_COST));
            }
        }

        let up_right = Position::new(x + 1, y + 1);
        if Position::check_neighbor_walkable(&up_right, &query, &tile_storage) {
            if neighbors.contains(&(up, STRAIGHT_COST))
                && neighbors.contains(&(right, STRAIGHT_COST))
            {
                neighbors.push((up_right, DIAGONAL_COST));
            }
        }

        let down_right = Position::new(x + 1, y - 1);
        if Position::check_neighbor_walkable(&down_right, &query, &tile_storage) {
            if neighbors.contains(&(down, STRAIGHT_COST))
                && neighbors.contains(&(right, STRAIGHT_COST))
            {
                neighbors.push((down_right, DIAGONAL_COST));
            }
        }

        let down_left = Position::new(x - 1, y - 1);
        if Position::check_neighbor_walkable(&down_left, &query, &tile_storage) {
            if neighbors.contains(&(down, STRAIGHT_COST))
                && neighbors.contains(&(left, STRAIGHT_COST))
            {
                neighbors.push((down_left, DIAGONAL_COST));
            }
        }

        neighbors
    }

    fn check_neighbor_walkable(
        position: &Position,
        query: &Query<(Entity, &Position, &Walkable), (With<Tile>, With<Floor>)>,
        tile_storage: &Query<&TileLookup>,
    ) -> bool {
        if let Ok(tile_storage) = tile_storage.get_single() {
            if let Some(entity) = tile_storage.tiles.get(position) {
                if query.get(*entity).is_err() {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }
}

impl From<Vec3> for Position {
    fn from(Vec3 { x, y, z: _z }: Vec3) -> Self {
        Self::new((x / TILE_SIZE as f32) as i32, (y / TILE_SIZE as f32) as i32)
    }
}
