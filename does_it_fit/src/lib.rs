mod areas_volumes;

pub use crate::areas_volumes::{
    GeometricalShapes, GeometricalVolumes, circle_area, cone_volume, cube_volume,
    parallelepiped_volume, rectangle_area, sphere_volume, square_area, triangle_area,
    triangular_pyramid_volume,
};

pub fn area_fit(
    x: usize,
    y: usize,
    objects: areas_volumes::GeometricalShapes,
    times: usize,
    a: usize,
    b: usize,
) -> bool {
    let area = x * y;
    let shape_area = match objects {
        GeometricalShapes::Square => areas_volumes::square_area(a),
        GeometricalShapes::Rectangle => areas_volumes::rectangle_area(a, b),
        GeometricalShapes::Triangle => areas_volumes::triangle_area(a, b) as usize,
        GeometricalShapes::Circle => areas_volumes::circle_area(a) as usize,
    };
    area >= shape_area * times
}
pub fn volume_fit(
    x: usize,
    y: usize,
    z: usize,
    objects: areas_volumes::GeometricalVolumes,
    times: usize,
    a: usize,
    b: usize,
    c: usize,
) -> bool {
    let volume = x * y * z;
    let shape_volume = match objects {
        GeometricalVolumes::Cube => areas_volumes::cube_volume(a),
        GeometricalVolumes::Sphere => areas_volumes::sphere_volume(a) as usize,
        GeometricalVolumes::Cone => areas_volumes::cone_volume(a, b) as usize,
        GeometricalVolumes::Pyramid => {
            areas_volumes::triangular_pyramid_volume(a as f64, b) as usize
        }
        GeometricalVolumes::Parallelepiped => areas_volumes::parallelepiped_volume(a, b, c),
    };
    volume >= shape_volume * times
}
