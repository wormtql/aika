use cgmath::{BaseFloat, MetricSpace, Vector3};
use num_traits::Zero;
use aika_math::Ray;
use crate::material::{BSDF, MaterialTrait, VolumeSampleResult, VolumeTrait};
use crate::path_tracing::{ShadingContext, TracingService};
use anyhow::{anyhow, Result};

/// A homogeneous absorption volume, which follows beer-lambert law
#[derive(Clone)]
pub struct AbsorptionVolume<F> {
    pub absorption: Vector3<F>,
    // pub emission: F,
}

impl<F> AbsorptionVolume<F> where F: BaseFloat {
    pub fn new(absorption: Vector3<F>) -> Self {
        Self {
            absorption
        }
    }
}

impl<F> VolumeTrait<F> for AbsorptionVolume<F> where F: BaseFloat + 'static {
    fn transmittance(&self, p1: Vector3<F>, p2: Vector3<F>) -> Vector3<F> {
        let distance = p1.distance(p2);
        let e = F::from(std::f64::consts::E).unwrap();
        let ax = e.powf(-distance * self.absorption.x);
        let ay = e.powf(-distance * self.absorption.y);
        let az = e.powf(-distance * self.absorption.z);
        Vector3::new(ax, ay, az)
    }

    fn emit(&self, _p1: Vector3<F>, _p2: Vector3<F>) -> Vector3<F> {
        Vector3::zero()
    }

    fn sample_ray(
        &self,
        tracing_service: &TracingService<F>,
        shading_context: &ShadingContext<F>,
        current_dir: Vector3<F>
    ) -> Result<VolumeSampleResult<F>> {
        let point = shading_context.point;
        let normal = shading_context.normal;
        let ray = Ray::new(point - normal * F::from(1e-6).unwrap(), current_dir);
        // let ray = Ray::new(point, current_dir);
        // println!("Before hit");
        // let hit_result = tracing_service.hit_ray(&ray, F::from(1e-9).unwrap(), F::infinity());
        let hit_result = tracing_service.hit_ray(&ray, F::zero(), F::infinity());
        // println!("After hit");
        // if hit_result.is_none() {
        //     println!("{:?}", ray);
        // }
        // println!("{:?}", hit_result.is_some());

        if hit_result.is_none() {
            return Ok(VolumeSampleResult {
                next_direction: current_dir,
                point: shading_context.point,
                weight: Vector3::new(F::one(), F::one(), F::one())
            });
        }

        let hit_record = hit_result.unwrap();
        // println!("hit: {}", hit_record.back_facing.unwrap());
        let hit_point = hit_record.get_hit_point(&ray);

        let transmittance = self.transmittance(shading_context.point, hit_point);

        let result = VolumeSampleResult {
            next_direction: current_dir,
            point: hit_point,
            weight: transmittance,
        };

        Ok(result)
    }
}

pub struct AbsorptionVolumeMaterial<F> {
    pub absorption: Vector3<F>,
}

impl<F> AbsorptionVolumeMaterial<F> where F: BaseFloat {
    pub fn new(a: Vector3<F>) -> Self {
        Self {
            absorption: a
        }
    }
}

impl<F> MaterialTrait<F> for AbsorptionVolumeMaterial<F> where F: BaseFloat + 'static {
    fn has_volume(&self) -> bool {
        true
    }

    fn has_bsdf(&self) -> bool {
        false
    }

    fn get_bsdf(&self, context: &ShadingContext<F>) -> Option<Box<dyn BSDF<F>>> {
        None
    }

    fn get_volume(&self) -> Option<Box<dyn VolumeTrait<F>>> {
        Some(Box::new(AbsorptionVolume::new(self.absorption)))
    }
}

