use crate::driver::constants::BLOCK_CELLS;
use crate::driver::IBlock::IBlock;

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

    /// Creates a new MultilayerBlock from an existing data vector.
    ///
    /// # Note
    /// This constructor assumes the data is already valid and properly formatted.
    pub fn new(data: Vec<u8>) -> Self {
        Self { data }
    }

    /// Returns the length of the internal data buffer.
    pub fn data_length(&self) -> usize {
        self.data.len()
    }

    /// Returns a reference to the internal data buffer.
    pub fn data(&self) -> &[u8] {
        &self.data
    }

    /// Finds the minimum height across all cells in the block.
    pub fn get_min_height(&self) -> i32 {
        let mut cell_layers_index = 0;
        let mut min_height = i32::MAX;

        for _ in 0..BLOCK_CELLS {
            let n_layers = self.data[cell_layers_index] as usize;
            // Get the lowest layer (last in the list)
            let lowest_layer_data =
                self.extract_layer_data(cell_layers_index + 1 + 2 * (n_layers - 1));
            let layer_height = Self::extract_layer_height(lowest_layer_data);
            min_height = min_height.min(layer_height);
            cell_layers_index += 1 + 2 * n_layers;
        }
        min_height
    }

    /// Finds the maximum height across all cells in the block.
    pub fn get_max_height(&self) -> i32 {
        let mut cell_layers_index = 0;
        let mut max_height = i32::MIN;

        for _ in 0..BLOCK_CELLS {
            let n_layers = self.data[cell_layers_index] as usize;
            // Get the highest layer (first in the list)
            let highest_layer_data = self.extract_layer_data(cell_layers_index + 1);
            let layer_height = Self::extract_layer_height(highest_layer_data);
            max_height = max_height.max(layer_height);
            cell_layers_index += 1 + 2 * n_layers;
        }
        max_height
    }

    /// Gets the layer data closest to the specified world Z coordinate for a cell.
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

    /// Calculates the byte offset in the data array for the specified cell coordinates.
    fn get_cell_data_offset(&self, geo_x: i32, geo_y: i32) -> usize {
        let cell_local_offset = (((geo_x & 0x07) << 3) + (geo_y & 0x07)) as usize;
        let mut cell_data_offset = 0;

        for _ in 0..cell_local_offset {
            cell_data_offset += 1 + (self.data[cell_data_offset] as usize) * 2;
        }
        cell_data_offset
    }

    /// Extracts a 16-bit layer data value from two consecutive bytes (little-endian).
    fn extract_layer_data(&self, data_offset: usize) -> u16 {
        (self.data[data_offset] as u16) | ((self.data[data_offset + 1] as u16) << 8)
    }

    /// Gets the NSWE flags from a layer data value.
    fn get_nearest_nswe(&self, geo_x: i32, geo_y: i32, world_z: i32) -> u8 {
        let nearest_layer = self.get_nearest_layer(geo_x, geo_y, world_z);
        Self::extract_layer_nswe(nearest_layer)
    }

    /// Extracts the NSWE flags (lower 4 bits) from a layer data value.
    fn extract_layer_nswe(layer: u16) -> u8 {
        (layer & 0x000F) as u8
    }

    /// Extracts the height value from a layer data value.
    ///
    /// The height is stored in bits 4-15, shifted right by 1.
    fn extract_layer_height(layer: u16) -> i32 {
        ((layer & 0xFFF0) >> 1) as i32
    }
}

impl IBlock for MultilayerBlock {
    fn check_nearest_nswe(&self, geo_x: i32, geo_y: i32, world_z: i32, nswe: u8) -> bool {
        (self.get_nearest_nswe(geo_x, geo_y, world_z) & nswe) == nswe
    }

    fn get_nearest_z(&self, geo_x: i32, geo_y: i32, world_z: i32) -> i32 {
        Self::extract_layer_height(self.get_nearest_layer(geo_x, geo_y, world_z))
    }

    fn get_next_lower_z(&self, geo_x: i32, geo_y: i32, world_z: i32) -> i32 {
        let start_offset = self.get_cell_data_offset(geo_x, geo_y);
        let n_layers = self.data[start_offset] as usize;
        let end_offset = start_offset + 1 + n_layers * 2;

        let mut lower_z = i32::MIN;
        let mut offset = start_offset + 1;
        while offset < end_offset {
            let layer_data = self.extract_layer_data(offset);
            let layer_z = Self::extract_layer_height(layer_data);

            if layer_z == world_z {
                return layer_z; // Exact match
            }

            if layer_z < world_z && layer_z > lower_z {
                lower_z = layer_z;
            }
            offset += 2;
        }
        if lower_z == i32::MIN {
            world_z
        } else {
            lower_z
        }
    }

    fn get_next_higher_z(&self, geo_x: i32, geo_y: i32, world_z: i32) -> i32 {
        let start_offset = self.get_cell_data_offset(geo_x, geo_y);
        let n_layers = self.data[start_offset] as usize;
        let end_offset = start_offset + 1 + n_layers * 2;

        let mut higher_z = i32::MAX;
        let mut offset = start_offset + 1;
        while offset < end_offset {
            let layer_data = self.extract_layer_data(offset);
            let layer_z = Self::extract_layer_height(layer_data);

            if layer_z == world_z {
                return layer_z; // Exact match
            }

            if layer_z > world_z && layer_z < higher_z {
                higher_z = layer_z;
            }
            offset += 2;
        }
        if higher_z == i32::MAX {
            world_z
        } else {
            higher_z
        }
    }
}
