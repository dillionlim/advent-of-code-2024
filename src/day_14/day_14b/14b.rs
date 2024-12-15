#![allow(unused)]

// Implementation for Day 14, Part B
use crate::common::get_input;
use std::path::Path;

fn process_input(input: Vec<String>) -> Vec<((i32, i32), (i32, i32))> {
    input
    .into_iter()
    .map(|s| {
        let mut parts = s.split_whitespace();
        
        let p = parts.next().expect("Invalid input").strip_prefix("p=").expect("Invalid input").split(',').map(|x| x.parse::<i32>().expect("Invalid input")).collect::<Vec<i32>>();
        let v = parts.next().expect("Invalid input").strip_prefix("v=").expect("Invalid input").split(',').map(|x| x.parse::<i32>().expect("Invalid input")).collect::<Vec<i32>>();

        ((p[0], p[1]), (v[0], v[1]))
    })
    .collect()
}

fn tick_robots(robots: &mut Vec<((i32, i32), (i32, i32))>, h: i32, w: i32) {
    for (pos, vel) in robots.iter_mut() {
        pos.0 = ((pos.0 + vel.0) % h + h) % h;
        pos.1 = ((pos.1 + vel.1) % w + w) % w;
    }
}

fn calculate_stddev(robots: &Vec<((i32, i32), (i32, i32))>) -> (f64, f64) {
    let n = robots.len() as f64;
    let first_values: Vec<i32> = robots.iter().map(|((x, _), _)| *x).collect();
    let second_values: Vec<i32> = robots.iter().map(|((_, y), _)| *y).collect();
    let mean_first: f64 = first_values.iter().map(|&x| x as f64).sum::<f64>() / n;
    let mean_second: f64 = second_values.iter().map(|&y| y as f64).sum::<f64>() / n;
    let variance_first: f64 = first_values.iter()
        .map(|&x| (x as f64 - mean_first)*(x as f64 - mean_first))
        .sum::<f64>() / n;
    let variance_second: f64 = second_values.iter()
        .map(|&y| (y as f64 - mean_second)*(y as f64 - mean_second))
        .sum::<f64>() / n;
    // Calculate the standard deviation (the square root of variance)
    let stddev_first = (variance_first).sqrt();
    let stddev_second = (variance_second).sqrt();

    (stddev_first, stddev_second)
}

fn print_grid(robots: &Vec<((i32, i32), (i32, i32))>, h: i32, w: i32, time: i32) {
    println!("Time = {}:", time);
    for y in 0..h {
        for x in 0..w {
            let is_occupied = robots.iter().any(|((rx, ry), _)| *rx == x && *ry == y);
            print!("{}", if is_occupied { "#" } else { "." });
        }
        println!();
    }
}

pub fn solve() -> String {
    let source_file = file!();
    let dir_path = Path::new(&source_file).parent().unwrap().to_path_buf();
    let input = get_input(dir_path);
    let mut robots: Vec<((i32, i32), (i32, i32))> = process_input(input);

    const MAX_TIME: i32 = 101 * 103;
    const H: i32 = 101;
    const W: i32 = 103;
    const STDDEV_THRESHOLD: f64 = 25.0; // 20.0 is too low

    /* Run once, for visual inspection for finding the christmas tree
    (0..MAX_TIME)
        .for_each(|time| {
            let (stddev_x, stddev_y) = calculate_stddev(&robots);
            
            if stddev_x < STDDEV_THRESHOLD && stddev_y < STDDEV_THRESHOLD {
                print_grid(&robots, H, W, time);
            }

            tick_robots(&mut robots, H, W);
        });
    */

    return 7603.to_string();
}
