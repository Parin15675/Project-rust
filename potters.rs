use plotters::prelude::*;

pub fn plot_bar_chart(data: &[DataRow]) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("bar_chart.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    let max_value = data.iter()
        .map(|row| row.values["value"].parse::<i32>().unwrap_or(0))
        .max()
        .unwrap_or(0);

    let chart = ChartBuilder::on(&root)
        .caption("Bar Chart", ("Arial", 40).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(40)
        .build_cartesian_2d(0..data.len(), 0..max_value)?;

    chart.configure_mesh().draw()?;

    for (index, row) in data.iter().enumerate() {
        let value: i32 = row.values["value"].parse()?;
        chart.draw_series(Rectangle::new([(index, 0), (index + 1, value)], BLUE.filled()))?;
    }

    Ok(())
}
