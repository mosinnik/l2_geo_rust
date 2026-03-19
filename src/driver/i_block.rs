use crate::complex_block::ComplexBlock;
use crate::flat_block::FlatBlock;
use crate::multilayer_block::MultilayerBlock;

pub trait IBlock {
    fn get_nearest_z(&self, geo_x: i32, geo_y: i32, world_z: i32) -> i32;
}


pub enum BlockImpl {
    FlatBlock(Box<FlatBlock>),
    ComplexBlock(Box<ComplexBlock>),
    MultilayerBlock(Box<MultilayerBlock>),
}

impl IBlock for BlockImpl {
    fn get_nearest_z(&self, geo_x: i32, geo_y: i32, world_z: i32) -> i32 {
        match self {
            BlockImpl::FlatBlock(b) => b.get_nearest_z(geo_x, geo_y, world_z),
            BlockImpl::ComplexBlock(b) => b.get_nearest_z(geo_x, geo_y, world_z),
            BlockImpl::MultilayerBlock(b) => b.get_nearest_z(geo_x, geo_y, world_z),
        }
    }
}

impl BlockImpl {}