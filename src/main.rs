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

    let mainfile = MainFile::from(path).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("file header {:#?}", mainfile.header);
    println!("Geometry type {:#?}", mainfile.geom_type());
    println!("file records {:#?}", mainfile.records.len());
    println!("file records {:#?}", mainfile.records[0]);

    let csv = mainfile.to_csv();
    // write to csv
    std::fs::write("output.csv", csv).expect("Unable to write file");

    println!("Time elapsed: {:?}", start.elapsed());
}
