
use cg_rust::vector::Vec2d;
use cg_rust::polygon::Polygon;
use plotters::prelude::*;

const OUT_FILE_NAME: &str = "examples/plotters/plot-data/polygon.png";

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let root = BitMapBackend::new(OUT_FILE_NAME, (1024, 768)).into_drawing_area();

    let _ = root.fill(&WHITE);

    let mut chart = ChartBuilder::on(&root)
        .caption("Simple Polygon", ("sans-serif", 50))
        .build_cartesian_2d(-2.0..2.0, -1.5..1.5)?;

    let mut polygon= Polygon::<Vec2d>::new();
    
    polygon.push(Vec2d::new(-1.0_f64, -1.0_f64));
    polygon.push(Vec2d::new(1.0_f64, -1.0_f64));
    polygon.push(Vec2d::new(1.0_f64, 1.0_f64));
    polygon.push(Vec2d::new(-1.0_f64, 1.0_f64));

    let poly_vertices = polygon.get_points();
    let mut vertices: Vec<(f64, f64)> = Vec::<(f64, f64)>::new();
    for poly_vertex in poly_vertices {
        vertices.push((poly_vertex.x(), poly_vertex.y()));
    }
    vertices.push((poly_vertices[0].x(), poly_vertices[0].y()));

    chart.draw_series(std::iter::once(PathElement::new(vertices, RED)))?;

    root.present().expect("[cg-rust]: Unable to write result to file, please make sure 'plot-data' dir exists under current directory!");
    println!("Result has been saved to {}", OUT_FILE_NAME);
    Ok(())
}