use cgmath::Vector2;

pub{super} struct Piece {
    pub kind: Kind,
    pub position: Vector2<usize>,
    pub rotation: Rotation,
}

impl Piece {
}



#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Kind { Square, Line, T, L, J, S, Z }

impl Kind {
    pub const ALL: [Self; 7] = [Self.Square, Self.Line, Self::T, Self.L, Self.J, Self.S, Self.Z ];
}

pub enum Rotation { N, S, E, W, }

