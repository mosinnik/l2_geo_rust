use crate::driver::complex_block::ComplexBlock;
use crate::driver::constants::*;
use crate::driver::flat_block::FlatBlock;
use crate::driver::i_block::IBlock;
use crate::driver::multilayer_block::MultilayerBlock;
use crate::i_block::BlockImpl;

pub trait IRegion {
    fn get_block(&self, geo_x: i32, geo_y: i32) -> &BlockImpl;

    fn get_nearest_z(&self, geo_x: i32, geo_y: i32, world_z: i32) -> i32;

    fn has_geo(&self) -> bool;
}

pub enum RegionImpl {
    Region(Box<Region>),
    NullRegion(NullRegion),
}

impl IRegion for RegionImpl {
    fn get_block(&self, geo_x: i32, geo_y: i32) -> &BlockImpl {
        match self {
            RegionImpl::Region(r) => r.get_block(geo_x, geo_y),
            RegionImpl::NullRegion(r) => r.get_block(geo_x, geo_y),
        }
    }

    fn get_nearest_z(&self, geo_x: i32, geo_y: i32, world_z: i32) -> i32 {
        match self {
            RegionImpl::Region(r) => r.get_nearest_z(geo_x, geo_y, world_z),
            RegionImpl::NullRegion(r) => r.get_nearest_z(geo_x, geo_y, world_z),
        }
    }

    fn has_geo(&self) -> bool {
        match self {
            RegionImpl::Region(r) => r.has_geo(),
            RegionImpl::NullRegion(r) => r.has_geo(),
        }
    }
}

impl RegionImpl {
    pub fn null() -> Self {
        RegionImpl::NullRegion(NullRegion::new())
    }

    pub fn from_data(data: &Vec<u8>) -> Self {
        RegionImpl::Region(Box::new(Region::new(data)))
    }
}

pub struct NullRegion;

impl NullRegion {
    pub(crate) fn new() -> Self {
        NullRegion
    }
}

impl IRegion for NullRegion {
    fn get_block(&self, _geo_x: i32, _geo_y: i32) -> &BlockImpl {
        // Возвращаем заглушку блока
        unimplemented!()
    }
    fn get_nearest_z(&self, _geo_x: i32, _geo_y: i32, _world_z: i32) -> i32 { 0 }
    fn has_geo(&self) -> bool { false }
}

pub struct Region {
    pub blocks: Vec<BlockImpl>,
}

impl Region {
    pub fn new(data: &Vec<u8>) -> Self {
        let mut blocks: Vec<BlockImpl> = Vec::with_capacity(REGION_BLOCKS as usize);
        let mut base_offset: usize = 0;

        for i in 0..REGION_BLOCKS {
            let block_type = *data.get(base_offset).unwrap();
            base_offset += 1;
            match block_type {
                TYPE_FLAT => {
                    let (block, offset) = FlatBlock::from_bytes(data, base_offset);
                    base_offset = offset;
                    blocks.push(BlockImpl::FlatBlock(Box::new(block)));
                    // println!("{} TYPE_FLAT: {}", i, base_offset);
                }
                TYPE_COMPLEX => {
                    let (block, offset) = ComplexBlock::from_bytes(data, base_offset);
                    base_offset = offset;
                    blocks.push(BlockImpl::ComplexBlock(Box::new(block)));
                    // println!("{} TYPE_COMPLEX: {}", i, base_offset);
                }
                TYPE_MULTILAYER => {
                    let (block, offset) = MultilayerBlock::from_bytes(data, base_offset).unwrap();
                    base_offset = offset;
                    blocks.push(BlockImpl::MultilayerBlock(Box::new(block)));
                    // println!("{} TYPE_MULTILAYER: {}", i, base_offset);
                }
                _ => {
                    panic!("Unknown type: {}, offset: {}, total: {}, blocks: {}",
                           block_type, base_offset, data.len(), blocks.len()
                    );
                }
            }
        }

        Region { blocks }
    }

    pub fn get_block_by_offset(&self, block_offset: usize) -> &BlockImpl {
        &self.blocks[block_offset]
    }

    fn get_block_offset(geo_x: i32, geo_y: i32) -> usize {
        (((geo_x >> 3) & 0xFF) << 8) as usize + ((geo_y >> 3) & 0xFF) as usize
    }
}

impl IRegion for Region {
    fn get_block(&self, geo_x: i32, geo_y: i32) -> &BlockImpl {
        let block_offset = Self::get_block_offset(geo_x, geo_y);
        &self.blocks[block_offset]
    }

    fn get_nearest_z(&self, geo_x: i32, geo_y: i32, world_z: i32) -> i32 {
        let block = self.get_block(geo_x, geo_y);
        block.get_nearest_z(geo_x, geo_y, world_z)
    }

    fn has_geo(&self) -> bool {
        true
    }
}
