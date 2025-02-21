use objective::{Mesh, Model};

fn main() {
    let model = Model::load_from_file("assets/box.obj").unwrap();

    print_mesh(&model.meshes[0]);
}

fn print_mesh(mesh: &Mesh) {
    for (i, vertex) in mesh.vertices.chunks_exact(3).enumerate() {
        println!("vertex {} => {vertex:?}", i + 1);
    }

    for (i, prim) in mesh.iter_primitives().enumerate() {
        println!("primitive {} => {:?}", i + 1, prim);
    }
}
