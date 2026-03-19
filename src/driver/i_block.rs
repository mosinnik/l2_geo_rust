pub trait IBlock {
    fn get_nearest_z(&self, geo_x: i32, geo_y: i32, world_z: i32) -> i32;
}
