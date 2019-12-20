use std::collections::HashSet;
use std::fs;

//type Result<T> = ::std::result::Result<T, dyn std::error::Error>;

pub fn main() -> std::io::Result<()> {
    let f = fs::read_to_string("input/day16.txt")?.trim().to_string();
    dbg!(f.len());
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
        .take(10000)
        .flat_map(|e| e.clone())
        .collect();
    let msg_offset: usize = part2_input
        .iter()
        .take(7)
        .map(|e| e.to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse()
        .unwrap();
    //dbg!(part2_input.len());
    //dbg!(msg_offset);

    //dbg!(part2_input.len() as i32 - msg_offset);
    //let g = calc_tree(&5977359, &650000);
    //dbg!(g.len());

    let sliced_fft = SlicedFFT::new(part2_input.clone(), msg_offset);
    for (i, e) in sliced_fft.take(100).enumerate() {
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

/// Makes a sequence of numbers like this:
///
/// 1: [1, 0, -1, 0, 1, 0, -1, 0]
/// 2: [0, 1, 1, 0, 0, -1, -1, 0]
/// 3: [0, 0, 1, 1, 1, 0, 0, 0]
/// ...
pub fn base_pattern(ith_element: usize) -> impl Iterator<Item = i8> {
    let base: Vec<i8> = vec![0, 1, 0, -1];
    let expanded = base
        .into_iter()
        .map(|e| {
            let mut v = Vec::new();
            v.resize(ith_element as usize, e);
            v
        })
        .flat_map(|e| e.clone())
        .collect::<Vec<i8>>();
    expanded.into_iter().cycle().skip(1)
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
}

impl FFT {
    fn new(s: Vec<i32>) -> FFT {
        let len = s.len();
        FFT { current: s }
    }
}

impl Iterator for FFT {
    type Item = Vec<i32>;

    fn next(&mut self) -> Option<Vec<i32>> {
        let mut output = Vec::with_capacity(self.current.len());
        //println!("{}", self.current.len());
        for (i, _) in self.current.iter().enumerate() {
            let this_digit = self
                .current
                .iter()
                //.zip(&self.bases[i])
                .zip(base_pattern(i + 1))
                .map(|(left, right)| left * right as i32)
                .fold(0, |a, b| a + b);
            output.push(this_digit.abs() % 10);
        }
        self.current = output;
        Some(self.current.clone())
    }
}

struct SlicedFFT {
    current: Vec<i32>,
    offset: usize,
}

impl SlicedFFT {
    fn new(s: Vec<i32>, offset: usize) -> SlicedFFT {
        let actual_input: Vec<i32> = s.into_iter().skip(offset).collect();
        let _input_len = actual_input.len();
        SlicedFFT {
            current: actual_input,
            offset: offset,
            /*
            bases: (0..input_len)
                .map(|i| {
                    if i % 1000 == 0 {
                        println!("{}/{}", i, input_len);
                    }
                    base_pattern(i).skip(offset).take(input_len).collect()
                })
                .collect(),
            */
        }
    }
}

impl Iterator for SlicedFFT {
    type Item = Vec<i32>;

    fn next(&mut self) -> Option<Vec<i32>> {
        println!("go");
        let mut output = Vec::with_capacity(self.current.len());
        //println!("{}", self.current.len());
        for (i, _) in self.current.iter().enumerate() {
            let digit_ix = self.offset + i + 1;
            let mut this_digit = 0;
            if i % 50 == 0 {
                println!("{} / {} ", i, self.current.len());
            }
            for (i_little, (left, right)) in self
                .current
                .iter()
                //.zip(base_pattern(digit_ix).skip(self.offset))
                .zip(base_pattern(digit_ix).skip(self.offset))
                .enumerate()
            {
                this_digit += left * right as i32;
            }
            output.push(this_digit.abs() % 10);
        }
        self.current = output;
        Some(self.current.clone())
    }
}

/// for a given position and total puzzle length, return indexes that are needed from the prior
/// phase to calculate the current element
fn calc_tree(index: &usize, len: &usize) -> HashSet<usize> {
    let mut queue_to_look_up = vec![index.clone()];
    let mut looked_up = HashSet::new();
    let mut o = HashSet::new();
    while queue_to_look_up.len() > 0 {
        let mut skipped = vec![];
        //dbg!(o.len());
        let current_lookup = queue_to_look_up.remove(0);
        //dbg!(current_lookup);
        // wait is it always + 1?
        let items_needed: Vec<usize> = base_pattern(current_lookup + 1)
            .take(*len)
            .enumerate()
            .filter(|(i, e)| *e != 0)
            .map(|(i, e)| i)
            .collect();
        //dbg!(items_needed.len());
        looked_up.insert(current_lookup);
        for i in &items_needed {
            o.insert(i.clone());
            if looked_up.contains(&i) {
                skipped.push(i);
                continue;
            } else {
                queue_to_look_up.push(*i);
            }
        }
        skipped.sort();
        //dbg!(skipped);
    }
    o
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
    fn test_offset_optimization() {
        let big: Vec<i32> = vec![parse_str("12345678".to_string())]
            .iter()
            .cycle()
            .take(300)
            .flat_map(|e| e.clone())
            .collect();
        assert_eq!(big.len(), 2400);
        let slice_offset = 2340;
        let mut fft = FFT::new(big.clone());
        let mut sliced_fft = SlicedFFT::new(big, slice_offset);
        let msg_len = 10;
        assert_eq!(
            fft.next().unwrap()[slice_offset..slice_offset + msg_len],
            sliced_fft.next().unwrap()[..msg_len],
            //parse_str("9625702352".to_string())[..]
        );
        assert_eq!(
            fft.next().unwrap()[slice_offset..slice_offset + msg_len],
            sliced_fft.next().unwrap()[..msg_len],
            //parse_str("8785074125".to_string())[..]
        );
    }

    #[test]
    fn base1() {
        let b: Vec<i32> = base_pattern(1).take(8).collect();
        assert_eq!(b, vec![1, 0, -1, 0, 1, 0, -1, 0]);
    }

    #[test]
    fn base2() {
        let b: Vec<i32> = base_pattern(2).take(8).collect();
        assert_eq!(b, vec![0, 1, 1, 0, 0, -1, -1, 0]);
    }
    #[test]
    fn base3() {
        let b: Vec<i32> = base_pattern(3).take(8).collect();
        assert_eq!(b, vec![0, 0, 1, 1, 1, 0, 0, 0]);
    }
}
