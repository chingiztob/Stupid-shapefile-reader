use shape::*;

fn main() {
    // r"D:\GIS\DIPLOM\Outputs\Chelyabinsk\Shapes\chel_H3_TT.shp"

    let mainfile = MainFile::from(r"D:\Rust\shape\files\demo_1.shp").unwrap();
    //let mainfile = MainFile::from(r"D:\GIS\DIPLOM\Outputs\Chelyabinsk\Shapes\chel_H3_TT.shp").unwrap();

    println!("file header {:#?}", mainfile.header);
    println!("file records {:#?}", mainfile.records);

    let csv = mainfile.to_csv();
    println!("{}", csv);

    // write to csv
    std::fs::write("output.csv", csv).expect("Unable to write file");
}
