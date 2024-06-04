use shape::*;
use std::io;
use std::process;

fn main() {
    let mut path = String::new();
    println!("Enter the path to the shape file:");
    io::stdin()
        .read_line(&mut path)
        .expect("Failed to read input");

    let mainfile = MainFile::from(path.trim()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("file header {:#?}", mainfile.header);
    println!("Geometry type {:#?}", mainfile.geom_type());
    println!("file records {:#?}", mainfile.records.len());

    if !mainfile.records.is_empty() {
        println!("First record: {:#?}", mainfile.records[0]);
    } else {
        println!("No records found.");
    }

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
        })
    }

    println!("Done!");
}
