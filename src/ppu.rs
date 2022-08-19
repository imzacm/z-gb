#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum ColorIndex {
    Zero,
    One,
    Two,
    Three,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct TileRow(u8, u8);

impl TileRow {
    #[inline(always)]
    fn pixel(&self, index: u8) -> ColorIndex {
        debug_assert!(index < 8);
        let a = (self.0 & (1 << index)) >> index;
        let b = ((self.1 & (1 << index)) >> index) << 1;
        let value = a | b;
        match value {
            0 => ColorIndex::Zero,
            1 => ColorIndex::One,
            2 => ColorIndex::Two,
            3 => ColorIndex::Three,
            _ => unreachable!(),
        }
    }

    #[inline(always)]
    fn set_pixel(&mut self, index: u8, color: ColorIndex) {
        debug_assert!(index < 8);
        let value: u8 = match color {
            ColorIndex::Zero => 0,
            ColorIndex::One => 1,
            ColorIndex::Two => 2,
            ColorIndex::Three => 3,
        };
        let a = (value & (1 << 0)) << index;
        let b = ((value & (1 << 1)) >> 1) << index;
        self.0 |= a;
        self.1 |= b;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tile_row_get_pixel() {
        let row = TileRow(0xA5, 0xC3);
        assert_eq!(row.pixel(0), ColorIndex::Three);
        assert_eq!(row.pixel(1), ColorIndex::Two);
        assert_eq!(row.pixel(2), ColorIndex::One);
        assert_eq!(row.pixel(3), ColorIndex::Zero);
        assert_eq!(row.pixel(4), ColorIndex::Zero);
        assert_eq!(row.pixel(5), ColorIndex::One);
        assert_eq!(row.pixel(6), ColorIndex::Two);
        assert_eq!(row.pixel(7), ColorIndex::Three);
    }

    #[test]
    fn tile_row_set_pixel() {
        let mut row = TileRow(0, 0);
        row.set_pixel(0, ColorIndex::Three);
        row.set_pixel(1, ColorIndex::Two);
        row.set_pixel(2, ColorIndex::One);
        row.set_pixel(3, ColorIndex::Zero);
        row.set_pixel(4, ColorIndex::Zero);
        row.set_pixel(5, ColorIndex::One);
        row.set_pixel(6, ColorIndex::Two);
        row.set_pixel(7, ColorIndex::Three);
        assert_eq!(row, TileRow(0xA5, 0xC3));
    }
}
