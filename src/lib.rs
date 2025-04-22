pub mod transform;
pub mod shape;
pub mod texture;
pub mod utils;
pub mod draws;
pub mod auxiliar;
pub mod colores;

pub mod prelude {
    pub use super::transform::*;
    pub use super::shape::*;
    pub use super::texture::*;
    pub use super::utils::*;
    pub use super::draws::*;
    pub use super::auxiliar::*;
    pub use super::colores::*;

    pub use rand::*;
    pub use glam::*;
    pub use image::*;
}