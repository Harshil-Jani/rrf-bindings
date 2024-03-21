use std::collections::HashMap;
use pyo3::prelude::*;

#[pyfunction]
/// Function to perform Reciprocal Rank Fusion
pub fn reciprocal_rank_fusion(lists: Vec<Vec<&str>>) -> PyResult<Vec<(String, f64)>> {
    let mut scores: HashMap<&str, f64> = HashMap::new();
    for list in &lists {
        for (rank, &item) in list.iter().enumerate() {
            let rank_score = 1.0 / ((rank as f64) + 1.0);
            match scores.get(item) {
                Some(&prev_score) => scores.insert(item, prev_score + rank_score),
                None => scores.insert(item, rank_score),
            };
        }
    }

    // Sort items by score in descending order
    let mut sorted_items: Vec<(String, f64)> = scores
        .into_iter()
        .map(|(k, v)| (k.to_string(), v))
        .collect();
    sorted_items.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    Ok(sorted_items)
}

#[pymodule]
fn ranx_module(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(reciprocal_rank_fusion, m)?)?;
    Ok(())
}


