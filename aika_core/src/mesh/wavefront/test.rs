use crate::mesh::WavefrontMeshLoader;

#[test]
fn test_wavefront_obj_loader1() {
    let suzanne = WavefrontMeshLoader::suzanne::<f32>();
    assert!(suzanne.is_ok());
    let result = suzanne.unwrap();
    assert_eq!(result.face_count(), 15488);
}
