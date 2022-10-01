use super::hit::HitRecord;
use super::ray::Ray;
use super::vec::Color;

pub trait Scatter {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
}
