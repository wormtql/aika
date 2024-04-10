use std::marker::PhantomData;
use std::ops::Div;
use cgmath::{BaseFloat, ElementWise, InnerSpace, Matrix3, MetricSpace, Vector3};
use image::{Rgb, RgbImage};
use num_traits::{Num, Zero};
use aika_math::{Hittable, Ray};
use crate::camera::PerspectiveCamera;
use crate::component::Transform;
use crate::scene::{Scene};
use crate::mashed_scene::MashedScene;
use crate::material::{BSDF, Material};
use crate::path_tracing::{ShadingContext, TracingService};
use anyhow::Result;
use indicatif::ProgressBar;
use crate::f;
use crate::path_tracing::shading_context::RayObjectStatus;

pub struct SimplePathTracing<F> {
    _phantom: PhantomData<F>
}

fn float_to_u8<F>(f: F) -> u8 where F: BaseFloat {
    let f = f.min(F::from(255).unwrap()).max(F::zero());
    f.to_u8().unwrap()
}

fn vector3_to_rgb<F>(x: Vector3<F>) -> Rgb<u8> where F: BaseFloat {
    let m = F::from(255).unwrap();
    // let clamped_x = x.x.min(F::one()).max(F::zero());
    // let clamped_y = x.y.min(F::one()).max(F::zero());
    // let clamped_z = x.z.min(F::one()).max(F::zero());
    // Rgb([float_to_u8(clamped_x * m), float_to_u8(clamped_y * m), float_to_u8(clamped_z * m)])
    Rgb([float_to_u8(x.x * m), float_to_u8(x.y * m), float_to_u8(x.z * m)])
}

fn tone_mapping<F>(x: Vector3<F>) -> Vector3<F> where F: BaseFloat {
    let one = F::one();
    let o = Vector3::new(one, one, one);
    return x.div_element_wise(x + o);
}

fn dir_to_rgb<F>(x: Vector3<F>) -> Rgb<u8> where F: BaseFloat {
    let half = F::from(0.5).unwrap();
    vector3_to_rgb(x * half + Vector3::new(half, half, half))
}

impl<F> SimplePathTracing<F> where F: BaseFloat + 'static {
    // pub fn calculate_directional_light_contribution(
    //     tracing_service: &TracingService<F>,
    //     shading_context: &ShadingContext<F>,
    //     material: &Box<dyn BSDF<F>>,
    // ) -> Vector3<F> {
    //     let mut result = Vector3::zero();
    //
    //     let view_dir = -shading_context.ray_dir;
    //
    //     for directional_light in tracing_service.directional_lights.iter() {
    //         let light_dir = -directional_light.direction;
    //
    //         if light_dir.dot(shading_context.normal) < F::zero() {
    //             continue;
    //         }
    //
    //         let shadow_ray = Ray::new(shading_context.point + shading_context.normal * F::from(1e-3).unwrap(), light_dir);
    //         let shadow_ray_hit_result = tracing_service.hit_ray(&shadow_ray, F::zero(), F::infinity());
    //         let half = F::from(0.5).unwrap();
    //
    //         if shadow_ray_hit_result.is_none() {
    //             let light_dir_ts = shading_context.convert_vector_to_tangent_space(light_dir);
    //             let view_dir_ts = shading_context.convert_vector_to_tangent_space(view_dir);
    //             let brdf = material.evaluate(light_dir_ts, view_dir_ts);
    //             let cos_theta = light_dir_ts.z;
    //             // println!("{:?}", cos_theta);
    //
    //             let contribution = directional_light.color.mul_element_wise(brdf) * cos_theta;
    //             result += contribution;
    //             // return Vector3::new(F::one(), F::zero(), F::zero());
    //         } else {
    //             // return Vector3::new(F::zero(), F::one(), F::zero());
    //         }
    //     }
    //
    //     result
    // }

    // pub fn calculate_point_light_contribution(
    //     tracing_service: &TracingService<F>,
    //     shading_context: &ShadingContext<F>,
    //     material: &Box<dyn BSDF<F>>,
    // ) -> Vector3<F> {
    //     let mut result = Vector3::zero();
    //
    //     let view_dir = -shading_context.ray_dir;
    //
    //     for point_light in tracing_service.point_lights.iter() {
    //         let light_dir = (point_light.position - shading_context.point).normalize();
    //         let dis = point_light.position.distance(shading_context.point);
    //         let shadow_ray = Ray::new(shading_context.point, light_dir);
    //         let shadow_ray_hit_result = tracing_service.hit_ray(&shadow_ray, F::zero(), dis);
    //         if shadow_ray_hit_result.is_none() {
    //             let light_dir_ts = shading_context.convert_vector_to_tangent_space(light_dir);
    //             let view_dir_ts = shading_context.convert_vector_to_tangent_space(view_dir);
    //             let brdf = material.evaluate(light_dir_ts, view_dir_ts);
    //             let cos_theta = light_dir_ts.z;
    //
    //             let attenuate = F::one() / (dis * dis);
    //
    //             // let contribution = point_light.color * attenuate * cos_theta * brdf;
    //             let contribution = point_light.color;
    //             result += contribution;
    //         }
    //     }
    //
    //     result
    // }

    // return pdf, color
    pub fn shade_one_ray(tracing_service: &mut TracingService<F>, ray: &Ray<F>, depth: usize, pixel: (usize, usize)) -> Result<Vector3<F>> {
        // if depth == 0 {
        //     return Ok(Vector3::zero());
        // }

        // let env_light_color = Vector3::new(F::one(), F::one(), F::one());
        // let env_light_color = Vector3::new(F::zero(), F::zero(), F::zero());
        let env_light_color = Vector3::new(f!(0.05), f!(0.05), f!(0.05));
        let vector_one = Vector3::new(F::one(), F::one(), F::one());

        let mut current_ray = ray.clone();
        let mut radiance = Vector3::zero();
        let mut throughput = vector_one;
        let mut shading_context = ShadingContext::new();
        // add air ior
        shading_context.push_ior(vector_one);

        for ray_iter in 0..depth {
            // let hit_result = tracing_service.hit_ray(&current_ray, F::from(1e-6).unwrap(), F::infinity());
            let hit_result = tracing_service.hit_ray(&current_ray, F::zero(), F::infinity());
            if let Some(r) = hit_result {
                let hit_triangle = r.hit_object.as_ref().unwrap().clone();
                let hit_point = r.get_hit_point(&current_ray);
                let uvw = hit_triangle.triangle.get_bary_centric_coordinate(hit_point);
                // let interpolated_normal = hit_triangle.interpolate_normal(uvw).unwrap().normalize();
                let interpolated_normal = hit_triangle.triangle.get_normal();
                let go = hit_triangle.go.clone();
                shading_context.go_stack.push(go.clone());
                shading_context.hit_point_stack.push(hit_point);

                if go.has_component::<Material<F>>() {
                    shading_context.normal = interpolated_normal;
                    let tangent = (hit_triangle.triangle.a - hit_triangle.triangle.b).normalize();
                    let tangent = (tangent - interpolated_normal * interpolated_normal.dot(tangent)).normalize();
                    let bitangent = interpolated_normal.cross(tangent).normalize();
                    shading_context.tangent = tangent;
                    shading_context.bitangent = bitangent;
                    shading_context.ray_dir = current_ray.direction;
                    shading_context.point = hit_point;
                    shading_context.recalculate_tangent_space();

                    let back_face = current_ray.direction.dot(interpolated_normal) > F::zero();
                    // println!("depth: {}, back_face: {:?}", ray_iter, r.back_facing.unwrap());
                    // let back_face = back_face > F::zero();
                    // println!("{:?}", r.back_facing);
                    // println!("t: {:?}", r.t);
                    // println!("{:?}: {}", pixel, back_face);
                    // println!("{:?}: {}", pixel, r.back_facing.unwrap());
                    shading_context.back_face = back_face;

                    let material_component = go.get_component::<Material<F>>().unwrap();
                    let material = material_component.downcast::<Material<F>>();

                    let mut sampled_ray_dir_ws = current_ray.direction;
                    let mut sampled_ray_point = shading_context.point;
                    let mut is_transmit = true;

                    if material.material_impl.has_bsdf() {
                        let bsdf = material.material_impl.get_bsdf(&shading_context).unwrap();
                        let wo = -shading_context.ray_dir_tangent_space;

                        // account for emi
                        {
                            let emit = bsdf.emit(wo);
                            if let Some(e) = emit {
                                radiance += throughput.mul_element_wise(e);
                            }
                        }

                        let sample_result = bsdf.sample_ray(tracing_service, wo);
                        if sample_result.is_none() {
                            // terminates here
                            break;
                        }
                        let sample_result = sample_result.unwrap();
                        // if wo.z <= F::zero() {
                        //     println!("{:?}, iter: {}, pixel: {:?}, backface: {}", wo, ray_iter, pixel, back_face);
                        // }

                        sampled_ray_dir_ws = shading_context.convert_vector_tangent_to_world(sample_result.direction).normalize();
                        throughput = throughput.mul_element_wise(sample_result.get_weight());
                        let next_point_bias = shading_context.convert_vector_tangent_to_world(sample_result.next_point);
                        sampled_ray_point += next_point_bias;

                        is_transmit = sampled_ray_dir_ws.dot(shading_context.normal)
                            * current_ray.direction.dot(shading_context.normal) > F::zero();

                        // shading_context.ray_status = RayObjectStatus::Unknown;
                        // if back_face && is_transmit {
                        //     shading_context.ray_status = RayObjectStatus::Exiting;
                        // } else if !back_face && is_transmit {
                        //     shading_context.ray_status = RayObjectStatus::Entering;
                        // }
                        // println!("is transmit: {}, {}, back face: {}", is_transmit, ray_iter, back_face);
                        if is_transmit && !back_face {
                            if let Some(ior) = material.material_impl.get_ior() {
                                // println!("entering ior: {:?}, {}", ior, ray_iter);
                                // println!("ray: {:?}", sampled_ray_dir_ws.dot(shading_context.normal));
                                shading_context.push_ior(ior);
                            }
                        } else if is_transmit && back_face {
                            if material.material_impl.get_ior().is_some() {
                                shading_context.pop_ior();
                            }
                            // println!("exit ior: {}", ray_iter);
                        }
                    }

                    shading_context.ray_dir = sampled_ray_dir_ws;

                    // println!("{}", is_transmit);
                    // let is_transmit = true;
                    // if is_transmit {
                    //     println!("transmit");
                    // }

                    if is_transmit {
                        if material.material_impl.has_volume() {
                            let volume = material.material_impl.get_volume().unwrap();
                            let sample_result = volume.sample_ray(
                                &tracing_service, &shading_context, sampled_ray_dir_ws
                            )?;
                            // println!("{:?}", sample_result.weight);
                            throughput.mul_assign_element_wise(sample_result.weight);
                            sampled_ray_dir_ws = sample_result.next_direction;
                            sampled_ray_point = sample_result.point;
                        }
                    }

                    let next_ray = Ray::new(sampled_ray_point, sampled_ray_dir_ws);
                    current_ray = next_ray.clone();
                    // let indir_color = SimplePathTracing::shade_one_ray(
                    //     &tracing_service, &next_ray, depth - 1, pixel
                    // )?;
                    // let color = weight.mul_element_wise(indir_color);
                    // return Ok(color);
                } else {
                    // magenta error color
                    let error_color = Vector3::new(F::one(), F::zero(), F::one());
                    radiance += error_color.mul_element_wise(throughput);
                    break;
                }
            } else {
                // println!("not hit");
                radiance += env_light_color.mul_element_wise(throughput);
                break;
            } // end if hit
        } // end for

        Ok(radiance)
    }

    pub fn trace(scene: &Scene<F>, width: usize, height: usize, camera: &PerspectiveCamera<F>, camera_transform: &Transform<F>) -> RgbImage {
        let mut result = RgbImage::new(width as u32, height as u32);
        let mut tracing_service = TracingService::new(scene);

        let pb = ProgressBar::new((width * height) as u64);

        for (ray, (i, j)) in camera.iter_ray(&camera_transform, width, height) {
            let mut sum = Vector3::zero();
            let spp = 64;
            for k in 0..spp {
                let color = SimplePathTracing::shade_one_ray(&mut tracing_service, &ray, 5, (i, j)).unwrap();
                // println!("{:?}", color.div_element_wise(pdf));
                sum += color;
            }
            // println!("{:?}", sum);
            let color = sum / F::from(spp).unwrap();
            // let tone_mapped_color = tone_mapping(color);
            // println!("{:?}", tone_mapped_color);
            let rgb = vector3_to_rgb(color);
            // println!("{:?}", rgb);
            result.put_pixel(i as u32, height as u32 - 1 - j as u32, rgb);

            pb.inc(1);

            // if i == 147 && j == 62 {
            //     break
            // }
        }

        pb.finish();

        result
    }
}
