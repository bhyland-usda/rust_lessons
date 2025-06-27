pub mod shapes;
pub mod utils;

#[cfg(test)]
mod tests {
    use crate::{shapes::{Circle, Cube, Prism, Shape, Sphere}, utils::{FromFloat, Ops}};
    use std::f64::consts::PI;

    #[test]
    fn test_building_circle_diameter() {
        let cir = Circle::new_with_diameter(42.0);

        assert_eq!(cir.diameter(), 42.0);
        assert_eq!(cir.radius(), Ops::half(42.0));

        let area = cir.area();
        let vol = cir.volume();

        assert_eq!(vol, 0.0);
        assert_eq!(area, f32::from_f64(std::f64::consts::PI) * Ops::square(cir.radius()))
    }   

    #[test]
    fn test_building_circle_radius() {
        let cir = Circle::new_with_radius(10);
        
        assert_eq!(cir.diameter(), 20);
        assert_eq!(cir.radius(), 10);

        let area = cir.area();
        let vol = cir.volume();

        assert_eq!(vol, 0);
        assert_eq!(area, i32::from_f64(PI) * Ops::square(cir.radius()))
    }

    #[test]
    fn test_sphere_with_rad() {
        let s = Sphere::new_with_radius(20_u64);

        assert_eq!(s.diameter(), 40_u64);
        assert_eq!(s.radius(), 20_u64);

        let area = s.area();
        let vol = s.volume();
        let circumfrence = s.circumference();

        assert_eq!(area, 4 * PI as u64 * Ops::square(s.radius()));
        assert_eq!(vol, (4 / 3) as u64 * PI as u64 * Ops::cube(s.radius()));
        assert_eq!(circumfrence, 2 * PI as u64 * s.radius())
    }

    #[test]
    fn test_sphere_with_diameter() {
        let s = Sphere::new_with_diameter(20_u64);

        assert_eq!(s.diameter(), 20_u64);
        assert_eq!(s.radius(), 10_u64);

        let area = s.area();
        let vol = s.volume();
        let circumfrence = s.circumference();

        assert_eq!(area, 4 * PI as u64 * Ops::square(s.radius()));
        assert_eq!(vol, (4 / 3) as u64 * PI as u64 * Ops::cube(s.radius()));
        assert_eq!(circumfrence, 2 * PI as u64 * s.radius())
    }

    #[test]
    fn test_prism() {
        let prism = Prism::new(5, 10, 15);

        assert!(prism.length() == 5 && prism.width() == 10 && prism.height() == 15, 
            "length: {}, width: {}, height: {}", prism.length(), prism.width(), prism.height()
        );

        assert_eq!(prism.area(), 550);
        assert_eq!(prism.volume(), 750);
    }

    #[test]
    fn test_cube() {
        let cube = Cube::new(3);

        assert!(cube.sides() == 3);
        assert_eq!(cube.area(), 54);
        assert_eq!(cube.volume(), 27);
    }
}
