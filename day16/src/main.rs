use std::fs;

//type Result<T> = ::std::result::Result<T, dyn std::error::Error>;

pub fn main() -> std::io::Result<()> {
    let f = fs::read_to_string("input/day16.txt")?.trim().to_string();
    let fft = FFT::new(parse_str(f.clone()));
    let part1 = fft
        .skip(99)
        .next()
        .unwrap()
        .iter()
        .take(8)
        .map(|e| e.to_string())
        .collect::<Vec<String>>()
        .join("");
    println!("{}", part1);
    dbg!(&f.len());
    let part2_input: Vec<i32> = vec![parse_str(f)]
        .iter()
        .cycle()
        .take(10)
        .flat_map(|e| e.clone())
        .collect();

    let msg_offset: i32 = part2_input
        .iter()
        .take(7)
        .map(|e| e.to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse()
        .unwrap();
    let fft = FFT::new(part2_input.clone());
    for (i, e) in fft.take(100).enumerate() {
        println!(
            "{}: {}",
            i,
            e.iter()
                .take(8)
                .map(|d| d.to_string())
                .collect::<Vec<String>>()
                .join("")
        );
    }
    /*
    let part2 = fft
        .skip(99)
        .next()
        .unwrap()
        .iter()
        //.skip(msg_offset as usize)
        .take(8)
        .map(|e| e.to_string())
        .collect::<Vec<String>>()
        .join("");
    */
    //dbg!(part2);

    Ok(())
}

pub fn base_pattern(ith_element: usize, len: usize) -> Vec<i32> {
    let base: Vec<i32> = vec![0, 1, 0, -1];
    let expanded: Vec<i32> = base
        .iter()
        .map(|e| {
            let mut v = Vec::new();
            v.resize(ith_element as usize, e);
            v
        })
        .flat_map(|e| e.clone())
        .cloned()
        .collect();
    expanded.iter().cycle().skip(1).take(len).cloned().collect()
}

fn make_bases(len: usize) -> Vec<Vec<i32>> {
    let mut bases = vec![];
    for i in 0..len {
        bases.push(base_pattern(i + 1, len));
    }
    bases
}

fn parse_str(s: String) -> Vec<i32> {
    s.chars()
        .map(|e| {
            let r: Result<i32, _> = e.to_string().parse();
            match r {
                Ok(n) => n,
                Err(er) => {
                    panic!("broke on '{}', {}", e, er);
                }
            }
        })
        .collect()
}

struct FFT {
    current: Vec<i32>,
    bases: Vec<Vec<i32>>,
}

impl FFT {
    fn new(s: Vec<i32>) -> FFT {
        let len = s.len();
        FFT {
            current: s,
            bases: make_bases(len),
        }
    }
}

impl Iterator for FFT {
    type Item = Vec<i32>;

    fn next(&mut self) -> Option<Vec<i32>> {
        let mut output = Vec::with_capacity(self.current.len());
        for (i, _) in self.current.iter().enumerate() {
            let this_digit = self
                .current
                .iter()
                //.zip(&self.bases[i])
                .zip(base_pattern(i, self.current.len()))
                .map(|(left, right)| left * right)
                .fold(0, |a, b| a + b);
            output.push(this_digit.abs() % 10);
        }
        self.current = output;
        Some(self.current.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tiny() {
        let mut fft = FFT::new(parse_str("12345678".to_string()));
        assert_eq!(fft.next().unwrap(), parse_str("48226158".to_string()));
        assert_eq!(fft.next().unwrap(), parse_str("34040438".to_string()));
    }

    #[test]
    fn base1() {
        let b: Vec<i32> = base_pattern(1, 8);
        assert_eq!(b, vec![1, 0, -1, 0, 1, 0, -1, 0]);
    }

    #[test]
    fn base2() {
        let b: Vec<i32> = base_pattern(2, 8);
        assert_eq!(b, vec![0, 1, 1, 0, 0, -1, -1, 0]);
    }
    #[test]
    fn base3() {
        let b: Vec<i32> = base_pattern(3, 8);
        assert_eq!(b, vec![0, 0, 1, 1, 1, 0, 0, 0]);
    }
}
