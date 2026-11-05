#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FlashTarget {
    pub device_path: String,
    pub size_bytes: u64,
}

#[derive(Debug)]
pub struct UsbStickFlasher {
    pub is_writing: bool,
}

impl UsbStickFlasher {
    pub fn new() -> Self { Self { is_writing: false } }
    pub fn start_flash(&mut self, _iso: &str, _target: &FlashTarget) {
        self.is_writing = true;
    }
    pub fn complete_flash(&mut self) {
        self.is_writing = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_usb_stick_flasher_aaa() {
        // Arrange
        let mut flasher = UsbStickFlasher::new();
        let target = FlashTarget { device_path: "/dev/sdb".into(), size_bytes: 32_000_000_000 };
        // Act
        flasher.start_flash("sigmaos.iso", &target);
        assert!(flasher.is_writing);
        flasher.complete_flash();
        // Assert
        assert!(!flasher.is_writing);
    }
}
