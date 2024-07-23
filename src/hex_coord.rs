use derive_getters::Getters;

#[derive(Hash, Eq, PartialEq, Clone, Getters)]
pub struct HexCoord {
    x: isize,
    y: isize
}

impl HexCoord {
    pub fn new(x: isize, y: isize) -> HexCoord {
        HexCoord {
            x,
            y,
        }
    }

    pub fn surrounding(&self) -> [HexCoord; 6] {
        let x = self.x;
        let y = self.y;
        [
            HexCoord::new(x + 1, y),
            HexCoord::new(x - 1, y),
            HexCoord::new(x, y + 1),
            HexCoord::new(x - 1, y + 1),
            HexCoord::new(x, y - 1),
            HexCoord::new(x + 1, y - 1),
        ]
    }
}