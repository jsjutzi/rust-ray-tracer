use super::hit::HitRecord;
use super::ray::Ray;
use super::vec::{Vec3, Color};

pub trait Scatter {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
}

pub struct Lambertian {
    albedo: Color
}

impl Lambertian {
    pub fn new(a: Color) -> Lambertian {
        Lambertian { 
            albedo: a 
        }
    }
}

impl Scatter for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let scatter_direction = rec.normal + Vec3::random_in_unit_sphere();
        let scattered = Ray::new(rec.p, scatter_direction);
        
        Some((self.albedo, scattered))
    }
}