use crate::driver::constants::BLOCK_CELLS;
use crate::driver::IBlock::IBlock;

pub struct ComplexBlock {
    data: Vec<i16>,
}

impl ComplexBlock {
    pub fn from_bytes(bytes: &Vec<u8>, base_offset: usize) -> (ComplexBlock, usize) {
        let mut data = Vec::with_capacity(BLOCK_CELLS as usize);
        for i in 0..BLOCK_CELLS as usize {
            let height = i16::from_le_bytes([
                bytes[base_offset + i * 2],
                bytes[base_offset + i * 2 + 1]
            ]);
            data.push(height);
        }
        (ComplexBlock { data }, base_offset + 2 * BLOCK_CELLS as usize)
    }

    /// Конструктор из вектора short
    /// Аналог: public ComplexBlock(short[] tmpData)
    pub fn new(data: Vec<i16>) -> Self {
        ComplexBlock { data }
    }

    /// Геттер данных
    /// Аналог: public short[] getData()
    pub fn get_data(&self) -> &[i16] {
        &self.data
    }

    /// Внутренний метод: получение NSWE данных ячейки
    /// Java: private int getCellNSWE(int geoX, int geoY)
    fn get_cell_nswe(&self, geo_x: i32, geo_y: i32) -> i32 {
        let cell_offset = Self::get_cell_offset(geo_x, geo_y);
        let datum = self.data[cell_offset];
        (datum as i32) & 0x0F
    }

    /// Внутренний метод: получение высоты ячейки
    /// Java: private int getCellHeight(int geoX, int geoY)
    fn get_cell_height(&self, geo_x: i32, geo_y: i32) -> i32 {
        let cell_offset = Self::get_cell_offset(geo_x, geo_y);
        let short_val = self.data[cell_offset];
        ((short_val as i32) >> 1) & 0x7FFFFFFE
    }

    /// Вспомогательный метод: вычисление смещения ячейки
    fn get_cell_offset(geo_x: i32, geo_y: i32) -> usize {
        (((geo_x & 0x07) << 3) + (geo_y & 0x07)) as usize
    }
}

impl IBlock for ComplexBlock {
    fn check_nearest_nswe(&self, geo_x: i32, geo_y: i32, _world_z: i32, nswe: u8) -> bool {
        let cell_nswe = self.get_cell_nswe(geo_x, geo_y);
        (cell_nswe & nswe as i32) == nswe as i32
    }

    fn get_nearest_z(&self, geo_x: i32, geo_y: i32, _world_z: i32) -> i32 {
        self.get_cell_height(geo_x, geo_y)
    }

    fn get_next_lower_z(&self, geo_x: i32, geo_y: i32, world_z: i32) -> i32 {
        let cell_height = self.get_cell_height(geo_x, geo_y);
        cell_height.min(world_z)
    }

    fn get_next_higher_z(&self, geo_x: i32, geo_y: i32, world_z: i32) -> i32 {
        let cell_height = self.get_cell_height(geo_x, geo_y);
        cell_height.max(world_z)
    }
}
