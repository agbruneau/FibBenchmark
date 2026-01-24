#[cfg(test)]
mod tests {
    use crate::chart_generator::generate_index_html;
    use crate::data_parser::{AccuracyPoint, BenchmarkData, ComplexityPoint, GoldenRatioPoint};
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_generate_charts_and_index() {
        let dir = tempdir().unwrap();
        let output_dir = dir.path().to_str().unwrap();

        let _data = BenchmarkData {
            complexity: vec![
                ComplexityPoint {
                    n: 10,
                    iterative_ns: 100,
                    matrix_ns: 200,
                },
                ComplexityPoint {
                    n: 20,
                    iterative_ns: 200,
                    matrix_ns: 250,
                },
            ],
            accuracy: vec![AccuracyPoint {
                n: 10,
                binet: 55.0,
                exact: 55,
                rel_error: 0.0,
                abs_error: 0.0,
            }],
            golden_ratio: vec![GoldenRatioPoint {
                n: 10,
                ratio: 1.618,
                error_from_phi: 0.0001,
            }],
        };

        // We only test the index generation here to avoid Kaleido dependencies in test env
        generate_index_html(output_dir);

        let index_path = dir.path().join("index.html");
        assert!(index_path.exists());

        let content = fs::read_to_string(index_path).unwrap();
        assert!(content.contains("Fibonacci Benchmark Suite Report"));
        assert!(content.contains("complexity_chart.html"));
    }
}
