use super::{Coordinate, Offset};

// {super}
pub struct Piece {
    pub kind: Kind,
    pub position: Offset,
    pub rotation: Rotation,
}

impl Piece {
    const CELL_COUNT: usize = 4;

    pub fn cells(&self) -> Option<[Coordinate; Self::CELL_COUNT]> {
        let offsets = self.kind.cells()
            .map(self.rotator())
            .map(self.positioner());


    }

    fn rotator(&self) -> impl Fn(Offset) -> Offset {
        let rotation  = self.rotation;
        move |cell| cell * rotation      
    }

    fn positioner(&self) ->  impl Fn(Offset) -> Offset {
        let position = self.position;
        move |cell| cell * position
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Kind {
    O,
    I,
    T,
    L,
    J,
    S,
    Z,
}

impl Kind {
    pub const ALL: [Self; 7] = [Self.O, Self.I, Self::T, Self.L, Self.J, Self.S, Self.Z];

    pub fn cells(&self) -> [Offset;Piece::CELL_COUNT] {
        match self {
            Kind::O => &[(0, 0), (0, 1), (1, 0), (1, 1)],
            Kind::I => &[(-1, 0), (0, 0), (1, 0), (2, 0)],
            Kind::T => &[(-1, 0), (0, 0), (1, 0), (0, 1)],
            Kind::L => &[(-1, 0), (0, 0), (1, 0), (1, 1)],
            Kind::J => &[(-1, 1), (-1, 0), (0, 0), (1, 0)],
            Kind::S => &[(-1, 0), (0, 0), (0, 1), (1, 1)],
            Kind::Z => &[(-1, 1), (0, 1), (0, 0), (1, 0)],
        }.map(Offset::from)
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Rotation {
    N,
    S,
    E,
    W,
}

impl std::ops::Mul<Rotation> for Offset {
    type Output = Self;

    fn mul(self, rotation: Rotation) -> Self::Output {
        match rotation {
            Rotation::N => self,
            Rotation::S => Offset::new(-self.x, -self.y),
            Rotation::E => Offset::new(self.y, -self.x),
            Rotation::W => Offset::new(-self.y, -self.x),
        }
    }
}
