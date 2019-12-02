use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

//type Result<T> = ::std::result::Result<T, dyn std::error::Error>;

pub fn fuel_cost(n: f64) -> f64 {
    (n / 3.0).trunc() - 2.0
}

pub fn true_fuel_cost(n: f64) -> f64 {
    let cost = fuel_cost(n);
    if cost <= 0.0 {
        return 0.0;
    }
    cost + true_fuel_cost(cost)
}

fn main() -> std::io::Result<()> {
    let f = File::open("input/day01.txt")?;
    let reader = BufReader::new(f);
    let mut total = 0.0;
    let mut true_total = 0.0;
    for line in reader.lines() {
        let l = line?;
        let n: f64 = l.parse()?;
        total += fuel_cost(n);
        true_total += true_fuel_cost(n);
    }
    print!("{}\n", total);
    print!("{}\n", true_total);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2.0, fuel_cost(13.0));
        assert_eq!(654.0, fuel_cost(1969.0));
        assert_eq!(33583.0, fuel_cost(100756.0));
    }

    #[test]
    fn test_true_fuel() {
        /*
                 *
            A module of mass 14 requires 2 fuel. This fuel requires no further fuel (2 divided by 3 and rounded down is 0, which would call for a negative fuel), so the total fuel required is still just 2.
            At first, a module of mass 1969 requires 654 fuel. Then, this fuel requires 216 more fuel (654 / 3 - 2). 216 then requires 70 more fuel, which requires 21 fuel, which requires 5 fuel, which requires no further fuel. So, the total fuel required for a module of mass 1969 is 654 + 216 + 70 + 21 + 5 = 966.
            The fuel required by a module of mass 100756 and its fuel is: 33583 + 11192 + 3728 + 1240 + 411 + 135 + 43 + 12 + 2 = 50346.
        */
        assert_eq!(2_f64, true_fuel_cost(14_f64));
        assert_eq!(966_f64, true_fuel_cost(1969_f64));
        assert_eq!(50346_f64, true_fuel_cost(100756_f64));
    }
}
