/// day 8, image encoding
use std::fs;

//type Result<T> = ::std::result::Result<T, dyn std::error::Error>;

pub fn main() -> std::io::Result<()> {
    let f = fs::read_to_string("input/day08.txt")?;
    let input_state = f.trim();
    dbg!(input_state.len());
    let mut layers = vec![];
    let step = (25 * 6);
    let count_of_layers = input_state.len() / step;
    for i in 0..(count_of_layers) {
        let layer = &input_state[(i * step)..((i + 1) * step)];
        layers.push(layer);
    }
    // check we have all the pixels
    let mut pixel_count = 0;
    for layer in &layers {
        pixel_count += layer.chars().count();
    }
    dbg!(pixel_count);
    let target_layer = layers
        .iter()
        .min_by(|a, b| {
            let a_zeros = a.chars().filter(|c| *c == '0').count();
            let b_zeros = b.chars().filter(|c| *c == '0').count();
            a_zeros.cmp(&b_zeros)
        })
        .unwrap();
    let ones = target_layer.chars().filter(|c| *c == '1').count();
    let twos = target_layer.chars().filter(|c| *c == '2').count();
    let part1 = ones * twos;
    dbg!(part1);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
