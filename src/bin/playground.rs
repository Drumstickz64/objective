use objective::{Mesh, Model};

fn main() {
    let model = Model::load_from_file("assets/box.obj").unwrap();

    print_mesh(&model.meshes[0]);
}

fn print_mesh(mesh: &Mesh) {
    for (i, vertex) in mesh.vertices.chunks(3).enumerate() {
        println!("vertex {} => {vertex:?}", i + 1);
    }

    let mut prim_i = 0;
    let mut prim_num = 1;
    while prim_i < mesh.primitives.len() {
        let length = mesh.primitives[prim_i];
        println!(
            "primitive {} => {:?}",
            prim_num,
            &mesh.primitives[prim_i + 1..prim_i + 1 + length],
        );

        prim_i += length + 1;
        prim_num += 1;
    }
}
