extern crate cgmath;

use cgmath::Vector2;

/// Type for working with texture coordinates. Represents a section of a texture.
pub struct PixelsUv {
    uv_per: Vector2<f32>,
    pos_in_tex: Vector2<i32>,
    size_in_tex: Vector2<i32>,
}

impl PixelsUv {
    /// Creates a new `PixelsUv` type representing a full texture at a given size.
    pub fn full(size: [i32; 2]) -> Self {
        PixelsUv {
            uv_per: Vector2::new(1.0 / size[0] as f32, 1.0 / size[1] as f32),
            pos_in_tex: Vector2::new(0, 0),
            size_in_tex: size.into(),
        }
    }

    /// Creates a new `PixelsUv` section from the existing `TextureUv` at the given relative
    /// position and with the given size.
    pub fn subtexture(&self, position: [i32; 2], size: [i32; 2]) -> Self {
        PixelsUv {
            uv_per: self.uv_per,
            pos_in_tex: self.pos_in_tex + Vector2::from(position),
            size_in_tex: size.into(),
        }
    }

    /// Creates a new `OpenGlUv` type with the UV coordinates matching this texture section.
    pub fn to_opengl(&self) -> OpenGlUv {
        let start = [
            self.pos_in_tex.x as f32 * self.uv_per.x,
            1.0 - ((self.pos_in_tex.y + self.size_in_tex.y) as f32 * self.uv_per.y)
        ];

        let end = [
            (self.pos_in_tex.x + self.size_in_tex.x) as f32 * self.uv_per.x,
            1.0 - (self.pos_in_tex.y as f32 * self.uv_per.y)
        ];

        OpenGlUv::new(start, end)
    }
}

/// Represents texture coordinates in an OpenGL UV format.
pub struct OpenGlUv {
    start: [f32; 2],
    end: [f32; 2],
}

impl OpenGlUv {
    /// Creates a new `OpenGlUv` type with the given coordinates.
    pub fn new(start: [f32; 2], end: [f32; 2]) -> Self {
        OpenGlUv {
            start: start,
            end: end,
        }
    }

    /// Gets the OpenGL UV coordinates for the start of this texture section.
    pub fn start(&self) -> [f32; 2] {
        self.start
    }

    /// Gets the OpenGL UV coordinates for the end of this texture section.
    pub fn end(&self) -> [f32; 2] {
        self.end
    }

    /// Slightly adjusts start and end to correct for floating point errors.
    pub fn correct_fp_error(&self) -> Self {
        let c = 0.0001;

        let start = [self.start[0] + c, self.start[1] + c];
        let end = [self.end[0] - c, self.end[1] - c];

        Self::new(start, end)
    }
}

#[cfg(test)]
mod tests {
    use cgmath::Vector2;
    use {PixelsUv, OpenGlUv};

    #[test]
    fn start_and_end_return_unit_values_with_full_texture() {
        let tex = PixelsUv::full([10, 10]);

        let result = tex.to_opengl();
        assert_eq!(Vector2::from(result.start()), Vector2::new(0.0, 0.0));
        assert_eq!(Vector2::from(result.end()), Vector2::new(1.0, 1.0));
    }

    #[test]
    fn subtexture_with_full_texture_returns_new_subtexture_with_correct_uvs() {
        let tex = PixelsUv::full([64, 64]);
        let subtex = tex.subtexture([0, 0], [32, 32]);

        let result = subtex.to_opengl();
        assert_eq!(Vector2::from(result.start()), Vector2::new(0.0, 0.5));
        assert_eq!(Vector2::from(result.end()), Vector2::new(0.5, 1.0));
    }

    #[test]
    fn subtexture_with_subtexture_returns_new_subtexture_with_correct_uvs() {
        let tex = PixelsUv::full([200, 200]);
        let subtex = tex.subtexture([50, 50], [100, 100]).subtexture([50, 50], [50, 50]);

        let result = subtex.to_opengl();
        assert_eq!(Vector2::from(result.start()), Vector2::new(0.5, 0.25));
        assert_eq!(Vector2::from(result.end()), Vector2::new(0.75, 0.5));
    }

    #[test]
    fn correct_fp_error_returns_slightly_adjusted_uvs() {
        let original = OpenGlUv::new([0.2, 0.2], [0.8, 0.8]);

        let result = original.correct_fp_error();

        assert!(result.start()[0] > original.start()[0]);
        assert!(result.start()[1] > original.start()[1]);
        assert!(result.end()[0] < original.end()[0]);
        assert!(result.end()[1] < original.end()[1]);
    }
}
