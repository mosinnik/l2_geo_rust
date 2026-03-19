use crate::driver::constants::{GEO_REGIONS, GEO_REGIONS_Y};
use crate::driver::region::{IRegion, NullRegion, Region};
use crate::utils;
use crate::utils::{REGION_X, REGION_Y};
use std::error::Error;

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

    pub fn load_region(&mut self) -> Result<(), Box<dyn Error>> {
        let region_data = utils::read_region_data()?;

        let region = Region::new(&region_data);

        let region_offset = REGION_X * GEO_REGIONS_Y + REGION_Y;
        println!("Region offset: {}", region_offset);
        self.regions[region_offset as usize] = Box::new(region);

        Ok(())
    }

    fn get_region(&self, geo_x: i32, geo_y: i32) -> &Box<dyn IRegion> {
        // Java: int regionOffset = ((geoX >> 11) << 5) + (geoY >> 11);
        let region_offset = ((geo_x >> 11) << 5) + (geo_y >> 11);
        &self.regions[region_offset as usize]
    }
}


impl GeoDriver {
    pub fn get_nearest_z(&self, geo_x: i32, geo_y: i32, world_z: i32) -> i32 {
        let region = self.get_region(geo_x, geo_y);
        region.get_nearest_z(geo_x, geo_y, world_z)
    }
}
