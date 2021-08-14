use crate::scene::object::parsers::common::{integer::parse_integer, whitespace::tws};
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, opt, recognize, value},
    multi::many1,
    sequence::{preceded, tuple},
    IResult,
};

mod vertex;
use self::vertex::Vertex;

mod vector;
use self::vector::Vector;

mod texture_coordinates;
use self::texture_coordinates::TextureCoordinates;

#[derive(Debug, PartialEq)]
pub struct PolygonVertexIndices {
    vertex: usize,
    texture_coordinates: Option<usize>,
    normal: Option<usize>,
}

fn parse_polygon_vertex_indices(input: &str) -> IResult<&str, PolygonVertexIndices> {
    let (input, (vertex, texture_coordinates, normal)) = alt((
        tuple((
            parse_integer,
            value(None, tag("/")),
            preceded(tag("/"), map(parse_integer, |i| Some(i))),
        )),
        tuple((
            parse_integer,
            opt(preceded(tag("/"), parse_integer)),
            opt(preceded(tag("/"), parse_integer)),
        )),
    ))(input)?;
    // TODO: if 0 -> ERROR

    Ok((
        input,
        PolygonVertexIndices {
            vertex,
            texture_coordinates,
            normal,
        },
    ))
}

#[test]
fn test_parse_polygon_vertex_indices() {
    assert_eq!(
        parse_polygon_vertex_indices("1/1/5\r\n"),
        Ok((
            "\r\n",
            PolygonVertexIndices {
                vertex: 1,
                texture_coordinates: Some(1),
                normal: Some(5),
            }
        ))
    );
    assert_eq!(
        parse_polygon_vertex_indices("10//5\r\n"),
        Ok((
            "\r\n",
            PolygonVertexIndices {
                vertex: 10,
                texture_coordinates: None,
                normal: Some(5),
            }
        ))
    );
    assert_eq!(
        parse_polygon_vertex_indices("1/5\t"),
        Ok((
            "\t",
            PolygonVertexIndices {
                vertex: 1,
                texture_coordinates: Some(5),
                normal: None,
            }
        ))
    );
    assert_eq!(
        parse_polygon_vertex_indices("1/"),
        Ok((
            "/",
            PolygonVertexIndices {
                vertex: 1,
                texture_coordinates: None,
                normal: None,
            }
        ))
    );
    assert!(parse_polygon_vertex_indices("/1").is_err());
}

#[derive(Debug, PartialEq)]
pub struct PolygonIndices {
    points: Vec<PolygonVertexIndices>,
}

impl PolygonIndices {
    pub fn to_polygon(&self) -> Result<Polygon, &'static str> {
        Err("Not implemented") // TODO: take refs to lists of vertices, textures, normals
    }
}

pub fn parse_polygon_indices(input: &str) -> IResult<&str, PolygonIndices> {
    let (input, points) = preceded(tws(tag("f")), many1(tws(parse_polygon_vertex_indices)))(input)?;
    Ok((input, PolygonIndices { points }))
}

#[test]
fn test_parse_polygon_indices() {
    assert_eq!(
        parse_polygon_indices("f 1/2/3 4/5/9\r\n"),
        Ok((
            "",
            PolygonIndices {
                points: vec![
                    PolygonVertexIndices {
                        vertex: 1,
                        texture_coordinates: Some(2),
                        normal: Some(3),
                    },
                    PolygonVertexIndices {
                        vertex: 4,
                        texture_coordinates: Some(5),
                        normal: Some(9),
                    },
                ]
            }
        ))
    )
}

#[derive(Debug, PartialEq)]
pub struct PolygonVertex {
    vertex: Vertex,
    texture_coordinates: TextureCoordinates,
    normal: Vector,
}

pub struct Polygon {
    points: Vec<PolygonVertex>,
}

/*
impl Polygon {
    pub fn to_triangles(&self) -> Vec<Triangle> {
        let mut result = vec![];
        // TODO: split intelligently
        result
    }
}
*/
