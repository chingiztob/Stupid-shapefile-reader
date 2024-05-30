use shape::*;

fn main() {
    // r"D:\GIS\DIPLOM\Outputs\Chelyabinsk\Shapes\chel_H3_TT.shp"
    let start = std::time::Instant::now();

    let mainfile = MainFile::from(r"D:\Rust\shape\files\demo_1.shp").unwrap();
    let mainfile = MainFile::from(r"D:\Rust\shape\files\demo_big.shp").unwrap();

    println!("file header {:#?}", mainfile.header);
    println!("Geometry type {:#?}", mainfile.geom_type());
    println!("file records {:#?}", mainfile.records.len());
    println!("file records {:#?}", mainfile.records[0]);
    

    let csv = mainfile.to_csv();

    // write to csv
    std::fs::write("output.csv", csv).expect("Unable to write file");

    println!("Time elapsed: {:?}", start.elapsed());
}
