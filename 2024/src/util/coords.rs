use std::ops;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Coords {
    pub x: i32,
    pub y: i32,
}

impl ops::Add<(i8, i8)> for Coords {
    type Output = Coords;

    fn add(self, other: (i8, i8)) -> Self::Output {
        Coords {
            x: self.x + i32::from(other.0),
            y: self.y + i32::from(other.1),
        }
    }
}

impl ops::Add<Coords> for Coords {
    type Output = Coords;

    fn add(self, other: Coords) -> Self::Output {
        Coords {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl ops::Sub<Coords> for Coords {
    type Output = Coords;
    fn sub(self, other: Coords) -> Self::Output {
        Coords {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl ops::Neg for Coords {
    type Output = Coords;
    fn neg(self) -> Self::Output {
        Coords {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl ops::Mul<i32> for Coords {
    type Output = Coords;
    fn mul(self, other: i32) -> Self::Output {
        Coords {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl Coords {
    pub fn new(x: i32, y: i32) -> Coords {
        Coords { x, y }
    }

    pub fn abs(self) -> Coords {
        Coords {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }

    pub fn bound(self: &Coords, size_x: i32, size_y: i32) -> Coords {
        let x = (self.x + (self.x.abs()) * size_x) % size_x;
        let y = (self.y + (self.y.abs()) * size_y) % size_y;
        assert!(x >= 0 && x < size_x && y >= 0 && y < size_y);
        return Coords { x, y };
    }

    pub fn dist(self: &Coords, other: &Coords) -> f64 {
        f64::sqrt(f64::from(
            (self.x - other.x).pow(2) + (self.y - other.y).pow(2),
        ))
    }
}
