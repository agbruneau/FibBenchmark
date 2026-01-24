use crate::data_parser::BenchmarkData;
use plotly::common::{Mode, Title};
use plotly::layout::{Axis, AxisType, Layout};
use plotly::{Plot, Scatter};
use std::path::Path;

use plotly::ImageFormat;

pub fn generate_charts(data: &BenchmarkData, output_dir: &str) {
    let dir = Path::new(output_dir);
    if let Err(e) = std::fs::create_dir_all(dir) {
        eprintln!("Failed to create output directory: {}", e);
        return;
    }

    // 1. Complexity Chart
    let mut plot = Plot::new();

    let n_values: Vec<u64> = data.complexity.iter().map(|p| p.n).collect();
    let iter_times: Vec<u128> = data.complexity.iter().map(|p| p.iterative_ns).collect();
    let matrix_times: Vec<u128> = data.complexity.iter().map(|p| p.matrix_ns).collect();

    let trace1 = Scatter::new(n_values.clone(), iter_times)
        .name("Iterative")
        .mode(Mode::LinesMarkers);

    let trace2 = Scatter::new(n_values, matrix_times)
        .name("Matrix Exponentiation")
        .mode(Mode::LinesMarkers);

    plot.add_trace(trace1);
    plot.add_trace(trace2);

    let layout = Layout::new()
        .title(Title::new("Algorithm Complexity Comparison"))
        .x_axis(Axis::new().title(Title::new("n (Fibonacci Index)")))
        .y_axis(Axis::new().title(Title::new("Time (ns)")));

    plot.set_layout(layout);
    save_plot(&mut plot, dir, "complexity_chart");

    // 2. Binet Accuracy Chart
    let mut plot = Plot::new();

    let n_values: Vec<u64> = data.accuracy.iter().map(|p| p.n).collect();
    // Use log scale for error if possible or just raw error
    let errors: Vec<f64> = data.accuracy.iter().map(|p| p.abs_error).collect();

    let trace = Scatter::new(n_values, errors)
        .name("Absolute Error")
        .mode(Mode::LinesMarkers);

    plot.add_trace(trace);

    let layout = Layout::new()
        .title(Title::new("Binet Formula Accuracy"))
        .x_axis(Axis::new().title(Title::new("n")))
        .y_axis(Axis::new().title(Title::new("Absolute Error")));

    plot.set_layout(layout);
    save_plot(&mut plot, dir, "binet_accuracy_chart");

    // 3. Golden Ratio Convergence
    let mut plot = Plot::new();

    let n_values: Vec<u64> = data.golden_ratio.iter().map(|p| p.n).collect();
    let errors: Vec<f64> = data.golden_ratio.iter().map(|p| p.error_from_phi).collect();

    let trace = Scatter::new(n_values, errors)
        .name("Deviation from Phi")
        .mode(Mode::LinesMarkers);

    plot.add_trace(trace);

    let layout = Layout::new()
        .title(Title::new("Golden Ratio Convergence"))
        .x_axis(Axis::new().title(Title::new("n")))
        .y_axis(
            Axis::new()
                .title(Title::new("Error |Ratio - Phi|"))
                .type_(AxisType::Log),
        ); // Log scale is useful here

    plot.set_layout(layout);
    save_plot(&mut plot, dir, "golden_ratio_chart");

    generate_index_html(output_dir);
}

fn save_plot(plot: &mut Plot, dir: &Path, base_name: &str) {
    let html_path = dir.join(format!("{}.html", base_name));
    plot.write_html(&html_path);

    let png_path = dir.join(format!("{}.png", base_name));
    plot.write_image(&png_path, ImageFormat::PNG, 1024, 768, 1.0);

    let svg_path = dir.join(format!("{}.svg", base_name));
    plot.write_image(&svg_path, ImageFormat::SVG, 1024, 768, 1.0);
}

fn generate_index_html(output_dir: &str) {
    let html = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Fibonacci Benchmark Report</title>
    <style>
        body { font-family: system-ui, -apple-system, sans-serif; max-width: 1200px; margin: 0 auto; padding: 20px; background: #f4f4f9; }
        h1 { color: #333; border-bottom: 2px solid #ddd; padding-bottom: 10px; }
        .dashboard { display: grid; grid-template-columns: repeat(auto-fit, minmax(600px, 1fr)); gap: 20px; margin-top: 20px; }
        .chart-card { background: white; padding: 20px; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.1); }
        h2 { margin-top: 0; color: #555; font-size: 1.2rem; }
        iframe { width: 100%; height: 500px; border: none; }
    </style>
</head>
<body>
    <h1>🚀 Fibonacci Benchmark Suite Report</h1>
    <p>Generated automated report for Fibonacci algorithm performance and analysis.</p>
    
    <div class="dashboard">
        <div class="chart-card">
            <h2>Algorithm Complexity</h2>
            <iframe src="complexity_chart.html"></iframe>
        </div>
        <div class="chart-card">
            <h2>Binet Formula Accuracy</h2>
            <iframe src="binet_accuracy_chart.html"></iframe>
        </div>
        <div class="chart-card">
            <h2>Golden Ratio Convergence</h2>
            <iframe src="golden_ratio_chart.html"></iframe>
        </div>
    </div>
</body>
</html>
"#;

    let dir = Path::new(output_dir);
    std::fs::write(dir.join("index.html"), html).ok();
}

#[cfg(test)]
#[path = "chart_generator_tests.rs"]
mod tests;
