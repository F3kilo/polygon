use glam::Vec2;
use std::ops::Index;

/// Represent closed circuit of vertices
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

    /// Tuple of (`i-1`, `i`, `i+1`) vertices;
    /// * `i` - index of vertex. May be negative;
    pub fn prev_that_next(&self, i: isize) -> (Vec2, Vec2, Vec2) {
        (self[i - 1], self[i], self[i + 1])
    }

    /// Tuple of vectors to previous and to next vertex for `i`-th vertex;
    /// * `i` - index of vertex. May be negative;
    pub fn to_neighbors(&self, i: isize) -> (Vec2, Vec2) {
        let (prev, that, next) = self.prev_that_next(i);
        (prev - that, next - that)
    }

    /// Test if angle is convex;
    /// * `i` - index of vertex. May be negative;
    pub fn convex(&self, i: isize) -> bool {
        let (_, sin) = self.inner_angle_cos_sin(i);
        return sin > 0f32;
    }

    /// Test if angle is concave;
    /// * `i` - index of vertex. May be negative;
    pub fn concave(&self, i: isize) -> bool {
        !self.convex(i)
    }

    /// `sin()` and `cos()` for counter-clockwise angle between vector to next vertex and vector
    /// to previos.
    /// # Arguments
    /// * `i` - index of vertex. May be negative;
    pub fn inner_angle_cos_sin(&self, i: isize) -> (f32, f32) {
        let (to_prev, to_next) = self.to_neighbors(i);
        let prev_inv_len = to_prev.length_reciprocal();
        let next_inv_len = to_next.length_reciprocal();
        let norm_coef = prev_inv_len * next_inv_len;
        let cross = to_next.extend(0f32).cross(to_prev.extend(0f32));

        let cos = norm_coef * to_prev.dot(to_next);
        let sin = norm_coef * cross.z();
        (cos, sin)
    }

    /// Inner angle for vertex `i`-th vertex
    /// # Arguments
    /// * `i` - index of vertex. May be negative;
    pub fn inner_angle(&self, i: isize) -> f32 {
        let (cos, sin) = self.inner_angle_cos_sin(i);
        sin.atan2(cos)
    }

    /// Outer angle for vertex `i`-th vertex
    /// # Arguments
    /// * `i` - index of vertex. May be negative;
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

    #[test]
    fn inner_angle() {
        let a = Vec2::new(0f32, 0f32);
        let b = Vec2::new(1f32, 0f32);
        let c = Vec2::new(1f32, 1f32);
        let verts = vec![a, b, c];
        let outline = Outline::new(verts.into_iter());
        assert_eq!(outline.inner_angle(0), std::f32::consts::FRAC_PI_4);
        assert_eq!(outline.inner_angle(1), std::f32::consts::FRAC_PI_2);
    }

    #[test]
    fn outer_angle() {
        let a = Vec2::new(0f32, 0f32);
        let b = Vec2::new(1f32, 0f32);
        let c = Vec2::new(1f32, 1f32);
        let verts = vec![a, b, c];
        let outline = Outline::new(verts.into_iter());
        assert_eq!(outline.outer_angle(0), 7f32 * std::f32::consts::FRAC_PI_4);
        assert_eq!(outline.outer_angle(1), 3f32 * std::f32::consts::FRAC_PI_2);
    }
}
