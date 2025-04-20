mod areas_volumes;
use areas_volumes::{GeometricalShapes, GeometricalVolumes};

pub fn area_fit(
    (x, y): (usize, usize),
    kind: GeometricalShapes,
    times: usize,
    (a, b): (usize, usize),
) -> bool {
    let area_available = x * y;
    let area_needed = match kind {
        GeometricalShapes::Square => areas_volumes::square_area(a) as f64,
        GeometricalShapes::Rectangle => areas_volumes::rectangle_area(a, b) as f64,
        GeometricalShapes::Triangle => areas_volumes::triangle_area(a, b),
        GeometricalShapes::Circle => areas_volumes::circle_area(a),
    };

    area_needed * times as f64 <= area_available as f64
}

pub fn volume_fit(
    (x, y, z): (usize, usize, usize),
    kind: GeometricalVolumes,
    times: usize,
    (a, b, c): (usize, usize, usize),
) -> bool {
    let volume_available = (x * y * z) as f64;
    let volume_needed = match kind {
        GeometricalVolumes::Cube => areas_volumes::cube_volume(a) as f64,
        GeometricalVolumes::Sphere => areas_volumes::sphere_volume(a),
        GeometricalVolumes::TriangularPyramid => {
            let base_area = a as f64; // base_area is passed directly
            areas_volumes::triangular_pyramid_volume(base_area, b)
        }
        GeometricalVolumes::Parallelepiped => areas_volumes::parallelepiped_volume(a, b, c) as f64,
        GeometricalVolumes::Cone => areas_volumes::cone_volume(a, b),
    };

    volume_needed * times as f64 <= volume_available
}
