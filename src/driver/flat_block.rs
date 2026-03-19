use crate::driver::IBlock::IBlock;

pub struct FlatBlock {
    height: i16,
}

impl FlatBlock {
    pub fn from_bytes(bytes: &Vec<u8>, base_offset: usize) -> (FlatBlock, usize) {
        let height = i16::from_le_bytes([bytes[base_offset], bytes[base_offset + 1]]);
        (FlatBlock { height }, base_offset + 2)
    }

    pub fn new(height: i16) -> Self {
        FlatBlock { height }
    }

    /// Геттер высоты
    /// Аналог: public short getHeight()
    pub fn get_height(&self) -> i16 {
        self.height
    }
}

impl IBlock for FlatBlock {
    fn check_nearest_nswe(&self, _geo_x: i32, _geo_y: i32, _world_z: i32, _nswe: u8) -> bool {
        true
    }

    fn get_nearest_z(&self, _geo_x: i32, _geo_y: i32, _world_z: i32) -> i32 {
        self.height as i32
    }

    fn get_next_lower_z(&self, _geo_x: i32, _geo_y: i32, world_z: i32) -> i32 {
        let h = self.height as i32;
        if h <= world_z { h } else { world_z }
    }

    fn get_next_higher_z(&self, _geo_x: i32, _geo_y: i32, world_z: i32) -> i32 {
        let h = self.height as i32;
        if h >= world_z { h } else { world_z }
    }
}
