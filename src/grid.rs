use crate::prelude::*;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Deref, DerefMut)]
pub struct GridNode(pub IVec2);

impl GridNode {
    pub fn new(x: i32, y: i32) -> Self {
        Self(IVec2::new(x, y))
    }
}

impl From<(i32, i32)> for GridNode {
    fn from((x, y): (i32, i32)) -> Self {
        Self(IVec2::new(x, y))
    }
}

impl From<(u32, u32)> for GridNode {
    fn from((x, y): (u32, u32)) -> Self {
        Self(IVec2::new(x as i32, y as i32))
    }
}

impl GridNode {
    pub fn distance(&self, other: &GridNode) -> i32 {
        let GridNode(IVec2 { x: x1, y: y1 }) = self;
        let GridNode(IVec2 { x: x2, y: y2 }) = other;

        let dx = (x2 - x1).abs();
        let dy = (y2 - y1).abs();

        if dx > dy {
            14 * dy + 10 * (dx - dy)
        } else {
            14 * dx + 10 * (dy - dx)
        }
    }
}

pub struct Floors {
    pub hashmap: HashMap<GridNode, Entity>,
    pub z_layer: f32,
}

impl Default for Floors {
    fn default() -> Self {
        Self {
            hashmap: HashMap::new(),
            z_layer: 0.0,
        }
    }
}

pub struct Walls {
    pub hashmap: HashMap<GridNode, Entity>,
    pub z_layer: f32,
}

impl Default for Walls {
    fn default() -> Self {
        Self {
            hashmap: HashMap::new(),
            z_layer: 1.0,
        }
    }
}

pub struct Objects {
    pub hashmap: HashMap<GridNode, Entity>,
    pub z_layer: f32,
}

impl Default for Objects {
    fn default() -> Self {
        Self {
            hashmap: HashMap::new(),
            z_layer: 2.0,
        }
    }
}

pub struct Items {
    pub hashmap: HashMap<GridNode, Entity>,
    pub z_layer: f32,
}

impl Default for Items {
    fn default() -> Self {
        Self {
            hashmap: HashMap::new(),
            z_layer: 3.0,
        }
    }
}

#[derive(Resource, Default)]
pub struct Grid {
    pub nodes: HashSet<GridNode>,
    pub floors: Floors,
    pub walls: Walls,
    pub objects: Objects,
    pub items: Items,
}

impl Grid {
    pub fn new(nodes: HashSet<GridNode>) -> Self {
        Self {
            nodes,
            ..Default::default()
        }
    }

    pub fn get_neighbors(&self, node: &GridNode) -> Vec<(GridNode, u32)> {
        let mut neighbors = Vec::new();
        // let &GridNode { x, y } = node;
        let &GridNode(IVec2 { x, y }) = node;

        const STRAIGHT_COST: u32 = 10;
        const DIAGONAL_COST: u32 = 14;

        // TODO: (LOW PRIORITY) Refactor and clean up this code
        let up = GridNode::new(x, y + 1);
        if self.check_neighbor_walkable(&up) {
            neighbors.push((up, STRAIGHT_COST));
        }

        let right = GridNode::new(x + 1, y);
        if self.check_neighbor_walkable(&right) {
            neighbors.push((right, STRAIGHT_COST));
        }

        let down = GridNode::new(x, y - 1);
        if self.check_neighbor_walkable(&down) {
            neighbors.push((down, STRAIGHT_COST));
        }

        let left = GridNode::new(x - 1, y);
        if self.check_neighbor_walkable(&left) {
            neighbors.push((left, STRAIGHT_COST));
        }

        let up_left = GridNode::new(x - 1, y + 1);
        if self.check_neighbor_walkable(&up_left) {
            if neighbors.contains(&(up, STRAIGHT_COST))
                && neighbors.contains(&(left, STRAIGHT_COST))
            {
                neighbors.push((up_left, DIAGONAL_COST));
            }
        }

        let up_right = GridNode::new(x + 1, y + 1);
        if self.check_neighbor_walkable(&up_right) {
            if neighbors.contains(&(up, STRAIGHT_COST))
                && neighbors.contains(&(right, STRAIGHT_COST))
            {
                neighbors.push((up_right, DIAGONAL_COST));
            }
        }

        let down_right = GridNode::new(x + 1, y - 1);
        if self.check_neighbor_walkable(&down_right) {
            if neighbors.contains(&(down, STRAIGHT_COST))
                && neighbors.contains(&(right, STRAIGHT_COST))
            {
                neighbors.push((down_right, DIAGONAL_COST));
            }
        }

        let down_left = GridNode::new(x - 1, y - 1);
        if self.check_neighbor_walkable(&down_left) {
            if neighbors.contains(&(down, STRAIGHT_COST))
                && neighbors.contains(&(left, STRAIGHT_COST))
            {
                neighbors.push((down_left, DIAGONAL_COST));
            }
        }

        neighbors
    }

    fn check_neighbor_walkable(&self, node: &GridNode) -> bool {
        // self.tiles.contains_key(node) && self.tiles[node].walkable
        // TODO:
        true
    }
}
