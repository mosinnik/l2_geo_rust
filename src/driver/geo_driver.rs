use crate::driver::region::{IRegion, NullRegion, Region};
use crate::utils;
use std::error::Error;
use crate::driver::constants::{GEO_REGIONS, GEO_REGIONS_Y, REGION_BLOCKS};
use crate::driver::IBlock::IBlock;
use crate::utils::{REGION_X, REGION_Y};

pub struct GeoDriver {
    pub regions: Vec<Box<dyn IRegion>>,
}

impl GeoDriver {
    pub fn new() -> Self {
        let regions = (0..GEO_REGIONS)
            .map(|_| Box::new(NullRegion::new()) as Box<dyn IRegion>)
            .collect();

        Self {
            regions
        }
    }

    // pub fn load_region(&mut self) -> Result<(), Box<dyn Error>> {
    //     let region_data = utils::read_region_data()?;
    //
    //     let region = Region::new(&region_data);
    //     self.regions.push(region);
    //
    //     Ok(())
    // }

    pub fn load_region(&mut self) -> Result<(), Box<dyn Error>> {
        let region_data = utils::read_region_data()?;

        let region = Region::new(&region_data);

        let region_offset = REGION_X * GEO_REGIONS_Y + REGION_Y;
        println!("Region offset: {}", region_offset);
        self.regions[region_offset as usize] = Box::new(region);

        Ok(())
    }

    /// Внутренний метод получения региона по координатам
    /// Java: private IRegion getRegion(int geoX, int geoY)
    fn get_region(&self, geo_x: i32, geo_y: i32) -> &Box<dyn IRegion> {
        // Java: int regionOffset = ((geoX >> 11) << 5) + (geoY >> 11);
        let region_offset = ((geo_x >> 11) << 5) + (geo_y >> 11);
        // println!("region_offset = {}", region_offset);
        // В Rust проверка границ выполняется автоматически (panic при выходе за пределы)
        // Если нужно избежать паники, можно использовать self.regions.get(region_offset)
        &self.regions[region_offset as usize]
    }
}


impl GeoDriver {
    pub fn has_geo_pos(&self, geo_x: i32, geo_y: i32) -> bool {
        self.get_region(geo_x, geo_y).has_geo()
    }

    pub fn check_nearest_nswe(&self, geo_x: i32, geo_y: i32, world_z: i32, nswe: u8) -> bool {
        self.get_region(geo_x, geo_y).check_nearest_nswe(geo_x, geo_y, world_z, nswe)
    }

    pub fn get_nearest_z(&self, geo_x: i32, geo_y: i32, world_z: i32) -> i32 {
        let region = self.get_region(geo_x, geo_y);

        region.get_nearest_z(geo_x, geo_y, world_z)
    }

    pub fn get_next_lower_z(&self, geo_x: i32, geo_y: i32, world_z: i32) -> i32 {
        self.get_region(geo_x, geo_y).get_next_lower_z(geo_x, geo_y, world_z)
    }

    pub fn get_next_higher_z(&self, geo_x: i32, geo_y: i32, world_z: i32) -> i32 {
        self.get_region(geo_x, geo_y).get_next_higher_z(geo_x, geo_y, world_z)
    }

    pub fn get_block(&self, geo_x: i32, geo_y: i32) -> &dyn IBlock {
        self.get_region(geo_x, geo_y).get_block(geo_x, geo_y)
    }
}
