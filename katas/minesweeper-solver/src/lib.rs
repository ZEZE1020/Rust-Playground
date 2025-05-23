use std::collections::{HashSet, VecDeque};
use std::fmt;
use std::ops::{Add, Sub};

// Helper to get valid neighbour coordinates
fn get_neighbours(
    r: usize,
    c: usize,
    rows: usize,
    cols: usize,
) -> Vec<(usize, usize)> {
    let mut neigbours = Vec::new();
    for dr in -1..=1 {
        for dc in -1..=1 {
            if dr == 0 && dc == 0 {
                continue;
            }
            let new_r = r as isize + dr;
            let new_c = c as isize + dc;
            if new_r >= 0 && new_r < rows as isize && new_c >= 0 && new_c < cols as isize {
                neigbours.push((new_r as usize, new_c as usize));
            }   
        }
    }
    neigbours
}   