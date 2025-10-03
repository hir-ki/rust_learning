// 身長のデータ
const HEIGHT: [f64; 100] = [
    148.7, 149.5, 133.7, 157.9, 154.2, 147.8, 154.6, 159.1, 148.2, 153.1, 138.2, 138.7, 143.5,
    153.2, 150.2, 157.3, 145.1, 157.2, 152.3, 148.3, 152.0, 146.0, 151.5, 139.4, 158.8, 147.6,
    144.0, 145.8, 155.4, 155.5, 153.6, 138.5, 147.1, 149.6, 160.9, 148.9, 157.5, 155.1, 138.9,
    153.0, 153.9, 150.9, 144.4, 160.3, 153.4, 163.0, 150.9, 153.3, 146.6, 153.3, 152.3, 153.3,
    142.8, 149.0, 149.4, 156.5, 141.7, 146.2, 151.0, 156.5, 150.8, 141.0, 149.0, 163.2, 144.1,
    147.1, 167.9, 155.3, 142.9, 148.7, 164.8, 154.1, 150.4, 154.2, 161.4, 155.0, 146.8, 154.2,
    152.7, 149.7, 151.5, 154.5, 156.8, 150.3, 143.2, 149.5, 145.6, 140.4, 136.5, 146.9, 158.9,
    144.4, 148.1, 155.5, 152.4, 153.3, 142.3, 155.3, 153.1, 152.3,
];

fn sum_up(data: [f64; 100]) -> f64 {
    let sum = data.iter().sum::<f64>();
    println!("sum : {}", sum);
    sum
}
fn calculate_the_mean(height: [f64; 100]) -> f64 {
    let mean = (sum_up(height)) / 100 as f64;
    println!("mean : {}", mean);
    mean
}
fn calculate_the_standard_deviation(height: [f64; 100]) -> f64 {
    //分散 = (各データの偏差)^2の合計 / データの個数  標準偏差= root(分散)
    let mean = calculate_the_mean(height);
    let vec_data = height
        .iter()
        .map(|i| (i - mean).powf(2.0))
        .collect::<Vec<f64>>();
    let sum = vec_data.iter().sum::<f64>();
    let dispersion = sum / 100 as f64;
    let standard_deviation = dispersion.sqrt();
    standard_deviation
}

fn main() {
    let standard_deviation = calculate_the_standard_deviation(HEIGHT);
    println!("standard_deviation: {}", standard_deviation);
}
