fn nth(digit: i32, n: u32) -> i32 {
    (digit % 10_i32.pow(n)) / 10_i32.pow(n-1)
}

fn sum(square: &[[i32; 300]], startx: usize, starty: usize, size: usize) -> i32
{
    let mut sum = 0;
    for x in 0..size {
        let x_vals = &square[startx + x];
        for y in 0..size {
            sum += &x_vals[starty + y];
        }
    }
    sum
}

fn main() {
    let serial: i32 = 8141;
    let mut grid: [[i32; 300]; 300] = [[0; 300]; 300];

    for x in 0..300 {
        for y in 0..300 {
            let rack_id = x + 10;
            let power_level = (rack_id * y + serial) * rack_id;

            grid[x as usize][y as usize] = nth(power_level, 3) - 5;
        }
    }

    let mut largest_power: Option<(usize, usize, usize, i32)> = None;
    for size in 1..300 {
        println!("Checking size: {}", size);
        for x in 0..(300-size) {
            for y in 0..(300-size) {
                let s = sum(&grid, x, y, size);
                
                match largest_power {
                    None => {
                        largest_power = Some((x, y, size, s));
                    }
                    
                    Some((_, _, _, _)) => {
                        if largest_power.unwrap().3 < s {
                            largest_power = Some((x, y, size, s));
                        }
                    }
                }
            }
        }
    }

    println!("Largest: {:?}", largest_power.unwrap());
}
