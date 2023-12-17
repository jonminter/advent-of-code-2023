mod map {
    use std::collections::HashMap;

    #[derive(PartialEq, Eq, Hash, Clone, Debug)]
    pub(crate) enum MoveDir {
        Up,
        Left,
        Down,
        Right,
    }
    impl MoveDir {
        fn move_from(&self, x: i64, y: i64) -> (i64, i64) {
            match self {
                MoveDir::Up => (x, y.saturating_sub(1)),
                MoveDir::Down => (x, y.saturating_add(1)),
                MoveDir::Left => (x.saturating_sub(1), y),
                MoveDir::Right => (x.saturating_add(1), y),
            }
        }
    }

    #[derive(Copy, Clone, Debug, PartialEq)]
    pub(crate) enum Tile {
        Ground,
        Start,
        Vertical,
        Horizontal,
        NorthAndEast,
        NorthAndWest,
        SouthAndEast,
        SouthAndWest,
    }

    impl Tile {
        pub(crate) fn from_char(c: char) -> Result<Tile, String> {
            match c {
                '.' => Ok(Tile::Ground),
                'S' => Ok(Tile::Start),
                '|' => Ok(Tile::Vertical),
                '-' => Ok(Tile::Horizontal),
                'F' => Ok(Tile::SouthAndEast),
                '7' => Ok(Tile::SouthAndWest),
                'J' => Ok(Tile::NorthAndWest),
                'L' => Ok(Tile::NorthAndEast),
                _ => Err(format!("Invalid char '{}'", c)),
            }
        }

        fn next_dir(&self, last_dir: MoveDir) -> Option<MoveDir> {
            match (last_dir, self) {
                (MoveDir::Up, Self::Vertical) => Some(MoveDir::Up),
                (MoveDir::Down, Self::Vertical) => Some(MoveDir::Down),

                (MoveDir::Left, Self::Horizontal) => Some(MoveDir::Left),
                (MoveDir::Right, Self::Horizontal) => Some(MoveDir::Right),

                (MoveDir::Down, Self::NorthAndEast) => Some(MoveDir::Right),
                (MoveDir::Left, Self::NorthAndEast) => Some(MoveDir::Up),

                (MoveDir::Down, Self::NorthAndWest) => Some(MoveDir::Left),
                (MoveDir::Right, Self::NorthAndWest) => Some(MoveDir::Up),

                (MoveDir::Up, Self::SouthAndEast) => Some(MoveDir::Right),
                (MoveDir::Left, Self::SouthAndEast) => Some(MoveDir::Down),

                (MoveDir::Up, Self::SouthAndWest) => Some(MoveDir::Left),
                (MoveDir::Right, Self::SouthAndWest) => Some(MoveDir::Down),

                (_, Self::Start) => unreachable!("BUG: Should not be moving from start!"),
                _ => None,
            }
        }
    }

    #[derive(PartialEq, Debug)]
    pub(crate) struct TileMap {
        tiles: Vec<Vec<Tile>>,
        map_width: i64,
        map_height: i64,
        start_tile: (i64, i64),
    }

    #[derive(Debug)]
    struct MapTraversal {
        x: i64,
        y: i64,
        last_dir: MoveDir,
        steps_from_start: usize,
    }
    impl MapTraversal {
        fn coords(&self) -> (i64, i64) {
            (self.x, self.y)
        }

        fn same_coords(&self, other: &MapTraversal) -> bool {
            self.x == other.x && self.y == other.y
        }

        fn move_to_next_pos(self, &cur_tile: &Tile) -> Option<MapTraversal> {
            cur_tile.next_dir(self.last_dir).map(|next_dir| {
                let (new_x, new_y) = next_dir.move_from(self.x, self.y);
                Self {
                    x: new_x,
                    y: new_y,
                    last_dir: next_dir,
                    steps_from_start: self.steps_from_start + 1,
                }
            })
        }
    }

    enum TraverseResult {
        FoundLoop(usize),
        Continue(MapTraversal),
        EndOfPath,
    }

    impl TileMap {
        fn tile_at(&self, x: i64, y: i64) -> Option<&Tile> {
            if x < 0 || y < 0 {
                return None;
            }
            self.tiles
                .get(y as usize)
                .and_then(|row| row.get(x as usize))
        }

        fn traversal_in_bounds(&self, traversal: &MapTraversal) -> bool {
            traversal.x >= 0
                && traversal.y >= 0
                && traversal.x < self.map_width
                && traversal.y < self.map_height
        }

        fn get_first_moves_and_dirs(&self) -> HashMap<(i64, i64), MapTraversal> {
            let _ = self
                .tile_at(self.start_tile.0, self.start_tile.1)
                .expect("BUG: We should have checked validity of start tile already!");

            let (start_x, start_y) = self.start_tile;
            [
                ((start_x, start_y - 1), MoveDir::Up),
                ((start_x, start_y + 1), MoveDir::Down),
                ((start_x - 1, start_y), MoveDir::Left),
                ((start_x + 1, start_y), MoveDir::Right),
            ]
            .into_iter()
            .flat_map(|((x, y), last_dir)| {
                self.tile_at(x, y).map(|_| {
                    (
                        (x, y),
                        MapTraversal {
                            x,
                            y,
                            last_dir,
                            steps_from_start: 1,
                        },
                    )
                })
            })
            .collect()
        }

        fn check_for_loop_and_return_steps<'a>(
            traversal: &MapTraversal,
            mut other_traversals: impl Iterator<Item = &'a MapTraversal>,
        ) -> Option<usize> {
            other_traversals
                .find(|other| other.same_coords(traversal))
                .map(|other| traversal.steps_from_start + other.steps_from_start)
        }

        fn move_to_next_tile<'a>(
            &self,
            traversal: MapTraversal,
            other_traversals: impl Iterator<Item = &'a MapTraversal>,
        ) -> TraverseResult {
            let cur_tile = self
                .tile_at(traversal.x, traversal.y)
                .expect("BUG: Should have tile at current traversal pos!");

            match traversal
                .move_to_next_pos(cur_tile)
                .filter(|traversal| self.traversal_in_bounds(traversal))
            {
                Some(continued_traversal) => {
                    let new_tile = self
                        .tile_at(continued_traversal.x, continued_traversal.y)
                        .expect("BUG: Should not have continued traversal if coords not in bounds");

                    if matches!(new_tile, Tile::Start) {
                        return TraverseResult::FoundLoop(continued_traversal.steps_from_start);
                    }

                    match Self::check_for_loop_and_return_steps(
                        &continued_traversal,
                        other_traversals,
                    ) {
                        Some(combined_steps) => TraverseResult::FoundLoop(combined_steps),
                        None => TraverseResult::Continue(continued_traversal),
                    }
                }
                None => TraverseResult::EndOfPath,
            }
        }

        pub(crate) fn steps_till_furthest_from_start(&self) -> usize {
            // From the start we can only go in up to 4 dirs, and from each path there is only one choice at each step
            // Two of these will eventually meet and form the loop
            let mut traversals = self.get_first_moves_and_dirs();

            // For each of the initial start points follow the path until we reach a point where we can't go anywhere
            // or we find the other part of the loop
            while !traversals.is_empty() {
                let keys = traversals.keys().cloned().collect::<Vec<_>>();
                for key in keys {
                    let traversal = traversals.remove(&key).unwrap();
                    match self.move_to_next_tile(traversal, traversals.values()) {
                        TraverseResult::FoundLoop(steps) => return steps.div_ceil(2),
                        TraverseResult::Continue(continued_traversal) => {
                            traversals.insert(continued_traversal.coords(), continued_traversal);
                        }
                        TraverseResult::EndOfPath => {}
                    }
                }
            }

            unreachable!("BUG: Should have found a loop!")
        }

        pub(crate) fn new(
            tiles: Vec<Vec<Tile>>,
            start_tile: (i64, i64),
        ) -> Result<TileMap, String> {
            if tiles.is_empty() {
                return Err("Map cannot be empty!".to_string());
            }
            if tiles.iter().any(|row| row.is_empty()) {
                return Err("Row cannot be empty!".to_string());
            }

            let map_height = tiles.len();

            let map_width = tiles.first().unwrap().len();

            if tiles.iter().any(|row| row.len() != map_width) {
                return Err("All rows in map must be the same length!".to_string());
            }
            if start_tile.1 as usize >= tiles.len() {
                return Err("Start tile must be less than map height!".to_string());
            }
            if start_tile.0 as usize >= tiles.get(0).unwrap().len() {
                return Err("Start tile must be less than map width!".to_string());
            }

            Ok(TileMap {
                tiles,
                start_tile,
                map_width: map_width
                    .try_into()
                    .map_err(|_| "Map width i64 overflow".to_string())?,
                map_height: map_height
                    .try_into()
                    .map_err(|_| "Map height i64 overflow".to_string())?,
            })
        }
    }
}

mod parse {
    use crate::map::Tile;
    use crate::map::TileMap;

    pub(crate) fn parse_tile_map(lines: impl Iterator<Item = String>) -> Result<TileMap, String> {
        let mut start_x: i64 = -1;
        let mut start_y: i64 = -1;
        let mut tiles = vec![];

        for (y, line) in lines.enumerate() {
            let row = line
                .chars()
                .enumerate()
                .filter(|(_, c)| *c != '\n')
                .map(|(x, c)| {
                    Tile::from_char(c).and_then(|tile| {
                        if matches!(tile, Tile::Start) {
                            start_x = x.try_into().map_err(|_| "Start x overflow")?;
                            start_y = y.try_into().map_err(|_| "Start y overflow")?;
                        }
                        Ok(tile)
                    })
                })
                .collect::<Result<Vec<_>, String>>()?;
            tiles.push(row);
        }

        if start_x < 0 || start_y < 0 {
            Err("Could not find start".to_string())
        } else {
            TileMap::new(tiles, (start_x, start_y))
        }
    }
}
fn main() {
    let lines = std::io::stdin()
        .lines()
        .map(|line| line.expect("Failed to get line from stdin"));
    let map = parse::parse_tile_map(lines).expect("Failed parsing input");

    println!("Steps: {}", map.steps_till_furthest_from_start());
}

#[cfg(test)]
mod test {

    use crate::{
        map::{Tile, TileMap},
        parse::parse_tile_map,
    };

    fn tile_map<const W: usize, const H: usize>(
        tiles: [[Tile; W]; H],
        start: (i64, i64),
    ) -> TileMap {
        let mut tile_map = vec![];

        for row in tiles {
            let mut tile_row = vec![];
            for tile in row {
                tile_row.push(tile);
            }
            tile_map.push(tile_row);
        }
        TileMap::new(tile_map, start).unwrap()
    }

    fn get_test_cases() -> Vec<(&'static str, TileMap, usize)> {
        vec![(
            r#".....
.S-7.
.|.|.
.L-J.
....."#,
            tile_map(
                [
                    [Tile::Ground; 5],
                    [
                        Tile::Ground,
                        Tile::Start,
                        Tile::Horizontal,
                        Tile::SouthAndWest,
                        Tile::Ground,
                    ],
                    [
                        Tile::Ground,
                        Tile::Vertical,
                        Tile::Ground,
                        Tile::Vertical,
                        Tile::Ground,
                    ],
                    [
                        Tile::Ground,
                        Tile::NorthAndEast,
                        Tile::Horizontal,
                        Tile::NorthAndWest,
                        Tile::Ground,
                    ],
                    [Tile::Ground; 5],
                ],
                (1, 1),
            ),
            4,
        )]
    }

    #[test]
    fn test_parse() {
        for (input, expected_map, _) in get_test_cases() {
            let parsed_map = parse_tile_map(input.split('\n').map(|s| s.to_string())).unwrap();
            assert_eq!(parsed_map, expected_map);
        }
    }

    #[test]
    fn test_calculates_steps_properly() {
        for (_, map, expected_steps) in get_test_cases() {
            assert_eq!(map.steps_till_furthest_from_start(), expected_steps);
        }
    }
}
