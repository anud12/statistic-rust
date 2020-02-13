use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Arbuthnot {
    id: String,
    year: String,
    boys: String,
    girls: String,
}


fn read_data() -> Vec<Arbuthnot> {
    let collection: Vec<std::result::Result<Arbuthnot, _>> = csv::Reader::from_path("./data/arbuthnot.csv")
    .expect("Boo")
    .deserialize()
    .collect();

    let mut vec:Vec<Arbuthnot> = Vec::new();

    for element in collection {
        vec.push(element.expect(""))
    }

    vec
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = read_data();
    build_ui()
}

fn build_ui() -> Result<(), Box<dyn std::error::Error>> {
    use plotters::prelude::*;

    let root = BitMapBackend::new("0.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("y=x^2", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_ranged(-1f32..1f32, -0.1f32..1f32)?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(
            (-50..=50).map(|x| x as f32 / 50.0).map(|x| (x, x * x)),
            &BLUE,
        ))?
        .label("y = x^2")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;
    Ok(())
}