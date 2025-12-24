use wasm_bindgen::prelude::*;
use web_sys::console;

/// Log a message to the browser console
pub fn log(message: &str) {
    console::log_1(&JsValue::from_str(message));
}

/// Log an error to the browser console
pub fn log_error(message: &str) {
    console::error_1(&JsValue::from_str(message));
}

/// Log a warning to the browser console
pub fn log_warn(message: &str) {
    console::warn_1(&JsValue::from_str(message));
}

/// Set panic hook for better error messages
pub fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

/// Linear interpolation
#[inline]
pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

/// Clamp value between min and max
#[inline]
pub fn clamp(value: f32, min: f32, max: f32) -> f32 {
    value.max(min).min(max)
}

/// Map value from one range to another
#[inline]
pub fn map_range(value: f32, in_min: f32, in_max: f32, out_min: f32, out_max: f32) -> f32 {
    out_min + (value - in_min) * (out_max - out_min) / (in_max - in_min)
}

/// Check if two floats are approximately equal
#[inline]
pub fn approx_eq(a: f32, b: f32, epsilon: f32) -> bool {
    (a - b).abs() < epsilon
}

/// Safe division with default value
#[inline]
pub fn safe_div(a: f32, b: f32, default: f32) -> f32 {
    if b.abs() < 1e-10 {
        default
    } else {
        a / b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lerp() {
        assert_eq!(lerp(0.0, 10.0, 0.5), 5.0);
        assert_eq!(lerp(0.0, 10.0, 0.0), 0.0);
        assert_eq!(lerp(0.0, 10.0, 1.0), 10.0);
    }

    #[test]
    fn test_clamp() {
        assert_eq!(clamp(5.0, 0.0, 10.0), 5.0);
        assert_eq!(clamp(-5.0, 0.0, 10.0), 0.0);
        assert_eq!(clamp(15.0, 0.0, 10.0), 10.0);
    }

    #[test]
    fn test_approx_eq() {
        assert!(approx_eq(1.0, 1.00001, 0.001));
        assert!(!approx_eq(1.0, 1.1, 0.001));
    }
}
