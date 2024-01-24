use bevy::prelude::*;
use rand::Rng;

pub fn clone_material_with_transparency(
    material: &StandardMaterial,
    alpha: f32,
) -> StandardMaterial {
    let mut new_material = material.clone();
    new_material.base_color = clone_color_with_transparency(new_material.base_color, alpha);
    new_material.emissive = clone_color_with_transparency(new_material.emissive.as_rgba(), alpha);
    new_material.alpha_mode = AlphaMode::Blend;
    new_material
}

pub fn clone_color_with_transparency(color: Color, alpha: f32) -> Color {
    match color {
        Color::Rgba {
            red,
            green,
            blue,
            alpha: _,
        } => Color::rgba(red, green, blue, alpha),
        Color::RgbaLinear {
            red,
            green,
            blue,
            alpha: _,
        } => Color::rgba_linear(red, green, blue, alpha),
        Color::Hsla {
            hue,
            saturation,
            lightness,
            alpha: _,
        } => Color::hsla(hue, saturation, lightness, alpha),
        Color::Lcha {
            lightness,
            chroma,
            hue,
            alpha: _,
        } => Color::lcha(lightness, chroma, hue, alpha),
    }
}

/// create a random color
pub fn create_random_color() -> Color {
    let mut rng = rand::thread_rng();
    Color::rgb(rng.gen(), rng.gen(), rng.gen())
}

/// create a material with a random color and default PBR properties
pub fn create_random_color_material() -> StandardMaterial {
    StandardMaterial {
        base_color: create_random_color(),
        ..default()
    }
}

/// create a material with a random color and PBR properties
pub fn create_random_pbr_material() -> StandardMaterial {
    let mut rng = rand::thread_rng();
    StandardMaterial {
        base_color: create_random_color(),
        perceptual_roughness: rng.gen_range(0.089..=1.0),
        metallic: rng.gen_range(0.0..=1.0),
        reflectance: rng.gen_range(0.0..=1.0),
        ..default()
    }
}
