use crate::vec3::Vec3;
use std::{fs::File, io::Write};
pub fn write_color(pixel_color: Vec3, file: &mut File) {
    // Write the translated [0,255] value of each color component.
    file.write_all((255.999 * pixel_color.x()).to_string().as_bytes())
        .expect("wrong write");
    file.write_all(b" ").expect("wrong write");
    file.write_all((255.999 * pixel_color.y()).to_string().as_bytes())
        .expect("wrong write");
    file.write_all(b" ").expect("wrong write");
    file.write_all((255.999 * pixel_color.z()).to_string().as_bytes())
        .expect("wrong write");
    file.write_all(b"\n").expect("wrong write");
}
