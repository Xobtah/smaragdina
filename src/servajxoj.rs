pub fn cxifri_bajton(rgba: [u8; 4], bajto: u8) -> [u8; 4] {
    let [r, g, b, a] = rgba;
    [
        (r & 0xFC) + ((bajto & 0xC0) >> 6),
        (g & 0xFC) + ((bajto & 0x30) >> 4),
        (b & 0xFC) + ((bajto & 0x0C) >> 2),
        (a & 0xFC) + (bajto & 0x03),
    ]
}

pub fn decxifri_bajton(rgba: [u8; 4]) -> u8 {
    let [r, g, b, a] = rgba;
    ((r & 0x03) << 6) + ((g & 0x03) << 4) + ((b & 0x03) << 2) + (a & 0x03)
}

#[cfg(test)]
mod tests {
    use super::{cxifri_bajton as cxifri, decxifri_bajton as decxifri};

    #[test]
    fn encode_empty_pixel() {
        assert_eq!(cxifri([0, 0, 0, 0], 0b0000_0000), [0, 0, 0, 0]);
        assert_eq!(cxifri([0, 0, 0, 0], 0b0000_0001), [0, 0, 0, 1]);
        assert_eq!(cxifri([0, 0, 0, 0], 0b0000_0010), [0, 0, 0, 2]);
        assert_eq!(cxifri([0, 0, 0, 0], 0b0000_0100), [0, 0, 1, 0]);
        assert_eq!(cxifri([0, 0, 0, 0], 0b0000_1000), [0, 0, 2, 0]);
        assert_eq!(cxifri([0, 0, 0, 0], 0b0001_0000), [0, 1, 0, 0]);
        assert_eq!(cxifri([0, 0, 0, 0], 0b0010_0000), [0, 2, 0, 0]);
        assert_eq!(cxifri([0, 0, 0, 0], 0b0100_0000), [1, 0, 0, 0]);
        assert_eq!(cxifri([0, 0, 0, 0], 0b1000_0000), [2, 0, 0, 0]);
        assert_eq!(cxifri([0, 0, 0, 0], 0b1111_1111), [3, 3, 3, 3]);
    }

    #[test]
    fn decode() {
        assert_eq!(decxifri([0, 0, 0, 0]), 0);
        assert_eq!(decxifri([0, 0, 0, 1]), 1);
        assert_eq!(decxifri([0, 0, 0, 2]), 2);
        assert_eq!(decxifri([0, 0, 0, 3]), 3);
        assert_eq!(decxifri([0, 0, 0, 4]), 0);
        assert_eq!(decxifri([0, 0, 1, 0]), 4);
        assert_eq!(decxifri([0, 0, 2, 0]), 8);
        assert_eq!(decxifri([0, 0, 3, 0]), 12);
        assert_eq!(decxifri([0, 0, 4, 0]), 0);
        assert_eq!(decxifri([0, 1, 0, 0]), 16);
        assert_eq!(decxifri([0, 2, 0, 0]), 32);
        assert_eq!(decxifri([0, 3, 0, 0]), 48);
        assert_eq!(decxifri([0, 4, 0, 0]), 0);
        assert_eq!(decxifri([1, 0, 0, 0]), 64);
        assert_eq!(decxifri([2, 0, 0, 0]), 128);
        assert_eq!(decxifri([3, 0, 0, 0]), 192);
        assert_eq!(decxifri([4, 0, 0, 0]), 0);
    }
}
