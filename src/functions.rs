use ultraviolet::Vec4;

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

/// Prints errors until there is no more
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
