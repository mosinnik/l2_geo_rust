use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, Throughput, SamplingMode};
use std::path::Path;
use l2_geo_rust::GeoDriver;
use l2_geo_rust::parse_geo_file;


/// Бенчмарк обработки всего файла
fn benchmark_full_file(c: &mut Criterion) {
    let random_records = parse_geo_file("./geodata/tst_points/RANDOM_points.txt")
        .expect("Failed to load geo data 1");
    let flat_records = parse_geo_file("./geodata/tst_points/FLAT_BLOCK_points.txt")
        .expect("Failed to load geo data 2");
    let complex_records = parse_geo_file("./geodata/tst_points/COMPLEX_BLOCK_points.txt")
        .expect("Failed to load geo data 3");
    let multilayer_records = parse_geo_file("./geodata/tst_points/MULTILAYER_BLOCK_points.txt")
        .expect("Failed to load geo data 4");

    println!("Загружено записей random_records: {}", random_records.len());
    println!("Загружено записей flat_records: {}", flat_records.len());
    println!("Загружено записей complex_records: {}", complex_records.len());
    println!("Загружено записей multilayer_records: {}", multilayer_records.len());

    // Инициализация драйвера
    let mut driver = GeoDriver::new();
    driver.load_region()
        .expect("Failed to load_region");

    // Предварительная загрузка регионов (если нужно)
    // driver.load_region(...)?;

    let mut group = c.benchmark_group("geo_driver");
    group.throughput(Throughput::Elements(1));
    group.measurement_time(std::time::Duration::from_secs(10));
    group.warm_up_time(std::time::Duration::from_secs(2));
    group.sample_size(10000);
    // group.sampling_mode(SamplingMode::Flat);

    // Бенчмарк: обработка всех записей
    group.bench_function("get_nearest_z_random", |b| {
        b.iter(|| {
            // let mut total_z = 0;
            for (world_x, world_y, geo_x, geo_y, nswe) in &random_records {
                // let z = driver.get_nearest_z(*geo_x, *geo_y, -3000);
                std::hint::black_box(driver.get_nearest_z(*geo_x, *geo_y, -3000));
                // total_z += z;
            }
            // std::hint::black_box(total_z);
        });
    });
    group.bench_function("get_nearest_z_flat", |b| {
        b.iter(|| {
            // let mut total_z = 0;
            for (world_x, world_y, geo_x, geo_y, nswe) in &flat_records {
                // let z = driver.get_nearest_z(*geo_x, *geo_y, -3000);
                std::hint::black_box(driver.get_nearest_z(*geo_x, *geo_y, -3000));
                // total_z += z;
            }
            // std::hint::black_box(total_z);
        });
    });
    group.bench_function("get_nearest_z_complex", |b| {
        b.iter(|| {
            // let mut total_z = 0;
            for (world_x, world_y, geo_x, geo_y, nswe) in &complex_records {
                // let z = driver.get_nearest_z(*geo_x, *geo_y, -3000);
                std::hint::black_box(driver.get_nearest_z(*geo_x, *geo_y, -3000));
                // total_z += z;
            }
            // std::hint::black_box(total_z);
        });
    });
    group.bench_function("get_nearest_z_multilayer", |b| {
        b.iter(|| {
            // let mut total_z = 0;
            for (world_x, world_y, geo_x, geo_y, nswe) in &multilayer_records {
                // let z = driver.get_nearest_z(*geo_x, *geo_y, -3000);
                std::hint::black_box(driver.get_nearest_z(*geo_x, *geo_y, -3000));
                // total_z += z;
            }
            // std::hint::black_box(total_z);
        });
    });

    group.finish();
}


criterion_group!(
    benches,
    benchmark_full_file,
);
// criterion_group!(name = benches;
//     config = Criterion::default()
//         .without_plots()  // ← Отключаем графики (не нужен gnuplot)
//         .measurement_time(std::time::Duration::from_secs(5));
//     targets = benchmark_full_file
// );

criterion_main!(benches);