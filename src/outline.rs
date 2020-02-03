use glam::Vec2;
use std::ops::Index;

/// Represent closed path of vertices
pub struct Outline {
    vertices: Vec<Vec2>,
}

impl Outline {
    /// Creates new outline.
    /// # Arguments
    /// * `vertices` - iterator of vertices. They **MUST** follow in order, which guarantee:
    /// 1) when follow from i to i+1 vertex, inner area of polygon **MUST** be at left side;
    pub fn new(vertices: impl Iterator<Item = Vec2>) -> Self {
        Outline {
            vertices: vertices.collect(),
        }
    }

    pub fn prev_that_next(&self, i: isize) -> (Vec2, Vec2, Vec2) {
        (self[i - 1], self[i], self[i + 1])
    }

    pub fn to_neighbors(&self, i: isize) -> (Vec2, Vec2) {
        let (prev, that, next) = self.prev_that_next(i);
        (prev - that, next - that)
    }

    pub fn convex(&self, i: isize) -> bool {
        let (to_prev, to_next) = self.to_neighbors(i);
        let from_prev = -to_prev;
        println!("From prev: {}", from_prev);
        println!("To next: {}", to_next);

        let from_prev_ort_to_inside = Vec2::new(-from_prev.y(), from_prev.x());
        println!("To inside: {}", from_prev_ort_to_inside);

        let dot = from_prev_ort_to_inside.dot(to_next);
        println!("Dot: {}", dot);
        dot >= 0f32
    }

    pub fn concave(&self, i: isize) -> bool {
        !self.convex(i)
    }

    pub fn inner_angle(&self, i: isize) -> f32 {
        let (to_prev, to_next) = self.to_neighbors(i);
        let min_angle =
            (to_prev.length_reciprocal() * to_next.length_reciprocal() * to_prev.dot(to_next))
                .acos();

        if self.convex(i) {
            return min_angle;
        } else {
            return 2f32 * std::f32::consts::PI - min_angle;
        }
    }

    pub fn outer_angle(&self, i: isize) -> f32 {
        return 2f32 * std::f32::consts::PI - self.inner_angle(i);
    }
}

impl Index<isize> for Outline {
    type Output = Vec2;

    fn index(&self, i: isize) -> &Vec2 {
        &self.vertices[i.rem_euclid(self.vertices.len() as isize) as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::Outline;
    use glam::Vec2;

    fn default_verts() -> Vec<Vec2> {
        let a = Vec2::new(0f32, 1f32);
        let b = Vec2::new(2f32, 3f32);
        let c = Vec2::new(4f32, 5f32);
        let d = Vec2::new(6f32, 7f32);
        vec![a, b, c, d]
    }

    #[test]
    fn indexing() {
        let verts = default_verts();
        let outline = Outline::new(verts.clone().into_iter());
        assert_eq!(outline[0], verts[0]);
        assert_eq!(outline[1], verts[1]);
        assert_eq!(outline[2], verts[2]);
        assert_eq!(outline[3], verts[3]);
        assert_eq!(outline[-1], verts[3]);
        assert_eq!(outline[-3], verts[1]);
        assert_eq!(outline[-19], verts[1]);
    }

    #[test]
    fn prev_that_next() {
        let verts = default_verts();
        let outline = Outline::new(verts.clone().into_iter());
        let (p, t, n) = outline.prev_that_next(0);
        assert_eq!(p, verts[3]);
        assert_eq!(t, verts[0]);
        assert_eq!(n, verts[1]);

        let (p, t, n) = outline.prev_that_next(-1);
        assert_eq!(p, verts[2]);
        assert_eq!(t, verts[3]);
        assert_eq!(n, verts[0]);
    }

    #[test]
    fn convex() {
        let a = Vec2::new(0f32, 0f32);
        let b = Vec2::new(1f32, 0f32);
        let c = Vec2::new(1f32, 1f32);
        let verts = vec![a, b, c];
        let outline = Outline::new(verts.into_iter());
        assert!(outline.convex(1));

        let b = Vec2::new(0f32, 1f32);
        let verts = vec![a, b, c];
        let outline = Outline::new(verts.into_iter());
        assert!(!outline.convex(1));
    }

    #[test]
    fn concave() {
        let a = Vec2::new(0f32, 0f32);
        let b = Vec2::new(1f32, 0f32);
        let c = Vec2::new(1f32, 1f32);
        let verts = vec![a, b, c];
        let outline = Outline::new(verts.into_iter());
        assert!(!outline.concave(1));

        let b = Vec2::new(0f32, 1f32);
        let verts = vec![a, b, c];
        let outline = Outline::new(verts.into_iter());
        assert!(outline.concave(1));
    }
}
