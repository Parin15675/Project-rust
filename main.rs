mod data_loading;
mod templating;
use csv::ReaderBuilder;
use plotters::prelude::*;
use rand::Rng;
use csv::Reader;

fn plot_bar_chart(
    data: &[data_loading::DataRow],
    output_file: &str,
    chart_title: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(output_file, (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    let colors = [RED, GREEN, BLUE, YELLOW, MAGENTA, CYAN, BLACK];

    let max_age = data
        .iter()
        .filter_map(|row| row.values.get("age").and_then(|s| s.parse::<i32>().ok()))
        .max()
        .unwrap_or(0);

    let mut chart = ChartBuilder::on(&root)
        .caption(chart_title, ("Arial", 40).into_font())
        .margin(5)
        .x_label_area_size(50)
        .y_label_area_size(40)
        .build_cartesian_2d(0..data.len(), 0..max_age)?;

    chart
        .configure_mesh()
        .x_labels(data.len())
        .x_label_formatter(&|x| {
            data.get(*x as usize)
                .and_then(|row| row.values.get("name"))
                .map(ToString::to_string)
                .unwrap_or_else(|| "".to_string())
        })
        .draw()?;

    for (index, row) in data.iter().enumerate() {
        if let Some(age_str) = row.values.get("age") {
            if let Ok(age) = age_str.parse::<i32>() {
                let color = colors[index % colors.len()];
                chart.draw_series(std::iter::once(Rectangle::new(
                    [(index, 0), (index + 1, age)],
                    color.filled(),
                )))?;
            }
        }
    }

    Ok(())
}

fn get_user_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    input = input.trim().to_string();
    
    // Check for 'quit' input
    if &input == "q" || &input == "quit" {
        println!("Exiting program.");
        std::process::exit(0);  // Exit the program
    }
    
    input
}


fn main() {
    loop {
        println!("Choose the type of chart you want to generate:");
        println!("1. Bar Chart (Expected Columns: 'Category', 'Value')");
        println!("2. Scatter Plot (Expected Columns for two CSVs: 'X1', 'Y1' and 'X2', 'Y2')");
        println!("3. Pie Chart (Expected Columns: 'Category', 'Percentage')");
        println!("4. Line and Area Chart (Expected Columns: 'Date', 'Value')");
        println!("5. Radar Chart (Expected Columns: 'Label', 'Value')"); // Added option for Radar Chart
        println!("'q' or 'quit' to quit program.");

        let mut chart_choice = get_user_input("Enter your choice (1-5, q to quit):");

        if chart_choice == "q" || chart_choice == "quit" {
            println!("Exiting program.");
            break;
        }

        while !["1", "2", "3", "4", "5"].contains(&chart_choice.as_str()) {
            println!("Invalid choice! Please enter a number between 1 and 5.");
            chart_choice = get_user_input("Enter your choice (1-5):");
        }

        match chart_choice.as_str() {
            "1" => {
                let csv_file_name = get_valid_filename("Enter the name of the CSV file (e.g., 'data.csv'):", ".csv");
                let output_file_name = get_valid_output_filename("Enter the desired name for the PNG output file (.g., 'output.png'):", ".png");
                let chart_title = get_user_input("Enter the title for the chart:");

                match data_loading::load_csv(&csv_file_name) {
                    Ok(data) => {
                        let template_str = "type:table,columns:name|age|gender|city";
                        let template = templating::parse_template(template_str);

                        if let Err(e) = plot_bar_chart(&data, &output_file_name, &chart_title) {
                            eprintln!("Error plotting bar chart: {}", e);
                        }
                    }
                    Err(e) => {
                        eprintln!("Error loading CSV file: {}", e);
                    }
                }
            }
            "2" => {
                let csv_file_name1 = get_valid_filename(
                    "Enter the name of the first CSV file for scatter plot (e.g., 'data1.csv'):",
                    ".csv",
                );
                let csv_file_name2 = get_valid_filename(
                    "Enter the name of the second CSV file for scatter plot (e.g., 'data2.csv'):",
                    ".csv",
                );
                let output_file_name = get_valid_output_filename(
                    "Enter the desired name for the PNG output file (e.g., 'output.png'):",
                    ".png",
                );
                let chart_title = get_user_input("Enter the title for the chart:");

                if let Err(err) = draw_scatter_plot(
                    &csv_file_name1,
                    &csv_file_name2,
                    &output_file_name,
                    &chart_title,
                ) {
                    eprintln!("Error drawing the scatter plot: {}", err);
                }
            }
            "3" => {
                let csv_file_name = get_valid_filename(
                    "Enter the name of the CSV file (e.g., 'data.csv'):",
                    ".csv",
                );
                let output_file_name = get_valid_output_filename(
                    "Enter the desired name for the PNG output file (e.g., 'output.png'):",
                    ".png",
                );

                let chart_title = get_user_input("Enter the title for the chart:");
                if let Err(err) = draw_pie_chart_to_png(&csv_file_name, &output_file_name,&chart_title) {
                    eprintln!("Error drawing the pie chart: {}", err);
                }
            }
            "4" => {
                let csv_file_name = get_valid_filename(
                    "Enter the name of the CSV file (e.g., 'data.csv'):",
                    ".csv",
                );
                let output_file_name = get_valid_output_filename(
                    "Enter the desired name for the PNG output file (e.g., 'output.png'):",
                    ".png",
                );

                let chart_title = get_user_input("Enter the title for the chart:");

                if let Err(err) = draw_line_and_area(&csv_file_name, &output_file_name, &chart_title) {
                    eprintln!("Error drawing the line and area chart: {}", err);
                }
            }
            "5" => { 
                let csv_file_name = get_valid_filename("Enter the name of the CSV file for radar chart (e.g., 'radar_data.csv'):", ".csv");
                let output_file_name = get_valid_output_filename("Enter the desired name for the SVG output file (e.g., 'radar_chart.png):", ".png");
                let chart_title = get_user_input("Enter the title for the chart:");

                match read_from_csv_radar(&csv_file_name) {
                    Ok(data) => {
                        if let Err(e) = draw_radar_chart(&data, &output_file_name) {
                            eprintln!("Error plotting radar chart: {}", e);
                        } else {
                            println!("Radar chart generated successfully!");
                        }
                    }
                    Err(e) => {
                        eprintln!("Error loading CSV file: {}", e);
                    }
                }
            }
            
            _ => unreachable!(),
        }
    }
}

fn get_valid_filename(prompt: &str, extension: &str) -> String {
    loop {
        let filename = get_user_input(prompt);
        if filename.to_lowercase().ends_with(extension) && filename.to_lowercase() != "repeat" {
            return filename;
        } else if filename.to_lowercase() == "repeat" {
            println!("Filename 'Repeat' is not allowed! Please provide a different name.");
        } else {
            println!(
                "Invalid filename. Please ensure the filename ends with {}",
                extension
            );
        }
    }
}

use std::path::Path;

fn get_valid_output_filename(prompt: &str, extension: &str) -> String {
    loop {
        let filename = get_user_input(prompt);
        if Path::new(&filename).exists() {
            println!(
                "The file '{}' already exists. Please provide a different name.",
                filename
            );
        } else if filename.to_lowercase().ends_with(extension)
            && filename.to_lowercase() != "repeat"
        {
            return filename;
        } else if filename.to_lowercase() == "repeat" {
            println!("Filename 'Repeat' is not allowed! Please provide a different name.");
        } else {
            println!(
                "Invalid filename. Please ensure the filename ends with {}",
                extension
            );
        }
    }
}

use std::error::Error;
use std::fs::File;
type DataPoint = (i32, i32);

// Function to read data from CSV
fn read_data_from_csv_scatter(path: &str) -> Result<Vec<DataPoint>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(File::open(path)?);
    let mut data = Vec::new();

    for result in rdr.records() {
        let record = result?;
        let x: i32 = record[0].parse()?;
        let y: i32 = record[1].parse()?;
        data.push((x, y));
    }

    Ok(data)
}

fn validate_csv_data_for_scatter(filename: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(filename)?;
    let mut rdr = Reader::from_reader(file);

    for (line, result) in rdr.records().enumerate() {
        let record = result?;

        // Check if there are exactly 2 columns
        if record.len() != 2 {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Line {}: Expected exactly 2 columns for x and y values.", line + 1),
            )));
        }

        // Validate the x and y columns to ensure they can be parsed as i32
        if record[0].trim().parse::<i32>().is_err() {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Line {}: Invalid x value. Expected an integer.", line + 1),
            )));
        }

        if record[1].trim().parse::<i32>().is_err() {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Line {}: Invalid y value. Expected an integer.", line + 1),
            )));
        }
    }

    Ok(())
}


fn draw_scatter_plot(
    input_file1: &str,
    input_file2: &str,
    output_file: &str,
    chart_title: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    validate_csv_data_for_scatter(input_file1)?;
    validate_csv_data_for_scatter(input_file2)?;
    let data1 = read_data_from_csv_scatter(input_file1)?;
    let data2 = read_data_from_csv_scatter(input_file2)?;

    let root_area = BitMapBackend::new(output_file, (600, 400)).into_drawing_area();
    root_area.fill(&WHITE)?;

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption(chart_title, ("Arial", 40).into_font())
        .build_cartesian_2d(-10..50, -10..50)?;

    ctx.configure_mesh().draw()?;

    // Draw the data series
    ctx.draw_series(
        data1
            .iter()
            .map(|point| TriangleMarker::new(*point, 5, &BLUE)),
    )?;
    ctx.draw_series(data2.iter().map(|point| Circle::new(*point, 5, &RED)))
        .unwrap();

    Ok(())
}

// Define a struct to hold the data from the CSV file
struct PieChartData {
    label: String,
    value: f64,
    color: RGBColor,
}

impl PieChartData {
    fn new(label: String, value: f64, color: RGBColor) -> Self {
        Self {
            label,
            value,
            color,
        }
    }
}

fn read_data_from_csv_pie(filename: &str) -> Result<Vec<PieChartData>, Box<dyn Error>> {
    let mut data = Vec::new();
    let file = File::open(filename)?;
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);

    for (line, result) in rdr.records().enumerate() {
        let record = result?;
        let label = record[0].to_string();

        let value: f64 = record
            .get(1)
            .ok_or(format!("Missing value on line {}", line + 2))?
            .trim()
            .parse()
            .map_err(|e| format!("Error on line {}: {}", line + 2, e))?;

        let mut rng = rand::thread_rng();

        let color = match record.len() {
            2..=4 => RGBColor(
                rng.gen_range(0..=255),
                rng.gen_range(0..=255),
                rng.gen_range(0..=255),
            ), // Random color if not specified
            5 => RGBColor(
                record[2]
                    .trim()
                    .parse()
                    .map_err(|e| format!("Error on line {}: {}", line + 2, e))?,
                record[3]
                    .trim()
                    .parse()
                    .map_err(|e| format!("Error on line {}: {}", line + 2, e))?,
                record[4]
                    .trim()
                    .parse()
                    .map_err(|e| format!("Error on line {}: {}", line + 2, e))?,
            ),
            _ => {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "Unsupported number of columns",
                )))
            }
        };

        let pie_data = PieChartData::new(label, value, color);
        data.push(pie_data);
    }

    Ok(data)
}

fn validate_csv_data_pie(filename: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(filename)?;
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);

    for (line, result) in rdr.records().enumerate() {
        let record = result?;

        // Check if there are at least 2 columns (label and value)
        if record.len() < 2 {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Line {}: Expected at least 2 columns for label and value.", line + 2),
            )));
        }

        // Validate the value column to ensure it can be parsed as f64
        if record[1].trim().parse::<f64>().is_err() {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Line {}: Invalid value. Expected a number.", line + 2),
            )));
        }

        // If color columns are provided, validate them too
        if record.len() >= 5 {
            for i in 2..5 {
                if record[i].trim().parse::<u8>().is_err() {
                    return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        format!("Line {}: Invalid RGB color value at column {}. Expected a number between 0 and 255.", line + 2, i + 1),
                    )));
                }
            }
        }
    }

    Ok(())
}


fn draw_pie_chart_to_png(input_file: &str, output_file: &str, chart_title: &str) -> Result<(), Box<dyn Error>> {
    validate_csv_data_pie(&input_file)?;
    
    let root = BitMapBackend::new(output_file, (350, 350)).into_drawing_area();
    root.fill(&WHITE)?;

    let center = (175, 175);  // Adjusted center
    let radius = 100.0;

    // Read data from CSV file
    let mut data = read_data_from_csv_pie(input_file)?;

    // Adjust the values in the data vector to make the total 100
    let total: f64 = data.iter().map(|d| d.value).sum();
    let adjustment_factor = 100.0 / total;
    for datum in &mut data {
        datum.value *= adjustment_factor;
    }

    let mut start_angle = 0.0;

    for pie_data in &data {
        let value = pie_data.value;
        let end_angle = start_angle + value / 100.0 * 2.0 * std::f64::consts::PI;

        let points: Vec<_> = std::iter::once(center)
            .chain((0..=100).map(|p| {
                let angle = start_angle + (end_angle - start_angle) * p as f64 / 100.0;
                (
                    (center.0 as f64 + radius * angle.cos()).round() as i32,
                    (center.1 as f64 + radius * angle.sin()).round() as i32,
                )
            }))
            .collect();

        let polygon = Polygon::new(points, pie_data.color.filled());
        root.draw(&polygon)?;

        // Calculate label position
        let label_angle = (start_angle + end_angle) / 2.0;
        let label_distance = radius + 20.0;
        let label_x = (center.0 as f64 + label_distance * label_angle.cos()).round() as i32;
        let label_y = (center.1 as f64 + label_distance * label_angle.sin()).round() as i32;

        // Draw the label
        let label_style = TextStyle::from(("sans-serif", 15).into_font()).color(&BLACK);
        root.draw_text(&pie_data.label, &label_style, (label_x, label_y))?;

        start_angle = end_angle;
    }

    // Draw the title
    let title_style = TextStyle::from(("sans-serif", 24).into_font()).color(&BLACK);
    root.draw_text(chart_title, &title_style, (175, 25))?;  // Adjust position as needed

    Ok(())
}


use csv;
use plotters::backend::BitMapBackend;
use plotters::drawing::IntoDrawingArea;
use plotters::prelude::{AreaSeries, ChartBuilder, LabelAreaPosition, RED, WHITE};
use plotters::style::Color;





fn validate_csv_data_for_line_area(filename: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(filename)?;
    let mut rdr = Reader::from_reader(file);

    for (line, result) in rdr.records().enumerate() {
        let record = result?;

        // Check if there are exactly 2 columns
        if record.len() != 2 {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Line {}: Expected exactly 2 columns for x and y values.", line + 1),
            )));
        }

        // Validate the x and y columns to ensure they can be parsed as i32
        if record[0].trim().parse::<i32>().is_err() {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Line {}: Invalid x value. Expected an integer.", line + 1),
            )));
        }

        if record[1].trim().parse::<i32>().is_err() {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Line {}: Invalid y value. Expected an integer.", line + 1),
            )));
        }
    }

    Ok(())
}

fn read_data_from_csv_line_area(filename: &str) -> Result<Vec<(i32, i32)>, Box<dyn std::error::Error>> {
    let mut data = Vec::new();
    let file = File::open(filename)?;
    let mut rdr = csv::Reader::from_reader(file);

    for result in rdr.records() {
        let record = result?;
        let label: i32 = record[0].parse()?;
        let value: i32 = record[1].parse()?;
        data.push((label, value));
    }
    Ok(data)
}

fn draw_line_and_area(
    input_file: &str,
    output_file: &str,
    chart_title: &str
) -> Result<(), Box<dyn std::error::Error>> {
    validate_csv_data_for_line_area(input_file)?;
    let data = read_data_from_csv_line_area(input_file)?;

    // Determine the maximum x and y values for the range
    let max_x = data.iter().map(|(x, _)| *x).max().unwrap_or(10) + 1;  // +1 to make the plot a bit more spacious
    let max_y = data.iter().map(|(_, y)| *y).max().unwrap_or(50) + 5;  // +5 for the same reason

    let root_area = BitMapBackend::new(output_file, (600, 400)).into_drawing_area();
    root_area.fill(&WHITE)?;

    let mut ctx = ChartBuilder::on(&root_area)
    .set_label_area_size(LabelAreaPosition::Left, 40)
    .set_label_area_size(LabelAreaPosition::Bottom, 40)
    .caption(chart_title, ("sans-serif", 40))
    .build_cartesian_2d(0..max_x, 0..max_y)?
    ;


    ctx.configure_mesh().draw()?;

    ctx.draw_series(
        AreaSeries::new(data.iter().map(|(x, y)| (*x, *y)), 0, &RED.mix(0.2)).border_style(&RED),
    )
    .unwrap();

    Ok(())
}




fn read_from_csv_radar(file_path: &str) -> Result<Vec<(String, f32)>, Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new().from_path(file_path)?;
    let mut data = Vec::new();

    for result in rdr.records() {
        let record = result?;
        let label = record[0].to_string();
        let value: f32 = record[1].parse()?;
        data.push((label, value));
    }

    Ok(data)
}


use plotters::style::IntoFont;
use plotters::element::Circle;
use plotters::prelude::*;




fn draw_radar_chart(data: &[(String, f32)], output_file: &str) -> Result<(), Box<dyn Error>> {
    // Prepare drawing area
    let root = BitMapBackend::new(output_file, (800, 800)).into_drawing_area();
    root.fill(&WHITE)?;

    let max_val = data.iter().map(|(_, v)| *v).fold(0.0, f32::max);

    // Create chart
    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(-350..350, -350..350)?;

// Assuming you've already set up your drawing area, backend, etc.



    let center = (400.0, 400.0);  // Center coordinates
    let max_radius = 350.0;      // Maximum radius value

    // Ensure the backend and drawing root is set up before this
    // For example: 
    // let root = drawing_backend.into_drawing_area();

    for i in 0..5 {
        let radius_percentages = [0.0, 0.25, 0.5, 0.75, 1.0];
        let label_values = [0, 25, 50, 75, 100];

        let radius_percentage = radius_percentages[i];
        let label_value = label_values[i];
        
        let radius = max_radius * radius_percentage;

        // Drawing the circle
        root.draw(&Circle::new((center.0 as i32, center.1 as i32), radius as i32,  &BLACK.mix(0.1)))?;

        // Calculate label position
        let label_x = center.0 + radius;  // Positioned to the right of the circle
        let label_y = center.1;

        // Drawing the label
        root.draw(&Text::new(
            label_value.to_string(),
            (label_x as i32, label_y as i32), // Convert to integers for drawing
            ("Arial", 15).into_font()
        ))?;
    }

    let step_angle = 2.0 * std::f32::consts::PI / data.len() as f32;
    let mut radar_points = Vec::new();

    for (index, (label, value)) in data.iter().enumerate() {
        let scaled_value = (*value / max_val) * 350.0;
        let x = center.0 as f32 + scaled_value * (step_angle * index as f32).cos();
        let y = center.1 as f32 - scaled_value * (step_angle * index as f32).sin();

        let x_label = center.0 as f32 + 375.0 * (step_angle * index as f32).cos();
        let y_label = center.1 as f32 - 375.0 * (step_angle * index as f32).sin();
        
        root.draw(&Text::new(label.to_string(), (x_label as i32, y_label as i32), ("Arial", 15)))?;

        radar_points.push((x as i32, y as i32));
    }
    radar_points.push(radar_points[0]);

    root.draw(&Polygon::new(radar_points.clone(), RED.mix(0.5).filled()))?;

    Ok(())
}

