#[derive(Debug, Clone, Default, PartialEq)]
pub struct Model {
    pub meshes: Vec<Mesh>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Mesh {
    pub vertices: Vec<f32>,
    /// a flat array of a length, followed by a series of indices of that length
    /// e.g. 3 1 2 3 1 4 2 5 6
    pub primitives: Vec<usize>,
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct ObjParseError;

pub fn parse_obj(content: &str) -> Result<Model, ObjParseError> {
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
    primitives: &mut Vec<usize>,
    num_vertices: usize,
    segments: impl Iterator<Item = &'a str>,
    _line: usize,
) -> Result<usize, ObjParseError> {
    let mut length = 0;

    let length_index = primitives.len();
    primitives.push(0); // zero length for now, will be updated at end of primitive parsing

    for index in segments {
        let index: i64 = index.parse().unwrap();
        let index = match index {
            // + index instead of - index because it's negative
            // + 1 because indices are 1 based
            ..0 => (num_vertices as i64 + index + 1) as usize,
            0 => panic!("indices in a primitive must not be 0"),
            1.. => index as usize,
        };
        primitives.push(index);
        length += 1;
    }

    primitives[length_index] = length;

    Ok(length)
}
