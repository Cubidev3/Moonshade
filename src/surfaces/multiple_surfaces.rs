use std::cmp::Ordering;
use crate::math::ray::Ray;
use crate::surfaces::surface::{Surface, SurfacePoint};

pub struct MultipleSurfaces {
    surfaces: Vec<Box<dyn Surface + Sync + Send>>
}

impl MultipleSurfaces {
    pub fn new(surfaces: Vec<Box<dyn Surface + Sync + Send>>) -> MultipleSurfaces {
        MultipleSurfaces { surfaces }
    }
}

impl Surface for MultipleSurfaces {
    fn intersect(&self, ray: Ray) -> Option<SurfacePoint> {
        self.surfaces.iter()
            .filter_map(|surface| surface.intersect(ray))
            .min_by(|s1, s2| s1.t.partial_cmp(&s2.t).unwrap_or(Ordering::Equal))
    }
}