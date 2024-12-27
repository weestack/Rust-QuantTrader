#[allow(dead_code)]
#[derive(Debug)]
pub struct Grid {
    pub min: f64,
    pub max: f64,
    pub midpoint: f64,
    pub lines: Vec<f64>
}

impl Grid {
    pub fn new(delta: f64, min: f64, max: f64) -> Grid {
        let mut lines: Vec<f64> = Vec::new();
        lines.push(min);
        let bar = (max - min) / delta;
        let midpoint = min + (max - min) / 2f64;

        for i in 1 .. bar.ceil() as usize {
            lines.push(min + (i as f64) * delta);
        }
        lines.push(max);

        Grid {
            min,
            max,
            midpoint,
            lines,
        }
    }

    pub fn relative_to_midpoint(&self, num: f64) -> i32 {
        /*  1 if greater than
         *  0 if equal
         * -1 if less than
         */
        if num < self.midpoint {
            -1
        } else if num == self.midpoint {
            0
        } else { 1 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialize_grid() {
        let grid = Grid::new(2f64, 10f64, 50f64);
        assert_eq!(grid.min, 10f64);
        assert_eq!(grid.max, 50f64);
        assert_eq!(grid.midpoint, 30f64);
        // (max - min) / 2 + 1
        assert_eq!(grid.lines.len(), 21);
        assert_eq!(grid.lines[5], 20f64);
    }
}