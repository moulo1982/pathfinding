use game_pathfinding::{map, vec2d, astar::AStar};

fn main() {
    let map_info = vec2d![
        1, 0, 1, 0, 1, 0, 1, 0;
        0, 0, 0, 1, 0, 1, 0, 1;
        1, 0, 1, 0, 1, 0, 1, 0;
        0, 0, 0, 1, 0, 1, 0, 1;
        1, 0, 1, 0, 1, 0, 1, 0;
        0, 0, 0, 1, 0, 1, 0, 1;
        1, 0, 1, 0, 1, 0, 1, 0;
        0, 0, 0, 1, 0, 1, 0, 1;
    ];

    let mut map = map::Map::new_astar();

    map.load(map_info);

    let result = map.find_path(&map::Point::new(1, 0), &map::Point::new(6, 7));

    println!("{:?}", result);
}
