use std::{fs, str::Chars};

use ultraviolet::{Vec2, Vec3};

pub type Vertex = [f32; 8];
pub type TriIndex = [usize; 3];

pub fn parse_wavefront(path: &str) -> Result<Option<Vec<Vertex>>, String> {
    let mut vertex_points = Vec::<Vec3>::new();
    let mut vertex_texcoords = Vec::<Vec2>::new();
    let mut vertex_normals = Vec::<Vec3>::new();
    let mut raw_vertices = Vec::<Vertex>::new();

    let obj_file = fs::read_to_string(path).map_err(|e| format!("Model read error: {}", e))?;

    for line in obj_file.lines() {
        if line.len() <= 2 {
            continue;
        }
        let mut chars = line.chars();
        match format!("{}{}", chars.next().unwrap(), chars.next().unwrap()).as_str() {
            "v " => {
                vertex_points.push(Vec3 {
                    x: read_f32(chars.by_ref())?,
                    y: read_f32(chars.by_ref())?,
                    z: read_f32(chars.by_ref())?,
                });
            }
            "vn" => {
                chars.next(); // skip the space after n since we're matching two chars
                vertex_normals.push(Vec3 {
                    x: read_f32(chars.by_ref())?,
                    y: read_f32(chars.by_ref())?,
                    z: read_f32(chars.by_ref())?,
                });
            }
            "vt" => {
                chars.next(); // skip the space after t
                vertex_texcoords.push(Vec2 {
                    x: read_f32(chars.by_ref())?,
                    y: read_f32(chars.by_ref())?,
                });
            }
            "f " => {
                let face_vertices = chars.as_str().split(" ");
                for vert in face_vertices {
                    let vert_param_indices = vert.split("/");
                    let mut vertex: Vertex = Vertex::default();

                    let position = *vertex_points
                        .get(
                            vert_param_indices
                                .clone()
                                .next()
                                .expect("face should provide at least position index")
                                .parse::<usize>()
                                .expect("position index should be a parseable usize")
                                - 1,
                        )
                        .expect("position should be a valid vector")
                        .as_array();

                    let tex_coords = *vertex_texcoords
                        .get(
                            vert_param_indices
                                .clone()
                                .nth(1)
                                .expect("face should provide at least position index")
                                .parse::<usize>()
                                .expect("position index should be a parseable usize")
                                - 1,
                        )
                        .map_or(Vec2::zero().as_array(), |tex_coords| tex_coords.as_array());

                    let normal = *vertex_normals
                        .get(
                            vert_param_indices.clone().nth(2).map_or(usize::MAX, |f| {
                                f.parse::<usize>()
                                    .or::<usize>(Ok(usize::MAX))
                                    .expect("Err should not be possible")
                            }) - 1,
                        )
                        .map_or(Vec3::zero().as_array(), |normal| normal.as_array());

                    vertex[0] = position[0];
                    vertex[1] = position[1];
                    vertex[2] = position[2];
                    vertex[3] = tex_coords[0];
                    vertex[4] = tex_coords[1];
                    vertex[5] = normal[0];
                    vertex[6] = normal[1];
                    vertex[7] = normal[2];

                    raw_vertices.push(vertex);
                }
            }
            _ => {}
        }
    }

    Ok(match raw_vertices.len() {
        0 => None,
        _ => Some(raw_vertices),
    })
}

/// Reads a float from a char iterator then consumes the subsequent space.
fn read_f32(chars: &mut Chars<'_>) -> Result<f32, String> {
    let float = chars
        .by_ref()
        .take_while(|c| !c.is_whitespace())
        .fold(String::new(), |mut value, char| {
            value.push(char);
            value
        })
        .parse()
        .map_err(|e| format!("Float parsing error: {}", e))?;
    Ok(float)
}
