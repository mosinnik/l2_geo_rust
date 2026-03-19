
pub trait IBlock {
    fn check_nearest_nswe(&self, geo_x: i32, geo_y: i32, world_z: i32, nswe: u8) -> bool;

    fn get_nearest_z(&self, geo_x: i32, geo_y: i32, world_z: i32) -> i32;

    fn get_next_lower_z(&self, geo_x: i32, geo_y: i32, world_z: i32) -> i32;

    fn get_next_higher_z(&self, geo_x: i32, geo_y: i32, world_z: i32) -> i32;
}
