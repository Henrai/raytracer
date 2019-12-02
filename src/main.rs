mod geometry;
use crate::geometry::vector::Vec3D;
fn main() {
    let x = Vec3D{x:3.0, y: 4.0, z:5.0};
    let y = Vec3D{x:2.0, y: 2.0, z:2.0};
    println!("{}", (x+y).length());
}
