mod vec3;

fn main() {
    let u: vec3::Vec3 = vec3::Vec3 { x: 1.0, y: 2.0, z: 3.0 };
    let v: vec3::Vec3 = vec3::Vec3 { x: 4.0, y: 5.0, z: 6.0 };
    println!("{}", u + v);
}
