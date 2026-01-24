pub mod chart_generator;
pub mod data_parser;

pub fn generate_report(input_dir: &str, output_dir: &str) -> std::io::Result<()> {
    println!("📊 Generating Visualization Report...");
    println!("   Input: {}", input_dir);
    println!("   Output: {}", output_dir);

    // 1. Load Data
    let data = data_parser::BenchmarkData::load(input_dir)?;
    println!("   ✓ Data loaded successfully");

    // 2. Generate Charts
    chart_generator::generate_charts(&data, output_dir);
    println!("   ✓ Charts generated in {}", output_dir);

    Ok(())
}
