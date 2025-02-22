use objective::{Mesh, Model};

fn main() {
    let model = Model::load_from_file("assets/capsule/capsule.obj").unwrap();

    print_mesh(&model.meshes[0]);
}

fn print_mesh(mesh: &Mesh) {
    for (i, vertex) in mesh.vertices.chunks_exact(3).enumerate() {
        println!("vertex {} => {vertex:?}", i + 1);
    }

    for (i, normal) in mesh.normals.chunks_exact(3).enumerate() {
        println!("normal {} => {normal:?}", i + 1);
    }

    for (i, uv) in mesh.uvs.chunks_exact(2).enumerate() {
        println!("uv {} => {uv:?}", i + 1);
    }

    for (i, prim) in mesh.iter_elements().enumerate() {
        println!("element {} => {:?}", i + 1, prim);
    }
}
