/// day 8, image encoding
use std::fs;

//type Result<T> = ::std::result::Result<T, dyn std::error::Error>;

pub fn main() -> std::io::Result<()> {
    let f = fs::read_to_string("input/day08.txt")?;
    let input_state = f.trim();
    dbg!(input_state.len());
    let layers = image_to_layers(input_state, 25, 6);
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
    let data = "0222112222120000";
    let layers_ex = image_to_layers(data, 2, 2);
    dbg!(layers_to_image(layers_ex.clone()));
    render(&layers_to_image(layers_ex), 2, 2);
    render(&layers_to_image(layers), 25, 6);
    Ok(())
}

pub fn render(image: &str, x: usize, y: usize) {
    let contrast: String = image
        .chars()
        // increase contrast so i can see the message
        .map(|e| if e == '0' { '.' } else { '#' })
        .collect();
    for row in 0..y {
        println!("{}", &contrast[x * row..x * (row + 1)]);
    }
}

pub fn layers_to_image(layers: Vec<&str>) -> String {
    layers
        .iter()
        .map(|e| e.chars())
        .fold(layers[0].to_string(), |acc, e| {
            let v: String = acc
                .chars()
                .zip(e)
                .map(|(a, b)| match (a, b) {
                    ('0', _) => '0',
                    ('1', _) => '1',
                    ('2', x) => x,
                    (x, _) => panic!("got pixel val {}", x),
                })
                .collect();
            v
        })
}

pub fn image_to_layers(data: &str, x_width: usize, y_width: usize) -> Vec<&str> {
    dbg!(data.len());
    let mut layers: Vec<&str> = vec![];
    let step = x_width * y_width;
    let count_of_layers = data.len() / step;
    for i in 0..(count_of_layers) {
        let layer = &data[(i * step)..((i + 1) * step)];
        layers.push(layer);
    }
    layers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let data = "0222112222120000";
        let layers = image_to_layers(data, 2, 2);
    }
}
