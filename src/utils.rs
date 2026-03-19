use anyhow::{Context, Error};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;

pub const TST_BLOCK_RESOURCE_MOST_COMPLEX: &str = "./geodata/23_12_mostcomplex.l2j";
pub const REGION_X: i32 = 23;
pub const REGION_Y: i32 = 12;
// pub const REGION_X: i32 = 12;
// pub const REGION_Y: i32 = 24;

pub fn read_region_data() -> Result<Vec<u8>, Error> {
    println!(
        "Reading region data from file {}",
        TST_BLOCK_RESOURCE_MOST_COMPLEX
    );
    let file =
        File::open(TST_BLOCK_RESOURCE_MOST_COMPLEX).context("failed to read data from file")?;
    let mut reader = BufReader::new(file);
    let mut data = Vec::new();
    reader
        .read_to_end(&mut data)
        .context("failed to read data from file")?;

    println!("Read {} bytes", data.len());
    Ok(data)
}



/// Парсинг файла с гео-данными
/// Возвращает вектор кортежей: (geo_x, geo_y, world_z, nswe, block_type)
pub fn parse_geo_file<P: AsRef<Path>>(path: P) -> io::Result<Vec<(i32, i32, i32, i32, i32)>> {
    let file = File::open(path.as_ref())?;
    let reader = BufReader::new(file);

    let mut records = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();

        // Пропускаем пустые строки и комментарии
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let parts: Vec<&str> = line.split(';').collect();

        if parts.len() == 5 {
            if let Ok(record) = parse_line(&parts) {
                records.push(record);
            }
        }
    }

    Ok(records)
}

/// Парсинг одной строки в кортеж
fn parse_line(parts: &[&str]) -> Result<(i32, i32, i32, i32, i32), ()> {
    Ok((
        parts[0].parse().map_err(|_| ())?,
        parts[1].parse().map_err(|_| ())?,
        parts[2].parse().map_err(|_| ())?,
        parts[3].parse().map_err(|_| ())?,
        parts[4].parse().map_err(|_| ())?,
    ))
}