
use cg_rust::vector::Vec2d;
use cg_rust::polygon::{Polygon, PolygonFloat};
use plotters::prelude::*;

const OUT_FILE_NAME: &str = "examples/plotters/plot-data/polygon.png";

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let root = BitMapBackend::new(OUT_FILE_NAME, (1024, 768)).into_drawing_area();

    let _ = root.fill(&WHITE);

    let mut chart = ChartBuilder::on(&root)
        .caption("Hexagon", ("sans-serif", 50))
        .build_cartesian_2d(-2.0..2.0, -1.5..1.5)?;

    let polygon= Polygon::<Vec2d>::regular(Vec2d::new(0.0, 0.0), 1.0, 6);

    let poly_vertices = polygon.get_points();
    let mut vertices: Vec<(f64, f64)> = Vec::<(f64, f64)>::new();
    for poly_vertex in poly_vertices {
        vertices.push((poly_vertex.x(), poly_vertex.y()));
    }

    chart.draw_series(std::iter::once(plotters::element::Polygon::new(vertices.clone(), RED.mix(0.2))))?;
    vertices.push((poly_vertices[0].x(), poly_vertices[0].y()));
    chart.draw_series(std::iter::once(PathElement::new(vertices, RED)))?;


    root.present().expect("[cg-rust]: Unable to write result to file, please make sure 'plot-data' dir exists under current directory!");
    println!("Result has been saved to {}", OUT_FILE_NAME);
    Ok(())
}