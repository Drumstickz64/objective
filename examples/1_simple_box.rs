use macroquad::prelude::*;
use objective::{ElementDataType, Model};

#[macroquad::main("Simple Box")]
async fn main() {
    let model = Model::load_from_file("assets/box.obj").unwrap();

    let mesh = &model.meshes[0];

    let mut macroquad_indices = Vec::new();
    for (i, prim) in mesh.iter_elements().enumerate() {
        let (prim, ElementDataType::VertexOnly) = prim else {
            panic!("expected vertex only data for 'box.obj'");
        };

        assert_eq!(
            prim.len(),
            4,
            "expected 'box.obj' to only have quad elements, but element '{i}' had {}",
            prim.len()
        );
        // triangle 1
        macroquad_indices.push(prim[0]);
        macroquad_indices.push(prim[1]);
        macroquad_indices.push(prim[3]);
        // triangle 2
        macroquad_indices.push(prim[1]);
        macroquad_indices.push(prim[2]);
        macroquad_indices.push(prim[3]);
    }

    let macroquad_mesh = Mesh {
        vertices: mesh
            .vertices
            .chunks(3)
            .map(|pos| Vertex::new2(Vec3::from_slice(pos), Vec2::ZERO, ORANGE))
            .collect(),
        indices: macroquad_indices,
        texture: None,
    };

    set_camera(&Camera3D {
        position: vec3(1.0, 3.0, 5.0),
        target: Vec3::ZERO,
        up: Vec3::Y,
        ..Default::default()
    });

    'GameLoop: loop {
        if is_key_pressed(KeyCode::Escape) {
            break 'GameLoop;
        }

        clear_background(BLACK);

        draw_mesh(&macroquad_mesh);

        next_frame().await
    }
}
