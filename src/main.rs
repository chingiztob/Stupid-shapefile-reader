use shape::MainFile;
use std::io;
use std::process;

fn main() {
    let mut path = String::new();
    println!("Enter the path to the shape file:");
    io::stdin()
        .read_line(&mut path)
        .expect("Failed to read input");

    // Remove double quotes before and after the file path
    let path = path.replace('"', "").trim().to_string();

    let bench_start = std::time::Instant::now();
    let mainfile = MainFile::from(&path).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    println!("File read in : {:?}", bench_start.elapsed());

    println!("file header {:#?}", mainfile.header);
    println!("Geometry type {:#?}", mainfile.geom_type());
    println!("file records {:#?}", mainfile.records.len());

    // Write to csv
    let mut write_csv = String::new();
    println!("Write to csv? (y/n)");
    io::stdin()
        .read_line(&mut write_csv)
        .expect("Failed to read input");

    if write_csv.trim().eq_ignore_ascii_case("y") {
        println!("Writing to csv...");
        mainfile.to_csv("output.csv").unwrap_or_else(|err| {
            eprintln!("Problem writing to csv: {err}");
            process::exit(1);
        });
    }

    println!("Done!");
}
