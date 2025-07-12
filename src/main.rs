use clap::{Parser, arg, command};
use image::{DynamicImage, GenericImage, GenericImageView, ImageReader, Rgba};

mod pathfinding;

#[derive(Parser)]
#[command(version = "1.0.0")]
struct Args {
    #[arg(value_name = "Path to input image")]
    input_file: std::path::PathBuf,

    #[arg(value_name = "Path to output image")]
    output_file: std::path::PathBuf,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Copy)]
struct Point {
    x: u32,
    y: u32,
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

    let start_vertex = Point {
        x: start.0,
        y: start.1,
    };
    let end_vertex = Point { x: end.0, y: end.1 };

    let get_weighted_edges = |vertex: &Point| {
        let mut res = Vec::new();

        for (x, y) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            if let Some(p) = get_pixel_opt(
                &img,
                vertex.x.wrapping_add_signed(x),
                vertex.y.wrapping_add_signed(y),
            ) {
                if !is_wall(p) {
                    res.push((
                        Point {
                            x: vertex.x.wrapping_add_signed(x),
                            y: vertex.y.wrapping_add_signed(y),
                        },
                        1,
                    ));
                }
            }
        }

        res.into_iter()
    };

    let get_edges = |vertex: &Point| get_weighted_edges(vertex).map(|(v, _)| v);

    let res = Vec::from_iter(pathfinding::dijkstra(start_vertex, end_vertex, get_weighted_edges).unwrap());
    let res = Vec::from_iter(pathfinding::depth_first_search(start_vertex, end_vertex, get_edges).unwrap());

    let mut out_image = img.clone();

    res.iter().for_each(|Point { x, y }| {
        out_image.put_pixel(*x, *y, Rgba([0xff, 0x66, 0x00, 255]));
    });

    out_image.save(args.output_file).unwrap();

    return Ok(());
}