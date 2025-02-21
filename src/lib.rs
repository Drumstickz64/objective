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
        let mut primitives = Vec::new();

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
                "v" => parse_vec3(&mut vertices, segments, line_number).unwrap(),
                "f" => {
                    let num_indices =
                        parse_primitive(&mut primitives, vertices.len() / 3, segments, line_number)
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
                primitives,
            }],
        })
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Mesh {
    pub vertices: Vec<f32>,
    /// a flat array of a length, followed by a series of indices of that length
    /// e.g. 3 1 2 3 1 4 2 5 6
    pub primitives: Vec<u16>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PrimitiveIter<'a> {
    primitives: &'a [u16],
}

impl<'a> Iterator for PrimitiveIter<'a> {
    type Item = &'a [u16];

    fn next(&mut self) -> Option<Self::Item> {
        if self.primitives.is_empty() {
            return None;
        }

        let length = self.primitives[0] as usize;
        let items = &self.primitives[1..1 + length];

        self.primitives = &self.primitives[1 + length..];

        Some(items)
    }
}

impl Mesh {
    pub fn iter_primitives(&self) -> PrimitiveIter {
        PrimitiveIter {
            primitives: &self.primitives,
        }
    }
}

fn parse_vec3<'a>(
    data: &mut Vec<f32>,
    mut segments: impl Iterator<Item = &'a str>,
    _line: usize,
) -> Result<(), ObjParseError> {
    data.push(segments.next().unwrap().parse::<f32>().unwrap());
    data.push(segments.next().unwrap().parse::<f32>().unwrap());
    data.push(segments.next().unwrap().parse::<f32>().unwrap());

    Ok(())
}

fn parse_primitive<'a>(
    primitives: &mut Vec<u16>,
    num_vertices: usize,
    segments: impl Iterator<Item = &'a str>,
    _line: usize,
) -> Result<u16, ObjParseError> {
    let mut length = 0;

    let length_index = primitives.len();
    primitives.push(0); // zero length for now, will be updated at end of primitive parsing

    for index in segments {
        let index: i64 = index.parse().unwrap();
        let index = match index {
            // + index instead of - index because it's negative
            ..0 => (num_vertices as i64 + index) as usize,
            0 => panic!("indices in a primitive must not be 0"),
            1.. => (index as usize) - 1,
        };
        primitives.push(index as u16);
        length += 1;
    }

    primitives[length_index] = length;

    Ok(length)
}
