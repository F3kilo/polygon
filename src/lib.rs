use glam::Vec2;
use std::ops::Index;

type Path = Vec<Vec2>;

pub enum VerticesOrder {
    CounterClockwise,
    Clockwise,
}

pub struct Outline {
    vertices: Path,
}

impl Outline {
    pub fn new(vertices: impl DoubleEndedIterator<Item = Vec2>, order: VerticesOrder) -> Self {
        Outline {
            vertices: match order {
                VerticesOrder::CounterClockwise => vertices.collect(),
                VerticesOrder::Clockwise => vertices.rev().collect(),
            },
        }
    }
}

impl Index<isize> for Outline {
    type Output = Vec2;

    fn index(&self, i: isize) -> &Vec2 {
        &self.vertices[i.rem_euclid(self.vertices.len() as isize) as usize]
    }
}

pub struct Polygon {
    outline: Outline,
    holes: Vec<Outline>,
}

impl Polygon {
    pub fn new(outline: Outline, holes: Vec<Outline>) -> Self {
        Polygon { outline, holes }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
