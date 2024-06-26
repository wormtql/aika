use std::rc::Rc;
use cgmath::{BaseFloat, Deg, Euler, Quaternion, Rotation3, Vector2, Vector3, Zero};
use aika_core::camera::PerspectiveCamera;
use aika_core::mesh::{DynMesh, PlaneMesh, WavefrontMeshLoader};
use aika_core::path_tracing::{ShadeNormal, SimplePathTracing, TracingService};
use aika_core::scene::{GameObject, Scene};
use anyhow::Result;
use image::RgbImage;
use indicatif::ProgressBar;
use aika_core::component::{MeshFilter, Transform};
use aika_core::lighting::{DirectionalLightComponent, SphericalLight, SphericalLightComponent};
use aika_core::material::{AbsorptionVolumeMaterial, ConductorBRDF, DielectricMaterial, DiffuseBRDF, DiffuseBRDFMaterial, Material, MaterialConstants, MaterialType, MetallicRoughnessBRDFMaterial, RoughConductorBRDF, RoughConductorBRDFMaterial, RoughDielectricBSDFMaterial, UniformEmitMaterial};
use aika_core::material_graph::Texture2DNode;
use aika_core::renderer::TexcoordsRenderer;
use aika_core::texture::{CheckerboardTexture, Texture2DTrait};
use aika_core::utils::vector3_to_rgb_clamped;
use aika_math::UniformSampler;
use aika_math::utils::new_vector3;

macro_rules! f {
    ($v:expr) => {
        F::from($v).unwrap()
    }
}

fn get_plane<F>() -> GameObject<F> where F: BaseFloat + 'static {
    let mut game_object = GameObject::new_empty(String::from("Plane"));
    game_object.set_name("plane");

    // transform
    {
        let transform = Transform::new(
            // Vector3::new(f!(0.0), f!(1.0), f!(-4)),
            Vector3::new(f!(0.0), f!(-1.1), f!(-2)),
            f!(0.7),
            Euler::new(Deg(f!(-45)), Deg(f!(0.0)), Deg(f!(0.0))).into()
        );
        game_object.add_component_owned(transform);
    }

    // mesh
    {
        // let mesh: DynMesh<F> = PlaneMesh::create_plane_mesh(f!(10), f!(10)).to_dyn_mesh();
        let mesh = PlaneMesh::create_plane_mesh(f!(20), f!(20));
        // let mesh: DynMesh<F> = WavefrontMeshLoader::suzanne()?.to_dyn_mesh();
        let mesh_filter = MeshFilter::new(mesh);
        game_object.add_component_owned(mesh_filter);
    }

    // material
    {
        // let material = Material { material_impl: Box::new(RoughConductorBRDFMaterial::new(f!(0.2), MaterialConstants::gold_ior())) };
        let material: Material<F> = Material { material_impl: Box::new(DiffuseBRDFMaterial::new(Vector3::new(f!(0.1), f!(0.8), f!(0.6))) ) };
        // let material: Material<F> = Material { material_impl: Box::new(AbsorptionVolumeMaterial::new(Vector3::new(f!(1.0), f!(0.5), f!(0.2)))) };
        // let material = Material { material_impl: Box::new(UniformEmitMaterial::new(Vector3::new(f!(1), f!(0), f!(0)))) };
        game_object.add_component_owned(material);
    }

    game_object
}

fn get_sphere<F>() -> GameObject<F> where F: BaseFloat + 'static {
    let mut game_object = GameObject::new_empty(String::from("Plane"));
    game_object.set_name("back");

    // transform
    {
        let transform = Transform::new(
            // Vector3::new(f!(0.0), f!(1.0), f!(-4)),
            Vector3::new(f!(0.0), f!(2.0), f!(-2)),
            f!(0.7),
            Quaternion::zero()
        );
        game_object.add_component_owned(transform);
    }

    // mesh
    {
        // let mesh: DynMesh<F> = PlaneMesh::create_plane_mesh(f!(10), f!(10)).to_dyn_mesh();
        let mesh: DynMesh<F> = WavefrontMeshLoader::sphere().unwrap().to_dyn_mesh();
        // let mesh: DynMesh<F> = WavefrontMeshLoader::suzanne()?.to_dyn_mesh();
        let mesh_filter = MeshFilter::new(mesh);
        game_object.add_component_owned(mesh_filter);
    }

    // material
    {
        // let material = Material { material_impl: Box::new(RoughConductorBRDFMaterial::new(f!(0.2), MaterialConstants::gold_ior())) };
        // let material: Material<F> = Material { material_impl: Box::new(DiffuseBRDFMaterial::new(Vector3::new(f!(1.0), f!(0.5), f!(0.2))) ) };
        // let material: Material<F> = Material { material_impl: Box::new(AbsorptionVolumeMaterial::new(Vector3::new(f!(1.0), f!(0.5), f!(0.2)))) };
        let material = Material { material_impl: Box::new(UniformEmitMaterial::new(Vector3::new(f!(5), f!(5), f!(6)) * f!(0.2))) };
        game_object.add_component_owned(material);
    }

    game_object
}

fn get_directional_light<F>() -> GameObject<F> where F: BaseFloat + 'static {
    let light: DirectionalLightComponent<F> = DirectionalLightComponent::new(Vector3::new(f!(0.5), f!(0.6), f!(0.7)) * f!(3.0));
    let mut go = GameObject::new_empty(String::from("light"));
    go.add_component_owned(light);

    let transform: Transform<F> = Transform::new(
        Vector3::zero(),
        f!(1.0),
        Euler::new(Deg(f!(180.0)), Deg(f!(45.0)), Deg(f!(45.0))).into()
    );
    go.add_component_owned(transform);

    go
}

fn get_spherical_light<F>() -> GameObject<F> where F: BaseFloat + 'static {
    let light: SphericalLightComponent<F> = SphericalLightComponent::new(f!(0.7), new_vector3(2, 2, 2.4) * f!(1));
    let mut go = GameObject::new_empty(String::from("spherical light"));
    go.add_component_owned(light);

    let transform: Transform<F> = Transform::new(
        // Vector3::new(f!(0.0), f!(1.0), f!(-4)),
        Vector3::new(f!(0.0), f!(2.0), f!(-2)),
        f!(1),
        Quaternion::zero()
    );
    go.add_component_owned(transform);

    go
}

// fn get_reactangular_light<F>() -> GameObject<F> where F: BaseFloat + 'static {
//
// }

fn get_torus<F: BaseFloat + 'static>() -> GameObject<F> {
    let mut game_object = GameObject::new_empty(String::from("sphere"));

    // transform
    {
        let transform = Transform::new(
            Vector3::new(f!(0.0), f!(-0.0), f!(-2)),
            f!(0.5),
            Euler::new(Deg(f!(45)), Deg(f!(0.0)), Deg(f!(0.0))).into()
        );
        game_object.add_component_owned(transform);
    }

    // mesh
    {
        // let mesh: DynMesh<F> = WavefrontMeshLoader::torus().unwrap().to_dyn_mesh();
        // let mesh: DynMesh<F> = WavefrontMeshLoader::lucy().unwrap().to_dyn_mesh();
        let mesh: DynMesh<F> = WavefrontMeshLoader::sphere().unwrap().to_dyn_mesh();
        // let mesh: DynMesh<F> = WavefrontMeshLoader::suzanne().unwrap().to_dyn_mesh();
        let mesh_filter = MeshFilter::new(mesh);
        game_object.add_component_owned(mesh_filter);
    }

    // material
    {
        let checkerboard: Rc<dyn Texture2DTrait<F>> = Rc::new(
            CheckerboardTexture::new(
                f!(0.07),
                Vector3::new(f!(0.1), f!(0.1), f!(0.1)),
                Vector3::new(F::one(), F::one(), F::one())
            ));
        // let material: Material<F> = Material { material_impl: Box::new(DiffuseBRDFMaterial::new(Vector3::new(f!(1.0), f!(0.8), f!(0.2))) ) };
        let material: Material<F> = Material { material_impl: Box::new(AbsorptionVolumeMaterial::new(Vector3::new(f!(0.1), f!(0.5), f!(0.2)))) };
        // let material: Material<F> = Material { material_impl: Box::new(AbsorptionVolumeMaterial::new(Vector3::new(f!(0), f!(0), f!(0)))) };
        // let material: Material<F> = Material {
        //     material_impl: Box::new(ConductorBRDF::gold_in_air())
        // };
        // let material = Material { material_impl: Box::new(DielectricMaterial::new(Vector3::new(f!(2.0), f!(2.0), f!(2.0)))) };
        // let material = Material {
        //     material_impl: Box::new(RoughDielectricBSDFMaterial::new(
        //         Rc::new(Texture2DNode::new(checkerboard)),
        //         f!(2)
        //     ))
        // };
        // let material = Material { material_impl: Box::new(RoughDielectricBSDFMaterial::new(f!(0.01), Vector3::new(f!(1.5), f!(1.5), f!(1.5)))) };
        // let material = Material { material_impl: Box::new(RoughConductorBRDFMaterial::new(f!(0.1), MaterialConstants::gold_ior())) };
        // let material = Material {
        //     material_impl: Box::new(MetallicRoughnessBRDFMaterial::new(
        //         Rc::new(f!(0.3)),
        //         // Rc::new(Texture2DNode::new(checkerboard)),
        //         Rc::new(f!(1)),
        //         // Rc::new(new_vector3(1, 0.782, 0.344))
        //         Rc::new(Texture2DNode::new(checkerboard))
        //     )),
        // };
        game_object.add_component_owned(material);
    }

    game_object
}

fn main_with_type<F>() -> Result<()> where F: BaseFloat + 'static {
    let mut scene = Scene::<F>::new();

    // scene.add_game_object(get_sphere());
    scene.add_game_object(get_plane());
    scene.add_game_object(get_directional_light());
    scene.add_game_object(get_spherical_light());

    scene.add_game_object(get_torus());

    let camera = PerspectiveCamera::new(f!(60.0 / 180.0 * 3.1415926), f!(0.01), f!(1000.0), f!(1.0));
    let camera_transform: Transform<F> = Transform::new(
        Vector3::new(f!(0.0), f!(0.0), f!(1.0)),
        f!(0.0),
        Euler::new(Deg(f!(0.0)), Deg(f!(0.0)), Deg(f!(0.0))).into()
    );

    // let uv_renderer = TexcoordsRenderer::new(0);
    // let image = uv_renderer.render(&scene, 300, 300, &camera, &camera_transform);

    let size = 300;
    let mut image = RgbImage::new(size as u32, size as u32);

    let pixel_size_x = F::one() / F::from(size).unwrap();
    let pixel_size_y = F::one() / F::from(size).unwrap();
    let sampler: UniformSampler<F> = UniformSampler::new();
    let h = F::from(0.5).unwrap();
    let mut tracing_service = TracingService::new(&scene);
    let spp = 1;
    let pb = ProgressBar::new((size * size) as u64);
    for x in 0..size {
        for y in 0..size {
            let mut total_value = Vector3::zero();

            for _sample_iter in 0..spp {
                let sample = sampler.sample_2d();
                let offset_x = sample.x - h;
                let offset_y = sample.y - h;

                let pixel_center_x = (h + F::from(x).unwrap() + offset_x) * pixel_size_x;
                let pixel_center_y = (h + F::from(y).unwrap() + offset_y) * pixel_size_y;
                let pixel_center = Vector2::new(pixel_center_x, pixel_center_y);
                let ray = camera.get_ray_world_space(pixel_center, &camera_transform);

                let pixel = SimplePathTracing::shade_one_ray(&mut tracing_service, &ray, 2, (x, y))?;

                total_value += pixel;
            }

            pb.inc(1);

            let pixel = total_value / F::from(spp).unwrap();
            image.put_pixel(x as u32, size as u32 - 1 - y as u32, vector3_to_rgb_clamped(pixel));
        }
    }

    pb.finish();
    image.save("trace.png")?;

    Ok(())
}

fn main() -> Result<()> {
    main_with_type::<f32>()
}
