use std::ops;
pub fn cmp_float_nums_approx(a: f64, b: f64) -> bool {
    const EPSILON: f64 = 0.00001;
    if (a - b).abs() < EPSILON {
        return true;
    }
    return false;
}

#[derive(Debug)]
struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}
impl Tuple {
    pub fn point(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z, w: 1.0 }
    }
    pub fn vector(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z, w: 0.0 }
    }
    pub fn is_point(&self) -> bool {
        self.w == 1.0
    }
    pub fn is_vector(&self) -> bool {
        self.w == 0.0
    }
}
impl ops::Add<Self> for Tuple {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self{
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w
        } 
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_point() {
        let p = Tuple::point(4.3, -4.2, 3.1);
        assert_eq!(p.x, 4.3);
        assert_eq!(p.y, -4.2);
        assert_eq!(p.z, 3.1);
        assert_eq!(p.w, 1.0);
    }
    #[test]
    fn point_has_w_1() {
        let p = Tuple::point(4.3, -4.2, 3.1);
        assert_eq!(p.w, 1.0);
    }
    #[test]
    fn test_vector() {
        let v = Tuple::vector(4.3, -4.2, 3.1);
        assert_eq!(v.x, 4.3);
        assert_eq!(v.y, -4.2);
        assert_eq!(v.z, 3.1);
        assert_eq!(v.w, 0.0)
    }
    #[test]
    fn vector_has_w_0() {
        let v = Tuple::vector(4.3, -4.2, 3.1);
        assert_eq!(v.w, 0.0);
    }
    #[test]
    fn test_for_is_point_working() {
        let p = Tuple::point(4.3, -4.2, 3.1);
        assert!(p.is_point());
    }
    #[test]
    fn test_for_is_vector_working() {
        let v = Tuple::vector(4.3, -4.2, 3.1);
        assert!(v.is_vector());
    }
    #[test]
    fn testing_add_for_tuple() {
        let a = Tuple::point(3.0, -2.0, 5.0);
        let b = Tuple::vector(-2.0, 3.0, 1.0);
        let sum = Tuple{ x:1.0,y:1.0,z:6.0,w:1.0 };
        let c = a + b;
        assert_eq!(c.x,sum.x);
        assert_eq!(c.y,sum.y);
        assert_eq!(c.z,sum.z);
        assert_eq!(c.w,sum.w);

    }
}
