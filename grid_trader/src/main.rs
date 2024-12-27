mod grid;
mod coin;

use grid::Grid;

#[tokio::main]
async fn main() {
    let grid = Grid::new(2f64, 10f64, 80f64);
    println!("{}", grid.lines.len());
    println!("{:?}", grid.relative_to_midpoint(14.9999))
}

