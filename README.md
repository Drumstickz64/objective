# Objective

A basic importer for the Wavefront .obj file format. Written in the Rust programming language.

> [!WARNING]
> This library is just an experiment and not meant to be used in a real application

## Basic Example

```rust
let model = Model::load_from_file("assets/box2.obj").unwrap();

let mesh = &model.meshes[0];
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
```
