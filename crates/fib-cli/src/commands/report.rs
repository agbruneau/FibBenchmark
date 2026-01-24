use fib_viz::generate_report;

pub fn run(input_dir: &str, output_dir: &str) {
    println!("📊 Initializing report generation...");

    match generate_report(input_dir, output_dir) {
        Ok(_) => println!("✨ Report generation completed successfully!"),
        Err(e) => {
            eprintln!("❌ Error generating report: {}", e);
            eprintln!(
                "   Make sure the input directory '{}' contains the required JSON files.",
                input_dir
            );
        }
    }
}
