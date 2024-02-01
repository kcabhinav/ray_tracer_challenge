use std::ops;
use std::num;
pub fn cmp_float_nums_approx(a: f64, b: f64) -> bool {
    const EPSILON: f64 = 0.00001;
    if (a - b).abs() < EPSILON {
        return true;
    }
    return false;
}

#[derive(Debug,PartialEq)]
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
trait Operations {
   fn mag(&self) -> f64;
   fn normalize(&self) -> Self;
   fn dot(self, other: &Self) -> f64;
   fn cross(self, other: &Self) -> Self;
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
impl ops::Sub<Self> for Tuple {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self{
        x: self.x - rhs.x,
        y: self.y - rhs.y,
        z: self.z - rhs.z,
        w: (self.w - rhs.w).abs(),
        }
    }
}
impl ops::Neg for Tuple {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            x: -1.0 * self.x ,
            y: -1.0 * self.y,
            z: -1.0 * self.z,
            w: -1.0 * self.w,
        }
    }
}
impl ops::Mul<f64> for Tuple {
    type Output = Self;
    fn mul(self, other: f64) -> Self::Output {
        Self {
            x: other * self.x,
            y: other * self.y,
            z: other * self.z,
            w: other * self.w,
        }
    }
}
impl Operations for Tuple {
    fn mag(&self) -> f64 {
        (self.x * self.x + self.y * self.y  + self.z * self.z + self.w * self.w).sqrt()
    }
    fn normalize(&self) -> Self {
        Self {
            x: self.x / self.mag(),
            y: self.y / self.mag(),
            z: self.z / self.mag(),
            w: self.w / self.mag(),
        }
    }
    fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.z
    }
    
}
impl Operations for Tuple::vector {
    fn cross(self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.y,
            z: self.x * other.y - self.y * other.z,
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
        assert_eq!(a+b,sum);
        }
    #[test]
    fn testing_sub_for_tuple() {
        let a = Tuple::point(3.0,2.0,1.0);
        let b = Tuple::vector(5.0, 6.0, 7.0);
        let dif = Tuple::point(-2.0, -4.0, -6.0);
        assert_eq!(a-b,dif);
    }
    #[test]
    fn testing_neg_for_tuple() {
        let a = Tuple { x:1.0, y:-2.0, z: 3.0, w:-4.0 };
        let minus_a = Tuple { x:-1.0, y:2.0, z: -3.0, w:4.0 };
        assert_eq!(minus_a, -a)
    }
    #[test]
    fn testing_scalar_mul_for_tuple() {
        let a = Tuple { x:1.0, y:-2.0, z: 3.0, w:-4.0 };
        let scalar_mul_a = Tuple { x: 3.5, y: -7.0, z: 10.5, w: -14.0};
        assert_eq!(scalar_mul_a, a * 3.5);
    }   
    #[test]
    fn testing_magnitutde_for_tuple() {
        let a = Tuple::vector(0.0, 1.0, 0.0);
        let mag_a = 1.0;
        assert_eq!(mag_a, a.mag())
    }
    #[test]
    fn testing_normalization_for_tuple() {
        let a = Tuple::vector(4.0, 0.0, 0.0);
        let norm_a = Tuple::vector(1.0, 0.0, 0.0);
        let b = Tuple::vector(1.0, 2.0, 3.0);
        let norm_b = Tuple::vector(0.26726, 0.53452, 0.80178);
        assert_eq!(norm_a, a.normalize());
        assert!(cmp_float_nums_approx(norm_b.x, b.normalize().x));
        assert!(cmp_float_nums_approx(norm_b.y, b.normalize().y));
        assert!(cmp_float_nums_approx(norm_b.z, b.normalize().z));
        assert!(cmp_float_nums_approx(norm_b.w, b.normalize().w));
    }
    #[test]
    fn testing_dot_product_for_tuple() {
        let a = Tuple::vector(1.0, 2.0, 3.0);
        let b = Tuple::vector(2.0, 3.0, 4.0);
        let dot_a_b = 20.0;
        assert_eq!(dot_a_b, a.dot(&b));
        
    }
}
