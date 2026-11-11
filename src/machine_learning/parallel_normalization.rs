use rayon::prelude::*;

pub fn index(show: bool) {
    if show {
        println!("-- Parallel Normalization --");

        let large_dataset: Vec<f64> = (1..=1_000_000).map(|x| x as f64).collect();

        let normalized = parallel_normalize(&large_dataset);
        println!("First 5 normalized: {:?}", &normalized[..5]);
        println!("Last 5 normalized: {:?}", &normalized[999_995..]);
    }
}

fn parallel_normalize(data: &[f64]) -> Vec<f64> {
    let min = data.par_iter().cloned().reduce(|| f64::INFINITY, f64::min);
    let max = data
        .par_iter()
        .cloned()
        .reduce(|| f64::NEG_INFINITY, f64::max);
    let range = max - min;
    data.par_iter().map(|&x| (x - min) / range).collect()
}
