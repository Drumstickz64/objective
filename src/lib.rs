use glam::Vec3;

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct ObjParseError;

pub fn parse_obj(content: &str) -> Result<Model, ObjParseError> {
    let mut vertices = Vec::new();

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
            "v" => vertices.push(parse_position(segments, line_number).unwrap()),
            stmt_type => eprintln!(
                "WARNING: unknown statement type '{}', skipping...",
                stmt_type
            ),
        }
    }

    // handle 1 mesh currently
    Ok(Model {
        meshes: vec![Mesh { vertices }],
    })
}

fn parse_position<'a>(
    segments: impl Iterator<Item = &'a str>,
    line: usize,
) -> Result<Vec3, ObjParseError> {
    parse_vec3(segments, line)
}

fn parse_vec3<'a>(
    mut segments: impl Iterator<Item = &'a str>,
    _line: usize,
) -> Result<Vec3, ObjParseError> {
    let x = segments.next().unwrap().parse::<f32>().unwrap();
    let y = segments.next().unwrap().parse::<f32>().unwrap();
    let z = segments.next().unwrap().parse::<f32>().unwrap();

    Ok(Vec3::new(x, y, z))
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Model {
    meshes: Vec<Mesh>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Mesh {
    pub vertices: Vec<Vec3>,
}
