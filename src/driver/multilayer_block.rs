use crate::driver::constants::BLOCK_CELLS;
use crate::driver::i_block::IBlock;

/// A block containing multiple layers of geographic data
pub struct MultilayerBlock {
    data: Vec<u8>,
}

impl MultilayerBlock {
    /// Creates a new MultilayerBlock from a byte slice, parsing the header to determine data length.
    ///
    /// # Errors
    /// Returns an error if the data is corrupted (invalid layer counts or insufficient data).
    pub fn from_bytes(bytes: &Vec<u8>, base_offset: usize) -> Result<(MultilayerBlock, usize), String> {
        let mut offset = 0;
        for i in 0..BLOCK_CELLS {
            let n_layers = bytes[base_offset + offset];
            if n_layers <= 0 || n_layers > 125 {
                return Err("L2JGeoDriver: Geo file corrupted! Invalid layers count!".to_string());
            }
            offset += 1 + (n_layers as usize) * 2;
        }

        // Copy the parsed data
        let data = bytes[base_offset..base_offset + offset].to_vec();
        Ok((Self { data }, base_offset + offset))
    }

    pub fn new(data: Vec<u8>) -> Self {
        Self { data }
    }

    fn get_nearest_layer(&self, geo_x: i32, geo_y: i32, world_z: i32) -> u16 {
        let start_offset = self.get_cell_data_offset(geo_x, geo_y);
        let n_layers = self.data[start_offset] as usize;
        let end_offset = start_offset + 1 + n_layers * 2;

        let mut nearest_dz = usize::MAX;
        let mut nearest_data: u16 = 0;

        let mut offset = start_offset + 1;
        while offset < end_offset {
            let layer_data = self.extract_layer_data(offset);
            let layer_z = Self::extract_layer_height(layer_data);

            if layer_z == world_z {
                return layer_data; // Exact match
            }

            let layer_dz = (layer_z - world_z).abs() as usize;
            if layer_dz < nearest_dz {
                nearest_dz = layer_dz;
                nearest_data = layer_data;
            }
            offset += 2;
        }
        nearest_data
    }

    fn get_cell_data_offset(&self, geo_x: i32, geo_y: i32) -> usize {
        let cell_local_offset = (((geo_x & 0x07) << 3) + (geo_y & 0x07)) as usize;
        let mut cell_data_offset = 0;

        for _ in 0..cell_local_offset {
            cell_data_offset += 1 + (self.data[cell_data_offset] as usize) * 2;
        }
        cell_data_offset
    }

    fn extract_layer_data(&self, data_offset: usize) -> u16 {
        (self.data[data_offset] as u16) | ((self.data[data_offset + 1] as u16) << 8)
    }

    fn extract_layer_height(layer: u16) -> i32 {
        ((layer & 0xFFF0) >> 1) as i32
    }
}

impl IBlock for MultilayerBlock {
    fn get_nearest_z(&self, geo_x: i32, geo_y: i32, world_z: i32) -> i32 {
        Self::extract_layer_height(self.get_nearest_layer(geo_x, geo_y, world_z))
    }
}
