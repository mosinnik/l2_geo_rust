// pub mod driver;
// pub mod utils;

// mod geo_benchmark;

use std::fs::File;
use std::io;


use l2_geo_rust::driver::constants::{WORLD_MIN_X, WORLD_MIN_Y};
use l2_geo_rust::utils::{REGION_X, REGION_Y};
use std::io::{BufRead, BufReader};
use std::path::Path;

use l2_geo_rust::{parse_geo_file, GeoDriver};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // println!("----- current dir");
    //
    // let currentDir = PathBuf::from(".");
    // println!("{:?}", currentDir.canonicalize()?);
    //
    // let paths = fs::read_dir(currentDir)?;
    //
    // for path in paths {
    //     println!("Name: {:?}", path?.file_type());
    // }
    //
    // for entry in fs::read_dir(".").unwrap() {
    //     println!("{}", entry?.file_name().to_string_lossy());
    // }
    // println!("----- geodata dir");
    // for entry in fs::read_dir("./geodata/").unwrap() {
    //     println!("{}", entry?.file_name().to_string_lossy());
    // }
    //
    // println!("-----------------");

    let mut geo_driver = GeoDriver::new();
    geo_driver.load_region()?;

    let corner_min_x = REGION_X * 32768 + WORLD_MIN_X;
    let corner_min_y = REGION_Y * 32768 + WORLD_MIN_Y;
    // let corner_min_x = 12 * 32768 + WORLD_MIN_X;
    // let corner_min_y = 24 * 32768 + WORLD_MIN_Y;
    let corner_max_x = corner_min_x + 32768 - 1;
    let corner_max_y = corner_min_y + 32768 - 1;

    println!("corner_min_x = {}", corner_min_x);
    println!("corner_min_y = {}", corner_min_y);
    println!("corner_max_x = {}", corner_max_x);
    println!("corner_max_y = {}", corner_max_y);
    println!("--------------------------");

    //        int x = 24576, y = 49152, z = -4304;
    //        int nearest_zold = oldDriver.getNearestZ(x, y, z);
    //        int nearestZFFM = driver.getNearestZ(x, y, z);
    //        System.out.println("nearest_zold = " + nearest_zold);
    //        System.out.println("nearestZFFM = " + nearestZFFM);

    // let x = 24576;
    // let y = 49152;
    // let z = -4304;
    // let nearest_zold = geo_driver.get_nearest_z(x, y, z);
    // println!("nearest_zold = {}", nearest_zold);


    // Чтение файла
    let records = parse_geo_file("./geodata/tst_points/RANDOM_points.txt")?;


    for (world_x, world_y, geo_x, geo_y, nswe) in records.iter().take(10) {
        // println!("({}, {}, {}, {}, {}, {})", world_x, world_y, geo_x, geo_y, -3000, nswe);
        let z = geo_driver.get_nearest_z(*geo_x, *geo_y, -3000);
        // println!("z = {}", z);
    }


    Ok(())
}


