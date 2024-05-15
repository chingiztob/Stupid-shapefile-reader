use shape::read_shape;

fn main() {
    println!("Hello, world!");
    let bytes = read_shape(r"D:\GIS\DIPLOM\Outputs\Chelyabinsk\Shapes\chel_H3_TT.shp").unwrap();

    println!("file contents {:#?}", bytes);
    println!("file geom type {:#?}", bytes.geom_type());
}
