use shape::*;
use std::io;
use std::process;

fn main() {
    let start = std::time::Instant::now();

    let mut path = String::new();
    println!("Enter the path to the shape file:");
    io::stdin()
        .read_line(&mut path)
        .expect("Failed to read input");

    let path = path.trim();

    //let path = r"D:\Rust\shape\files\ne_10m_railroads.shp";
    //let path = r"D:\Rust\shape\files\polylines.shp";

    let mainfile = MainFile::from(path).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("file header {:#?}", mainfile.header);
    println!("Geometry type {:#?}", mainfile.geom_type());
    println!("file records {:#?}", mainfile.records.len());

    if !mainfile.records.is_empty() {
        println!("Seconds record: {:#?}", mainfile.records[1]);
    } else {
        println!("No records found.");
    }

    let csv = mainfile.to_csv();
    //write to csv
    std::fs::write("output.csv", csv).expect("Unable to write file");

    println!("Time elapsed: {:?}", start.elapsed());
}
