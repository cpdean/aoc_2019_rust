use std::fs;

//type Result<T> = ::std::result::Result<T, dyn std::error::Error>;

pub fn main() -> std::io::Result<()> {
    let f = fs::read_to_string("input/day16.txt")?;
    let input_state: Vec<i32> = f
        .trim()
        .chars()
        .map(|c| c.to_string().parse().unwrap())
        .collect();
    dbg!(input_state);

    Ok(())
}

pub fn base_pattern(ith_element: i32) -> Vec<i32> {
    let base = vec![0, 1, 0, -1];
    let expanded = base
        .iter()
        .map(|e| {
            let mut v = Vec::new();
            v.resize(ith_element as usize, e);
            v
        })
        .flat_map(|e| e.clone());
    expanded.cycle().skip(1).take(1000).cloned().collect()
}

struct FFT {
    current: String,
}

impl FFT {
    fn new(s: String) -> FFT {
        FFT { current: s }
    }
}

impl Iterator for FFT {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        let parsed: Vec<i32> = self
            .current
            .chars()
            .map(|e| e.to_string().parse().unwrap())
            .collect();
        let mut output = vec![];
        for (i, e) in parsed.iter().enumerate() {
            let this_digit = parsed
                .iter()
                .zip(base_pattern(i as i32 + 1))
                .map(|(left, right)| left * right)
                .fold(0, |a, b| a + b);
            output.push(this_digit.abs() % 10);
        }
        self.current = output
            .iter()
            .map(|d| d.to_string())
            .collect::<Vec<String>>()
            .join("");
        Some(self.current.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tiny() {
        let mut fft = FFT::new("12345678".to_string());
        assert_eq!(fft.next().unwrap(), "48226158");
        assert_eq!(fft.next().unwrap(), "34040438");
    }

    #[test]
    fn base1() {
        let b: Vec<i32> = base_pattern(1).iter().take(8).cloned().collect();
        assert_eq!(b, vec![1, 0, -1, 0, 1, 0, -1, 0]);
    }

    #[test]
    fn base2() {
        let b: Vec<i32> = base_pattern(2).iter().take(8).map(|e| e.clone()).collect();
        assert_eq!(b, vec![0, 1, 1, 0, 0, -1, -1, 0]);
    }
    #[test]
    fn base3() {
        let b: Vec<i32> = base_pattern(3).iter().take(8).map(|e| e.clone()).collect();
        assert_eq!(b, vec![0, 0, 1, 1, 1, 0, 0, 0]);
    }
}
