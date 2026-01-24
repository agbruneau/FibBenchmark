use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct ComplexityPoint {
    pub n: u64,
    pub iterative_ns: u128,
    pub matrix_ns: u128,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccuracyPoint {
    pub n: u64,
    pub exact: u128,
    pub binet: f64,
    pub abs_error: f64,
    pub rel_error: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoldenRatioPoint {
    pub n: u64,
    pub ratio: f64,
    pub error_from_phi: f64,
}

#[derive(Debug)]
pub struct BenchmarkData {
    pub complexity: Vec<ComplexityPoint>,
    pub accuracy: Vec<AccuracyPoint>,
    pub golden_ratio: Vec<GoldenRatioPoint>,
}

impl BenchmarkData {
    pub fn load(results_dir: &str) -> std::io::Result<Self> {
        let dir = Path::new(results_dir);

        let complexity_path = dir.join("complexity_comparison.json");
        let accuracy_path = dir.join("binet_accuracy.json");
        let golden_path = dir.join("golden_ratio_convergence.json");

        let complexity_json = fs::read_to_string(complexity_path)?;
        let complexity: Vec<ComplexityPoint> = serde_json::from_str(&complexity_json)?;

        let accuracy_json = fs::read_to_string(accuracy_path)?;
        let accuracy: Vec<AccuracyPoint> = serde_json::from_str(&accuracy_json)?;

        let golden_json = fs::read_to_string(golden_path)?;
        let golden_ratio: Vec<GoldenRatioPoint> = serde_json::from_str(&golden_json)?;

        Ok(BenchmarkData {
            complexity,
            accuracy,
            golden_ratio,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_benchmark_data_load() {
        let dir = tempdir().unwrap();
        let dir_path = dir.path();

        // Create dummy JSON files
        let complexity_path = dir_path.join("complexity_comparison.json");
        let mut f = File::create(complexity_path).unwrap();
        writeln!(
            f,
            r#"[
            {{ "n": 10, "iterative_ns": 100, "matrix_ns": 200 }}
        ]"#
        )
        .unwrap();

        let accuracy_path = dir_path.join("binet_accuracy.json");
        let mut f = File::create(accuracy_path).unwrap();
        writeln!(
            f,
            r#"[
            {{ "n": 10, "exact": 55, "binet": 55.000001, "abs_error": 0.000001, "rel_error": 0.0 }}
        ]"#
        )
        .unwrap();

        let golden_path = dir_path.join("golden_ratio_convergence.json");
        let mut f = File::create(golden_path).unwrap();
        writeln!(
            f,
            r#"[
            {{ "n": 10, "ratio": 1.618, "error_from_phi": 0.0001 }}
        ]"#
        )
        .unwrap();

        // Test loading
        let data = BenchmarkData::load(dir_path.to_str().unwrap()).unwrap();

        assert_eq!(data.complexity.len(), 1);
        assert_eq!(data.complexity[0].n, 10);

        assert_eq!(data.accuracy.len(), 1);
        assert_eq!(data.accuracy[0].exact, 55);

        assert_eq!(data.golden_ratio.len(), 1);
        assert_eq!(data.golden_ratio[0].ratio, 1.618);
    }

    #[test]
    fn test_benchmark_data_load_missing_files() {
        let dir = tempdir().unwrap();
        let dir_path = dir.path();

        // Don't create any files or create only some
        let result = BenchmarkData::load(dir_path.to_str().unwrap());
        assert!(result.is_err());
    }
}
