use gl::types::GLuint;
use noise::{
    core::open_simplex::open_simplex_2d, permutationtable::PermutationTable, utils::PlaneMapBuilder,
};
use ultraviolet::Vec4;

use crate::functions::get_error;

/// Holds a texture object
pub struct Texture(pub GLuint);
impl Texture {
    /// Tries to generate a new texture object
    pub fn new() -> Option<Self> {
        let mut tex = 0;
        unsafe { gl::GenTextures(1, &mut tex) }
        if tex != 0 { Some(Self(tex)) } else { None }
    }

    /// Binds this texture to the given target
    pub fn bind(&self, ty: TextureType) {
        unsafe { gl::BindTexture(ty as _, self.0) }
    }

    /// Deletes this texture
    pub fn delete(self) {
        unsafe { gl::DeleteTextures(1, self.0 as _) }
    }

    /// Generate mipmaps
    pub fn gen_mipmap(ty: TextureType) {
        unsafe { gl::GenerateMipmap(ty as _) }
    }

    /// Sets texture wrap behaviour for the given target
    pub fn set_wrap_behaviour(ty: TextureType, dir: TexDirectionWrap, behaviour: TexWrapBehaviour) {
        unsafe { gl::TexParameteri(ty as _, dir as _, behaviour as _) }
    }

    /// Shortcut for setting wrap behaviour in both axes
    pub fn set_dual_wrap_behaviour(ty: TextureType, behaviour: TexWrapBehaviour) {
        Texture::set_wrap_behaviour(ty, TexDirectionWrap::X, behaviour);
        Texture::set_wrap_behaviour(ty, TexDirectionWrap::Y, behaviour);
    }

    /// Sets texture scale behaviour for the given target
    pub fn set_scale_behaviour(ty: TextureType, scale: TexScaleType, behaviour: TexScaleOp) {
        if scale == TexScaleType::Magnify
            && !(behaviour == TexScaleOp::Nearest || behaviour == TexScaleOp::Linear)
        {
            return println!(
                "{:?} is not a valid magnification scaling behaviour. operation aborted",
                behaviour
            );
        }
        unsafe { gl::TexParameteri(ty as _, scale as _, behaviour as _) }
    }

    /// Shortcut for setting scale behaviour in both up- and down-scales
    pub fn set_dual_scale_behaviour(ty: TextureType, behaviour: TexScaleOp) {
        Texture::set_scale_behaviour(ty, TexScaleType::Minify, behaviour);
        Texture::set_scale_behaviour(ty, TexScaleType::Magnify, behaviour);
    }

    /// Sets border colour for TexWrapBehaviour.ClampToBorder
    pub fn set_border_colour(ty: TextureType, col: Vec4) {
        unsafe { gl::TexParameterfv(ty as _, gl::TEXTURE_BORDER_COLOR, col.as_array().as_ptr()) }
    }

    /// Fills the active Tex2d with OpenSimplex noise
    pub fn fill_noise(size: usize, noise_gen: &PermutationTable) {
        let mut map =
            PlaneMapBuilder::<_, 2>::new_fn(|point| open_simplex_2d(point.into(), noise_gen))
                .set_size(size, size)
                .set_x_bounds(0.0, 10.0)
                .set_y_bounds(0.0, 10.0)
                .build();

        let mut minv = f64::MAX;
        let mut maxv = f64::MIN;

        let mut result = Vec::with_capacity(size * size);
        for item in map.iter_mut() {
            *item += 0.5;
            minv = minv.min(*item);
            maxv = maxv.max(*item);
            result.push(*item as f32);
        }

        let t_size = size as i32;

        get_error(Some("pre-teximage2d"));

        unsafe {
            gl::TexImage2D(
                TextureType::Tex2d as _,
                0,
                gl::R32F as _,
                t_size,
                t_size,
                0,
                gl::RED,
                gl::FLOAT,
                result.as_ptr().cast(),
            )
        };

        get_error(Some("post-teximage2d"));
    }
}

/// Sets the currently active texture unit.
pub fn set_texture_slot(slot: u32) {
    unsafe {
        gl::ActiveTexture(gl::TEXTURE0 + slot);
    }
}

/// Represents the types of textures
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextureType {
    // standard 2D texture
    Tex2d = gl::TEXTURE_2D as isize,
}

/// Represents the directions of texture wrapping
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TexDirectionWrap {
    /// x direction
    X = gl::TEXTURE_WRAP_S as isize,
    /// y direction
    Y = gl::TEXTURE_WRAP_T as isize,
}

/// Represents the various repeat behaviours at the edge of a texture
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TexWrapBehaviour {
    /// Repeats the texture by tiling it.
    Repeat = gl::REPEAT as isize,
    /// Repeats the texture by flipping at the edge
    MirroredRepeat = gl::MIRRORED_REPEAT as isize,
    /// Clamps tex coordinates to 0-1. 'Smears' colours along the texture edge
    ClampToEdge = gl::CLAMP_TO_EDGE as isize,
    /// Coordinates outside the texture are given another colour. See Texture.set_border_colour
    ClampToBorder = gl::CLAMP_TO_BORDER as isize,
}

/// Represents the kinds of texture scaling
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TexScaleType {
    /// Minifying operations
    Minify = gl::TEXTURE_MIN_FILTER as isize,
    /// Magnifying operations
    Magnify = gl::TEXTURE_MAG_FILTER as isize,
}

/// Represents the options for texture scaling
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TexScaleOp {
    /// Completely copies nearest texel
    Nearest = gl::NEAREST as isize,
    /// Bilinearly interpolates between the four neighbouring texels
    Linear = gl::LINEAR as isize,
    /// takes the nearest mipmap to match the pixel size and uses nearest neighbor interpolation for texture sampling.
    /// only available for minification operations.
    NearestMipmapNearest = gl::NEAREST_MIPMAP_NEAREST as isize,
    /// takes the nearest mipmap level and samples that level using linear interpolation.
    /// only available for minification operations.
    LinearMipmapNearest = gl::LINEAR_MIPMAP_NEAREST as isize,
    /// linearly interpolates between the two mipmaps that most closely match the size of a pixel and samples the interpolated level via nearest neighbor interpolation.
    /// only available for minification operations.
    NearestMipmapLinear = gl::NEAREST_MIPMAP_LINEAR as isize,
    /// linearly interpolates between the two closest mipmaps and samples the interpolated level via linear interpolation
    /// only available for minification operations.
    LinearMipmapLinear = gl::LINEAR_MIPMAP_LINEAR as isize,
}
