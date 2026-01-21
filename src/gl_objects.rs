/// Wrapper for a [VAO](https://www.khronos.org/opengl/wiki/Vertex_Specification#Vertex_Array_Object)
pub struct VertexArray(pub u32);
impl VertexArray {
    /// Creates a new VAO
    pub fn new() -> Option<Self> {
        let mut vao = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
        }
        if vao != 0 { Some(Self(vao)) } else { None }
    }

    /// Binds this VAO as current VAO
    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.0);
        }
    }

    /// Clear current VAO binding
    pub fn clear_bind() {
        unsafe {
            gl::BindVertexArray(0);
        }
    }
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
