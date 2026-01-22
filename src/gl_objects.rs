use crate::wavefront_parser::Vertex;

/// Wrapper for a [VAO](https://www.khronos.org/opengl/wiki/Vertex_Specification#Vertex_Array_Object)
pub struct VertexArray(pub u32, pub Option<Buffer>, pub Option<Vec<Vertex>>);
impl VertexArray {
    /// Creates a new VAO
    pub fn new() -> Option<Self> {
        let mut vao = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
        }
        if vao != 0 {
            Some(Self(vao, None, None))
        } else {
            None
        }
    }

    /// Binds this VAO as current VAO
    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.0);
        }
    }

    /// Attaches vertex data with a standard format (pos-tex-normal)
    pub fn attach_vertex(&mut self, vertices: Vec<Vertex>) {
        self.bind();
        self.1 = Buffer::new();
        self.1.expect("VBO should create").bind(BufferType::Array);

        self.2 = Some(vertices);
        
        buffer_data(
            BufferType::Array,
            bytemuck::cast_slice(self.2.as_ref().expect("vertex vector should exist")),
            gl::STATIC_DRAW,
        );

        unsafe {
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                size_of::<Vertex>().try_into().unwrap(),
                0 as *const _,
            );
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                1,
                2,
                gl::FLOAT,
                gl::FALSE,
                size_of::<Vertex>().try_into().unwrap(),
                (3 * size_of::<f32>()) as *const _,
            );
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(
                2,
                3,
                gl::FLOAT,
                gl::FALSE,
                size_of::<Vertex>().try_into().unwrap(),
                (5 * size_of::<f32>()) as *const _,
            );
            gl::EnableVertexAttribArray(2);
        }
    }

    /// Draws from the attached buffer. Binds the VAO and draws the complete buffer once. Does not attach a shader.
    pub fn draw(&self, prim: Primitive) {
        if self.1.is_none() {
            eprintln!("VertexArray::draw called on VAO without VBO")
        } else {
            self.bind();
            unsafe {
                gl::DrawArrays(prim as _, 0, self.2.as_ref().unwrap().len() as i32);
            }
        }
    }

    /// Clear current VAO binding
    pub fn clear_bind() {
        unsafe {
            gl::BindVertexArray(0);
        }
    }
}

/// Types of vertex primitives
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Primitive {
    Points = gl::POINTS as isize,
    Lines = gl::LINES as isize,
    LineStrip = gl::LINE_STRIP as isize,
    Triangles = gl::TRIANGLES as isize,
    TriangleStrip = gl::TRIANGLE_STRIP as isize,
    TriangleFan = gl::TRIANGLE_FAN as isize,
}

/// Possible types of buffer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BufferType {
    /// Vertex data array
    Array = gl::ARRAY_BUFFER as isize,
    /// Array for pointers to vertices to form shapes
    ElementArray = gl::ELEMENT_ARRAY_BUFFER as isize,
}

/// Wrapper for a (generic buffer)[https://www.khronos.org/opengl/wiki/Buffer_Object]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Buffer(pub u32);
impl Buffer {
    /// Makes a new buffer
    pub fn new() -> Option<Self> {
        let mut vbo = 0;
        unsafe {
            gl::GenBuffers(1, &mut vbo);
        }
        if vbo != 0 { Some(Self(vbo)) } else { None }
    }

    /// Bind this buffer to given type
    pub fn bind(&self, ty: BufferType) {
        unsafe { gl::BindBuffer(ty as _, self.0) }
    }

    /// Clear current buffer binding for given type.
    pub fn clear_binding(ty: BufferType) {
        unsafe { gl::BindBuffer(ty as _, 0) }
    }
}

/// places data into the bound buffer of given type
pub fn buffer_data(ty: BufferType, data: &[u8], usage: gl::types::GLenum) {
    unsafe {
        gl::BufferData(
            ty as _,
            data.len().try_into().unwrap(),
            data.as_ptr().cast(),
            usage,
        );
    }
}
