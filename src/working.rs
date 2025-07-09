// Older iteration of this code, one of the first times I got it working

use std::{cmp::Ordering, collections::BTreeMap, fmt::Error, ops::Add};
use clap::{Parser, arg, command};
use image::{DynamicImage, GenericImage, GenericImageView, ImageReader, Rgba};

trait GraphVertex<L>
where Self: Sized,
{
    fn get_edges(&self) -> impl Iterator<Item = GraphEdge<L, Self>>;
}

struct GraphEdge<L, T: GraphVertex<L>> {
    weight: L,
    vertex: T,
}

fn dijkstra<L, T: GraphVertex<L>>(start: T, end: T) -> Result<Vec<T>, Error>
where
    L: Add<Output = L> + Ord + Copy + Default,
    T: Clone + Ord,
{
    let mut distances = BTreeMap::new();
    let mut unvisited = Vec::new();
    let mut shortest_connections = BTreeMap::new();

    distances.insert(start.clone(), L::default());

    if start == end {
        return Ok(Vec::default());
    }

    start.get_edges().for_each(|GraphEdge { weight, vertex }| {
        unvisited.push(vertex.clone());
        distances.insert(vertex.clone(), weight);
        shortest_connections.insert(vertex.clone(), start.clone());
    });

    // unvisited.sort(|a, b| {});
    unvisited.sort_by(|a, b| distances[a].cmp(&distances[b]));

    while let Some(current_node) = unvisited.pop() {
        if current_node == end {
            let mut ret = Vec::new();
            let mut c = current_node.clone();
            while let Some(n) = shortest_connections.get(&c) {
                ret.push(n.clone());
                c = n.clone();
            }
            ret.reverse();
            return Ok(ret);
        }
        let node_distance = distances[&current_node];
        current_node
            .get_edges()
            .for_each(|GraphEdge { weight, vertex }| {
                if let Some(dist) = distances.get(&vertex) {
                    if *dist > node_distance + weight {
                        distances.insert(vertex.clone(), node_distance + weight);
                        shortest_connections.insert(vertex.clone(), current_node.clone());
                    }
                } else {
                    shortest_connections.insert(vertex.clone(), current_node.clone());
                    distances.insert(vertex.clone(), node_distance + weight);
                    unvisited.push(vertex.clone());
                }
            });

        unvisited.sort_by(|a, b| distances[a].cmp(&distances[b]));
    }
    return Err(Error);
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg()]
    input_file: std::path::PathBuf,

    #[arg()]
    output_file: std::path::PathBuf,
}

#[derive(Clone, PartialEq, Copy)]
struct ImgVertex<'a> {
    x: u32,
    y: u32,
    img: &'a DynamicImage,
}

fn is_wall(p: image::Rgba<u8>) -> bool {
    p == image::Rgba([0, 0, 0, 255])
}

fn get_pixel_opt(img: &DynamicImage, x: u32, y: u32) -> Option<Rgba<u8>> {
    if x as u32 >= img.width() || y as u32 >= img.width() {
        return None;
    }
    Some(img.get_pixel(x as u32, y as u32))
}

impl<'a> Eq for ImgVertex<'a> {}

impl<'a> PartialOrd for ImgVertex<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for ImgVertex<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        return match self.x.cmp(&other.x) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => self.y.cmp(&other.y),
        };
    }
}

impl<'a> GraphVertex<i32> for ImgVertex<'a> {
    fn get_edges(&self) -> impl Iterator<Item = GraphEdge<i32, Self>> {
        let mut v = Vec::new();

        if let Some(p) = get_pixel_opt(
            self.img,
            self.x.wrapping_add_signed(1),
            self.y.wrapping_add_signed(0),
        ) {
            if !is_wall(p) {
                v.push(GraphEdge {
                    weight: 1,
                    vertex: Self {
                        x: self.x.wrapping_add_signed(1),
                        y: self.y.wrapping_add_signed(0),
                        img: self.img,
                    },
                });
            }
        }
        if let Some(p) = get_pixel_opt(
            self.img,
            self.x.wrapping_add_signed(-1),
            self.y.wrapping_add_signed(0),
        ) {
            if !is_wall(p) {
                v.push(GraphEdge {
                    weight: 1,
                    vertex: Self {
                        x: self.x.wrapping_add_signed(-1),
                        y: self.y.wrapping_add_signed(0),
                        img: self.img,
                    },
                });
            }
        }
        if let Some(p) = get_pixel_opt(
            self.img,
            self.x.wrapping_add_signed(0),
            self.y.wrapping_add_signed(1),
        ) {
            if !is_wall(p) {
                v.push(GraphEdge {
                    weight: 1,
                    vertex: Self {
                        x: self.x.wrapping_add_signed(0),
                        y: self.y.wrapping_add_signed(1),
                        img: self.img,
                    },
                });
            }
        }
        if let Some(p) = get_pixel_opt(
            self.img,
            self.x.wrapping_add_signed(0),
            self.y.wrapping_add_signed(-1),
        ) {
            if !is_wall(p) {
                v.push(GraphEdge {
                    weight: 1,
                    vertex: Self {
                        x: self.x.wrapping_add_signed(0),
                        y: self.y.wrapping_add_signed(-1),
                        img: self.img,
                    },
                });
            }
        }

        v.into_iter()
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let img = ImageReader::open(args.input_file)?.decode()?;

    let start = (1..img.height() - 1)
        .map(|y| (0u32, y))
        .find(|(x, y)| !is_wall(img.get_pixel(*x, *y)))
        .unwrap();
    let end = (1..img.height() - 1)
        .map(|y| (img.width() - 1, y))
        .find(|(x, y)| !is_wall(img.get_pixel(*x, *y)))
        .unwrap();

    let start_vertex = ImgVertex {
        x: start.0,
        y: start.1,
        img: &img,
    };
    let end_vertex = ImgVertex {
        x: end.0,
        y: end.1,
        img: &img,
    };

    let res = Vec::from_iter(dijkstra(start_vertex, end_vertex).unwrap());

    let mut out_image = img.clone();

    res.iter().for_each(|v| {
        out_image.put_pixel(v.x, v.y, Rgba([255, 0, 0, 255]));
    });

    out_image.save(args.output_file).unwrap();

    return Ok(());
}
