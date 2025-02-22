// TODO: get face parsing correct and load capsule successfully

use std::{fs, io, path::Path};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Model {
    pub meshes: Vec<Mesh>,
}

#[derive(Debug)]
pub enum ObjLoadError {
    Io(io::Error),
    Parse(ObjParseError),
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct ObjParseError;

impl Model {
    pub fn load_from_file(path: impl AsRef<Path>) -> Result<Model, ObjLoadError> {
        let content = fs::read_to_string(path).unwrap();

        Ok(Self::parse(&content).unwrap())
    }

    pub fn parse(content: &str) -> Result<Model, ObjParseError> {
        let mut vertices = Vec::new();
        let mut normals = Vec::new();
        let mut uvs = Vec::new();
        let mut elements = Vec::new();

        for (line_number, line) in content
            .lines()
            .enumerate()
            .filter(|(_, line)| !line.is_empty() && !line.starts_with('#'))
        {
            let mut segments = line.split_ascii_whitespace();

            let stmt_type = segments
                .next()
                .expect("expected statement type at start of line, but got nothing");

            match stmt_type {
                "v" => {
                    let num_components =
                        Self::parse_vector(&mut vertices, segments, line_number).unwrap();
                    if num_components != 3 {
                        panic!("{line} only 3 dimensional vectors are supported for statement 'v', but it had '{num_components}' components instead");
                    }
                }
                "vn" => {
                    let num_components =
                        Self::parse_vector(&mut normals, segments, line_number).unwrap();
                    if num_components != 3 {
                        panic!("{line} only 3 dimensional vectors are supported for statement 'vn', but it had '{num_components}' components instead");
                    }
                }
                "vt" => {
                    let num_components =
                        Self::parse_vector(&mut uvs, segments, line_number).unwrap();
                    if num_components != 2 {
                        panic!("{line} only 2 dimensional vectors are supported for statement 'vt', but it had '{num_components}' components instead");
                    }
                }
                "f" => {
                    let num_indices = Self::parse_face(
                        &vertices,
                        &normals,
                        &uvs,
                        &mut elements,
                        segments,
                        line_number,
                    )
                    .unwrap();
                    if num_indices < 3 {
                        panic!("{line}: 'f' command must provide at least 3 indices, but instead provided '{num_indices}");
                    }
                }
                stmt_type => eprintln!(
                    "WARNING: unknown statement type '{}', skipping...",
                    stmt_type
                ),
            }
        }

        // handle 1 mesh currently
        Ok(Model {
            meshes: vec![Mesh {
                vertices,
                normals,
                uvs,
                elements,
            }],
        })
    }

    fn parse_vector<'a>(
        data: &mut Vec<f32>,
        segments: impl Iterator<Item = &'a str>,
        _line: usize,
    ) -> Result<usize, ObjParseError> {
        let mut count = 0;
        for segment in segments {
            let component: f32 = segment.parse().unwrap();
            data.push(component);
            count += 1;
        }

        Ok(count)
    }

    fn parse_face<'a>(
        vertices: &[f32],
        normals: &[f32],
        uvs: &[f32],
        elements: &mut Vec<u16>,
        segments: impl Iterator<Item = &'a str>,
        _line: usize,
    ) -> Result<u16, ObjParseError> {
        // TODO

        let mut length = 0;
        let mut provided_data = ElementDataType::VertexOnly;

        let start_index = elements.len();

        elements.push(ElementDataType::VertexOnly.into_element_data());
        elements.push(0); // zero length for now, will be updated at end of element parsing

        for indices in segments {
            let mut index_parts = indices.split('/');

            let vertex_index: i64 = index_parts.next().unwrap().parse().unwrap();
            let vertex_index = Self::map_index(vertex_index, vertices, 3).unwrap();
            elements.push(vertex_index);

            if let Some(uv_index) = index_parts.next() {
                if !uv_index.is_empty() {
                    provided_data = ElementDataType::All;
                    let uv_index: i64 = uv_index.parse().unwrap();
                    let uv_index = Self::map_index(uv_index, uvs, 2).unwrap();
                    elements.push(uv_index);
                }
            }

            if let Some(normal_index) = index_parts.next() {
                if provided_data != ElementDataType::All {
                    provided_data = ElementDataType::VertexAndNormal;
                }

                let normal_index: i64 = normal_index.parse().unwrap();
                let normal_index = Self::map_index(normal_index, normals, 3).unwrap();
                elements.push(normal_index);
            }

            length += 1;
        }

        elements[start_index] = provided_data.into_element_data();
        elements[start_index + 1] = length;

        Ok(length)
    }

    fn map_index(index: i64, vectors: &[f32], num_components: usize) -> Result<u16, ObjParseError> {
        // TODO: return error when index out of bounds
        let num_vectors = vectors.len() / num_components;
        Ok(match index {
            // + index instead of - index because it's negative
            ..0 => (num_vectors as i64 + index) as u16,
            0 => panic!("indices in a element must not be 0"),
            1.. => (index as u16) - 1,
        })
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Mesh {
    /// a flat array of floats representing 3d vectors describing vertex positions
    pub vertices: Vec<f32>,
    /// a flat array of floats representing 3d vectors describing vertex normals
    pub normals: Vec<f32>,
    /// a flat array of floats representing 2d vectors describing vertex UVs (texture coordinates)
    pub uvs: Vec<f32>,
    /// a flat array of a length, followed by a series of indices of that length that
    /// make up a element (e.g. lines, triangles, quads, polygons, etc...)
    ///
    /// For example: 3 1 2 3 1 4 2 5 6
    pub elements: Vec<u16>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ElementDataType {
    VertexOnly,
    VertexAndNormal,
    All,
}

impl ElementDataType {
    pub fn into_element_data(self) -> u16 {
        self as u16
    }

    pub fn from_element_data(num: u16) -> Self {
        match num {
            0 => Self::VertexOnly,
            1 => Self::VertexAndNormal,
            2 => Self::All,
            _ => unreachable!(),
        }
    }
}

impl Mesh {
    pub fn iter_elements(&self) -> ElementIter {
        ElementIter {
            elements: &self.elements,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ElementIter<'a> {
    elements: &'a [u16],
}

impl<'a> Iterator for ElementIter<'a> {
    type Item = (&'a [u16], ElementDataType);

    fn next(&mut self) -> Option<Self::Item> {
        if self.elements.is_empty() {
            return None;
        }

        let element_data_type = ElementDataType::from_element_data(self.elements[0]);
        let length = self.elements[1] as usize;

        let length = match element_data_type {
            ElementDataType::VertexOnly => length,
            ElementDataType::VertexAndNormal => length * 2,
            ElementDataType::All => length * 3,
        };

        let items = &self.elements[2..2 + length];
        self.elements = &self.elements[2 + length..];

        Some((items, element_data_type))
    }
}
