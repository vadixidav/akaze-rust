use image::{DynamicImage, Pixel, RgbImage};
use random;
use random::Source;
use std::u32;
use std::path::PathBuf;

/// A point of interest in an image.
/// This pretty much follows from OpenCV conventions.
#[derive(Debug, Clone, Copy)]
pub struct Keypoint {
    /// The horizontal coordinate in a coordinate system is
    /// defined s.t. +x faces right and starts from the top
    /// of the image.
    /// the vertical coordinate in a coordinate system is defined
    /// s.t. +y faces toward the bottom of an image and starts
    /// from the left side of the image.
    pub point: (f32, f32),
    /// The magnitude of response from the detector.
    pub response: f32,

    /// The radius defining the extent of the keypoint, in pixel units
    pub size: f32,

    /// The level of scale space in which the keypoint was detected.
    pub octave: usize,

    /// A classification ID
    pub class_id: usize,
}

/// A feature descriptor.
#[derive(Debug, Clone)]
pub struct Descriptor {
    vector: Vec<bool>,
}


fn random_color() -> (u8, u8, u8) {
    let mut source = random::default().seed([42, 69]);
    (
        source.read::<u8>(),
        source.read::<u8>(),
        source.read::<u8>(),
    )
}

fn blend(p1: (u8, u8, u8), p2: (u8, u8, u8)) -> (u8, u8, u8) {
    (
        (((p1.0 as f32) + (p2.0 as f32)) / 2f32) as u8,
        (((p1.1 as f32) + (p2.1 as f32)) / 2f32) as u8,
        (((p1.2 as f32) + (p2.2 as f32)) / 2f32) as u8,
    )
}

/// Draw a circle to an image.
/// Values inside of the circle will be blended between their current color
/// value and the input.
///
/// `input_image` the image to draw on, directly mutated.
/// `point` the point at which to draw.
/// `rgb` The RGB value.
/// `radius` The maximum radius from the point to shade.
fn draw_circle(input_image: &mut RgbImage, point: (f32, f32), rgb: (u8, u8, u8), radius: f32) {
    for x in (point.0 as u32).saturating_sub(radius as u32)
        ..(point.0 as u32).saturating_add(radius as u32)
    {
        for y in (point.1 as u32).saturating_sub(radius as u32)
            ..(point.1 as u32).saturating_add(radius as u32)
        {
            let xy = (x as f32, y as f32);
            let delta_x = xy.0 - point.0;
            let delta_y = xy.1 - point.1;
            let radius_check = f32::sqrt(delta_x * delta_x + delta_y * delta_y);
            if radius_check <= radius {
                let pixel = input_image.get_pixel_mut(x, y);
                let rgb_point = (
                    pixel.channels()[0],
                    pixel.channels()[1],
                    pixel.channels()[2],
                );
                let color_to_set = blend(rgb, rgb_point);
                pixel.channels_mut()[0] = color_to_set.0;
                pixel.channels_mut()[1] = color_to_set.1;
                pixel.channels_mut()[2] = color_to_set.2;
            }
        }
    }
}

/// Draw keypoints onto an image
/// Keypoints of a random color will be drawn to the input image. The
/// points will be shaded between the existing pixel value and the
/// random color value.
/// `input_image` The image on which to draw.
/// `keypoints` a vector of keypoints to draw.
pub fn draw_keypoints_to_image(input_image: &mut RgbImage, keypoints: &Vec<Keypoint>) {
    for keypoint in keypoints.iter() {
        draw_circle(input_image, keypoint.point, random_color(), keypoint.size);
    }
}

/// Draw keypoints onto an image
/// Keypoints of a random color will be drawn to the input image. The
/// points will be shaded between the existing pixel value and the
/// random color value.
/// `input_image` The image on which to draw.
/// `keypoints` a vector of keypoints to draw.
/// # Return value
/// An new RGB image with keypoints drawn.
pub fn draw_keypoints(input_image: &DynamicImage, keypoints: &Vec<Keypoint>) -> RgbImage {
    let mut rgb_image = input_image.to_rgb();
    draw_keypoints_to_image(&mut rgb_image, keypoints);
    rgb_image
}

/// Serialize results to a file.
/// 'keypoints' - the keypoints detected from an image.
/// `descriptors` - The descriptors extracted from the keypoints. Will
///                 panic if the size of this vector is not equal to the
///                 size of the keypoints, or 0.
pub fn serialize_to_file(
    _keypoints: &Vec<Keypoint>, _descriptors: &Vec<Descriptor>,
    _path: PathBuf,
) {
    warn!("TODO: serialize to file.");
}
