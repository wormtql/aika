use std::fmt::Debug;
use std::path::Path;
use crate::mesh::{CommonVertex, Mesh};
use anyhow::Result;
use cgmath::{BaseFloat, Vector3};
use num_traits::ToPrimitive;

pub struct WavefrontMeshLoader;

fn get_vec3<T, F>(buf: &[T], index: usize) -> Vector3<F> where T: Copy + ToPrimitive, F: BaseFloat {
    Vector3::new(
        F::from(buf[3 * index]).unwrap(),
        F::from(buf[3 * index + 1]).unwrap(),
        F::from(buf[3 * index + 2]).unwrap()
    )
}

impl WavefrontMeshLoader {
    fn parse_model<F>(model: &tobj::Model) -> Mesh<Vec<CommonVertex<F>>> where F: BaseFloat {
        let mut vertices: Vec<CommonVertex<F>> = Vec::new();
        let mut triangles = Vec::new();

        let triangle_count = model.mesh.indices.len() / 3;
        for i in 0..triangle_count {
            let tri = [
                model.mesh.indices[i * 3] as usize,
                model.mesh.indices[i * 3 + 1] as usize,
                model.mesh.indices[i * 3 + 2] as usize
            ];
            triangles.push(tri.clone());

            for &j in tri.iter() {
                let mut v: CommonVertex<F> = CommonVertex::new();
                v.position = get_vec3::<_, F>(model.mesh.positions.as_slice(), j);
                if !model.mesh.normals.is_empty() {
                    v.normal = Some(get_vec3(&model.mesh.positions.as_slice(), j));
                }
                // todo
                // let uv0 = model.mesh.tex
            }
        }

        Mesh {
            vertices,
            triangles,
            sub_mesh: vec![[0, triangle_count]]
        }
    }

    pub fn load_wavefront_obj<F, P>(p: P) -> Result<Vec<Mesh<Vec<CommonVertex<F>>>>>
        where
            P: AsRef<Path> + Debug,
            F: BaseFloat
    {
        let (models, _materials) = tobj::load_obj(p, &Default::default())?;
        let mut result = Vec::new();
        for model in models.iter() {
            let m = WavefrontMeshLoader::parse_model::<F>(model);
            result.push(m);
        }

        Ok(result)
    }
}
