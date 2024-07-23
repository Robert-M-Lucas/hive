use derive_getters::Getters;

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct HexCoord {
    pub x: isize,
    pub y: isize
}

impl HexCoord {
    pub fn new(x: isize, y: isize) -> HexCoord {
        HexCoord {
            x,
            y,
        }
    }

    pub fn to_square(&self) -> (isize, isize) {
        ((self.x * 2) + self.y, self.y)
    }

    pub fn from_square(x: isize, y: isize) -> HexCoord {
        HexCoord {
            x: (x - y) / 2,
            y
        }
    }

    pub fn try_from_square(x: isize, y: isize) -> Option<HexCoord> {
        if (x - y) % 2 != 0 {
            return None
        }

        Some(HexCoord {
            x: (x - y) / 2,
            y
        })
    }

    pub fn surrounding(&self) -> [HexCoord; 6] {
        let x = self.x;
        let y = self.y;
        [
            HexCoord::new(x, y + 1),
            HexCoord::new(x - 1, y + 1),
            HexCoord::new(x + 1, y),
            HexCoord::new(x + 1, y - 1),
            HexCoord::new(x, y - 1),
            HexCoord::new(x - 1, y),
        ]
    }
}