use ultraviolet::{Vec3, Vec4};

use crate::wavefront_parser::Vertex;

pub fn set_clear_color(col: Vec4) {
    unsafe {
        gl::ClearColor(col.x, col.y, col.z, col.w);
    }
}

/// Polygon display modes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolygonMode {
    /// Only render points
    Point = gl::POINT as isize,
    /// Only render edges
    Line = gl::LINE as isize,
    /// Render filled polygons
    Fill = gl::FILL as isize,
}

/// Set the front/back polygon mode to the one provided
pub fn set_polygon_mode(mode: PolygonMode) {
    unsafe {
        gl::PolygonMode(gl::FRONT_AND_BACK, mode as _);
    }
}

/// Prints errors until there is no more. An optional context can be attached to the call to distinguish multiple calls
pub fn get_error(context: Option<&'static str>) {
    let mut error_code;
    loop {
        unsafe { error_code = gl::GetError() };
        if error_code == gl::NO_ERROR {
            return;
        }
        if context.is_some() {
            println!(
                "GL error: {}: {}",
                context.expect("value exists"),
                error_code
            )
        } else {
            println!("GL error: no context: {}", error_code);
        }
    }
}

/// Generate xz-plane data for tessellation patches
/// Starting from the minimum corner `pos` and spanning `size` along each dimension with `resolution` patches between
pub fn gen_patches(vertices: &mut Vec<Vertex>, resolution: u32, size: f32, pos: Vec3) {
    let unit_size = size / resolution as f32;
    for i in 0..resolution {
        for j in 0..resolution {
            vertices.push([
                pos.x + i as f32 * unit_size,
                pos.y,
                pos.z + j as f32 * unit_size,
                i as f32 / resolution as f32,
                j as f32 / resolution as f32,
                0.0,
                1.0,
                0.0,
            ]);
            vertices.push([
                pos.x + (i + 1) as f32 * unit_size,
                pos.y,
                pos.z + j as f32 * unit_size,
                (i + 1) as f32 / resolution as f32,
                j as f32 / resolution as f32,
                0.0,
                1.0,
                0.0,
            ]);
            vertices.push([
                pos.x + i as f32 * unit_size,
                pos.y,
                pos.z + (j + 1) as f32 * unit_size,
                i as f32 / resolution as f32,
                (j + 1) as f32 / resolution as f32,
                0.0,
                1.0,
                0.0,
            ]);
            vertices.push([
                pos.x + (i + 1) as f32 * unit_size,
                pos.y,
                pos.z + (j + 1) as f32 * unit_size,
                (i + 1) as f32 / resolution as f32,
                (j + 1) as f32 / resolution as f32,
                0.0,
                1.0,
                0.0,
            ]);
        }
    }
}
