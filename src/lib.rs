pub mod waveform;
pub mod chart;

pub enum Frames {
    StartFrame = 0x00000000,
    EndFrame = 0xFFFFFFFF
}

pub trait Dotstar {
    /// This function should return a slice of bytes that contains:
    /// Start frame (4 bytes)
    /// LED data (4 bytes per LED)
    /// End frame (4 bytes)
    fn generate_frame(&self) -> &[u8];
}

pub fn format_led(r: u8, g: u8, b: u8, brightness: u8) -> u32 {
    let mut global_brightness = brightness & 0x1F; // global brightness is 5 bits
    global_brightness |= 0xE0; // global brightness is in the upper 3 bits
    u32::from_be_bytes([global_brightness, b, g, r])
}

#[cfg(test)]
mod tests {
    use crate::format_led;

    #[test]
    fn test_format_led() {
        let led = format_led(0x00, 0x00, 0x00, 0x00);
        assert_eq!(led, 0xE0000000);

        let led = format_led(0xFF, 0xFF, 0xFF, 0xFF);
        assert_eq!(led, 0xFFFFFFFF);

        let led = format_led(0xFF, 0x00, 0x00, 0x1F);
        assert_eq!(led, 0xFF0000FF);

        let led = format_led(0x00, 0xFF, 0x00, 0x1F);
        assert_eq!(led, 0xFF00FF00);

        let led = format_led(0x00, 0x00, 0xFF, 0x1F);
        assert_eq!(led, 0xFFFF0000);
    }
}
