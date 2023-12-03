pub mod station;
#[cfg(test)]
pub mod test;
pub mod train;

use emath::{Pos2, Rect, RectTransform};

pub fn offset_x() -> f32 {
    100.0
}
pub fn offset_y() -> f32 {
    50.0
}

/// 1分を何ピクセルで描画するか
pub const PIXEL_PER_TIME: f32 = 20.0;
/// 1kmを何ピクセルで描画するか
pub const PIXEL_PER_KM: f32 = 10.0;

pub fn to_screen(rect: Rect) -> RectTransform {
    let rect = Rect {
        min: Pos2 {
            x: rect.min.x + offset_x(),
            y: rect.min.y + offset_y(),
        },
        max: Pos2 {
            x: rect.max.x,
            y: rect.max.y,
        },
    };
    let viewer_size = rect.max - rect.min;
    RectTransform::from_to(
        Rect::from_x_y_ranges(0.0..=0.0 + viewer_size.x, 0.0..=0.0 + viewer_size.y),
        rect,
    )
}
