use shape::*;

fn main() {
    // r"D:\GIS\DIPLOM\Outputs\Chelyabinsk\Shapes\chel_H3_TT.shp"

    let mainfile = MainFile::from(r"D:\Rust\shape\files\demo_1.shp").unwrap();
    let mainfile = MainFile::from(r"D:\GIS\DIPLOM\Outputs\Chelyabinsk\Shapes\chel_H3_TT.shp").unwrap();

    println!("file records {:#?}", mainfile.records);
}
