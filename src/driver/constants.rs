pub const TYPE_FLAT: u8 = 0;
pub const TYPE_COMPLEX: u8 = 1;
pub const TYPE_MULTILAYER: u8 = 2;

/// Cells in a block on the x axis
pub const BLOCK_CELLS_X: i32 = 8;
/// Cells in a block on the y axis
pub const BLOCK_CELLS_Y: i32 = 8;
/// Cells in a block
pub const BLOCK_CELLS: i32 = BLOCK_CELLS_X * BLOCK_CELLS_Y;

/// Blocks in a region on the x axis
pub const REGION_BLOCKS_X: i32 = 256;
/// Blocks in a region on the y axis
pub const REGION_BLOCKS_Y: i32 = 256;
/// Blocks in a region
pub const REGION_BLOCKS: i32 = REGION_BLOCKS_X * REGION_BLOCKS_Y;

/// Cells in a region on the x axis
// const REGION_CELLS_X: usize = Self::REGION_BLOCKS_X * IBlock::BLOCK_CELLS_X;
pub const REGION_CELLS_X: i32 = REGION_BLOCKS_X * BLOCK_CELLS_Y;
/// Cells in a region on the y axis
pub const REGION_CELLS_Y: i32 = REGION_BLOCKS_Y * BLOCK_CELLS_Y;
/// Cells in a region
pub const REGION_CELLS: i32 = REGION_CELLS_X * REGION_CELLS_Y;

pub const GEO_REGIONS_X: i32 = 32;
pub const GEO_REGIONS_Y: i32 = 32;
pub const GEO_REGIONS: i32 = GEO_REGIONS_X * GEO_REGIONS_Y;

pub const WORLD_MIN_X: i32 = -655360;
pub const WORLD_MAX_X: i32 = 393215;
pub const WORLD_MIN_Y: i32 = -589824;
pub const WORLD_MAX_Y: i32 = 458751;



