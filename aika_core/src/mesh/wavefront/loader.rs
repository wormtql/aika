use std::fmt::Debug;
use std::io::BufReader;
use std::path::Path;
use crate::mesh::{CommonVertex, Mesh, VertexBuffer};
use anyhow::Result;
use cgmath::{BaseFloat, Vector2, Vector3};
use num_traits::ToPrimitive;
use tobj::{LoadError, LoadOptions};

pub struct WavefrontMeshLoader;

fn get_vec3<T, F>(buf: &[T], index: usize) -> Vector3<F> where T: Copy + ToPrimitive, F: BaseFloat {
    Vector3::new(
        F::from(buf[3 * index]).unwrap(),
        F::from(buf[3 * index + 1]).unwrap(),
        F::from(buf[3 * index + 2]).unwrap()
    )
}

fn get_vec2<T, F>(buf: &[T], index: usize) -> Vector2<F> where T: Copy + ToPrimitive, F: BaseFloat {
    Vector2::new(
        F::from(buf[2 * index]).unwrap(),
        F::from(buf[2 * index + 1]).unwrap(),
    )
}

impl WavefrontMeshLoader {
    fn parse_model<F>(model: &tobj::Model) -> Mesh<Vec<CommonVertex<F>>> where F: BaseFloat {
        let mut vertices: Vec<CommonVertex<F>> = Vec::new();
        let mut triangles = Vec::new();

        let vertex_count = model.mesh.positions.len() / 3;
        for i in 0..vertex_count {
            let mut v: CommonVertex<F> = CommonVertex::new();
            v.position = get_vec3::<_, F>(model.mesh.positions.as_slice(), i);
            if !model.mesh.normals.is_empty() {
                v.normal = Some(get_vec3(&model.mesh.normals.as_slice(), i));
            }
            if !model.mesh.texcoords.is_empty() {
                v.uv0 = Some(get_vec2(&model.mesh.texcoords.as_slice(), i));
            }
            vertices.push(v);
        }

        let triangle_count = model.mesh.indices.len() / 3;
        for i in 0..triangle_count {
            let tri = [
                model.mesh.indices[i * 3] as usize,
                model.mesh.indices[i * 3 + 1] as usize,
                model.mesh.indices[i * 3 + 2] as usize
            ];
            triangles.push(tri.clone());
        }

        Mesh {
            vertices,
            triangles,
            sub_mesh: vec![[0, triangle_count]]
        }
    }

    pub fn load_wavefront_obj_memory<F>(data: &[u8]) -> Result<Vec<Mesh<Vec<CommonVertex<F>>>>>
    where
        F: BaseFloat
    {
        let load_options = LoadOptions {
            triangulate: true,
            single_index: true,
            ..Default::default()
        };
        let mut reader = BufReader::new(data);
        let (models, _materials) = tobj::load_obj_buf(
            &mut reader,
            &load_options,
            // todo. currently we don't care mtl materials
            |p| Err(LoadError::OpenFileFailed)
        )?;

        let mut result = Vec::new();
        for model in models.iter() {
            let m = WavefrontMeshLoader::parse_model::<F>(model);
            result.push(m);
        }

        Ok(result)
    }

    pub fn load_wavefront_obj<F, P>(p: P) -> Result<Vec<Mesh<Vec<CommonVertex<F>>>>>
        where
            P: AsRef<Path> + Debug,
            F: BaseFloat
    {
        let load_options = LoadOptions {
            triangulate: true,
            ..Default::default()
        };

        let (models, _materials) = tobj::load_obj(p, &load_options)?;
        let mut result = Vec::new();
        for model in models.iter() {
            let m = WavefrontMeshLoader::parse_model::<F>(model);
            result.push(m);
        }

        Ok(result)
    }

    pub fn suzanne<F>() -> Result<Mesh<Vec<CommonVertex<F>>>> where F: BaseFloat {
        let obj_file = include_bytes!("./suzanne.obj");
        let result = WavefrontMeshLoader::load_wavefront_obj_memory(obj_file.as_slice())?;

        Ok(result.into_iter().next().unwrap())
    }

    pub fn sphere<F>() -> Result<Mesh<Vec<CommonVertex<F>>>> where F: BaseFloat {
        let obj_file = include_bytes!("./sphere.obj");
        let result = WavefrontMeshLoader::load_wavefront_obj_memory(obj_file.as_slice())?;

        Ok(result.into_iter().next().unwrap())
    }

    pub fn sphere_smooth<F>() -> Result<Mesh<Vec<CommonVertex<F>>>> where F: BaseFloat {
        let obj_file = include_bytes!("./sphere_smooth.obj");
        let result = WavefrontMeshLoader::load_wavefront_obj_memory(obj_file.as_slice())?;

        Ok(result.into_iter().next().unwrap())
    }

    pub fn torus<F>() -> Result<Mesh<Vec<CommonVertex<F>>>> where F: BaseFloat {
        let obj_file = include_bytes!("./torus.obj");
        let result = WavefrontMeshLoader::load_wavefront_obj_memory(obj_file.as_slice())?;

        Ok(result.into_iter().next().unwrap())
    }

    pub fn beveled_cube<F>() -> Result<Mesh<Vec<CommonVertex<F>>>> where F: BaseFloat {
        let obj_file = include_bytes!("./cube.obj");
        let result = WavefrontMeshLoader::load_wavefront_obj_memory(obj_file.as_slice())?;

        Ok(result.into_iter().next().unwrap())
    }

    pub fn lucy<F>() -> Result<Mesh<Vec<CommonVertex<F>>>> where F: BaseFloat {
        let obj_file = include_bytes!("./lucy.obj");
        let result = WavefrontMeshLoader::load_wavefront_obj_memory(obj_file.as_slice())?;

        Ok(result.into_iter().next().unwrap())
    }
}
