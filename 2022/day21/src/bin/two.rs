use std::io::BufRead;

use aoc::calculator::Calculator;
fn between_exclusive(v: i64, min: i64, max: i64) -> bool {
    min.min(max) < v && max.max(min) > v
}
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::io::BufReader::new(std::io::stdin()).lines().filter_map(std::io::Result::ok).collect();
    let calculator = Calculator::from_strings(input);
    {
        let mut now = 0;
        let mut step = 1;
        while calculator.calculate(&"root".to_string(), Some(now)) == calculator.calculate(&"root".to_string(), Some(now + step)) {
            now -= step;
            step *= 3;
        }
        if calculator.calculate(&"root".to_string(), Some(now)).abs() < calculator.calculate(&"root".to_string(), Some(now + step)).abs() {
            now += step;
            step *= -1;
        }
        while calculator.calculate(&"root".to_string(), Some(now)) != 0 {
            let now_val = calculator.calculate(&"root".to_string(), Some(now));
            let next_val = calculator.calculate(&"root".to_string(), Some(now + step));
            if between_exclusive(0, now_val, next_val) {
                step = (step.abs() / 2).max(1) * (step / step.abs());
            } else {
                now += step;
                step *= 2;
            }
        }
        while calculator.calculate(&"root".to_string(), Some(now - 1)) == 0 { now -= 1; }
        println!("{}", now);
    }
    Ok(())
}
