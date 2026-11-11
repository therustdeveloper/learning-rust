pub fn index(show: bool) {
    if show {
        println!("-- Normalization --");

        let raw_prices: Vec<f64> = vec![2.50, 15.99, 4.75, 29.99, 8.00];
        let normalized = normalize(&raw_prices);
        println!("Original prices: {:?}", raw_prices);
        println!("Normalized:      {:?}", normalized);
    }
}

fn normalize(data: &[f64]) -> Vec<f64> {
    let min = data.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let range = max - min;
    data.iter().map(|&x| (x - min) / range).collect()
}
