use crate::driver::i_block::IBlock;

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
}

impl IBlock for FlatBlock {
    fn get_nearest_z(&self, _geo_x: i32, _geo_y: i32, _world_z: i32) -> i32 {
        self.height as i32
    }
}
