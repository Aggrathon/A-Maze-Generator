
use super::Maze;

pub(super) fn get_lowest_neighbour(arr: &[i32], i: usize, width: usize, default: i32) -> i32 {
    if i >= arr.len() { return default; }
    match [
        arr[i],
        if i >= width { arr[i-width] } else { 0 },
        if i > 0 { arr[i-1] } else { 0 },
        if i < arr.len()-width { arr[i+width] } else { 0 },
        if i < arr.len()-1 { arr[i+1] } else { 0 },
    ].iter().filter(|x| **x > 0).min() {
        Some(x) => { *x },
        None => { default }
    }
}

#[derive(PartialEq, Debug)]
pub(super) enum DiffNeigh {
    None,
    One,
    MultSame,
    MultDiff,
    Error
}

pub(super) fn get_num_diff_neighbours(arr: &[i32], i: usize, width: usize) -> DiffNeigh {
    if i >= arr.len() { return DiffNeigh::Error; }
    let mut dir = [
        if i >= width { arr[i-width] } else { 0 },
        if i > 0 { arr[i-1] } else { 0 },
        if i < arr.len()-width { arr[i+width] } else { 0 },
        if i < arr.len()-1 { arr[i+1] } else { 0 },
    ];
    let mut i = 0;
    for j in 0..dir.len() {
        if dir[j] > 0 {
            if j != i { dir[i] = dir[j]; }
            i += 1;
        }
    }
    if i == 0 { return DiffNeigh::None; }
    if i == 1 { return DiffNeigh::One; }
    if i <= dir.len() { for j in 1..i {
        if dir[j] != dir[j-1] { return DiffNeigh::MultDiff; }
    }};
    return DiffNeigh::MultSame;
} 

pub(super) fn get_num_neighbours(arr: &[i32], i:usize, width:usize) -> i32 {
    let mut num = 0;
    if arr[i-1] > 0 { num += 1; }
    if arr[i+1] > 0 { num += 1; }
    if arr[i-width] > 0 { num += 1; }
    if arr[i+width] > 0 { num += 1; }
    num
}

pub(super) fn dot_init_maze(maze: &mut Maze) {
    let x = maze.width/2;
    let y = maze.height/2;
    let i = maze.counter;
    maze.set(x, y, i);
    maze.counter += 1;
}

pub(super) fn get_neighbours_wrapping(i: usize, w: usize) -> [usize; 4] {
    [
        i.wrapping_add(1),
        i.wrapping_sub(1),
        i.wrapping_add(w),
        i.wrapping_sub(w)
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num() {
        assert_eq!(DiffNeigh::None, get_num_diff_neighbours(&[0,0,0], 1, 3));
        assert_eq!(DiffNeigh::One, get_num_diff_neighbours(&[1,0,0], 1, 3));
        assert_eq!(DiffNeigh::MultSame, get_num_diff_neighbours(&[1,0,1], 1, 3));
        assert_eq!(DiffNeigh::MultDiff, get_num_diff_neighbours(&[1,0,2], 1, 3));
        assert_eq!(DiffNeigh::Error, get_num_diff_neighbours(&[1,0,2], 5, 3));
        assert_eq!(DiffNeigh::MultDiff, get_num_diff_neighbours(&[0,3,0 ,1,0,2, 0,4,0,], 4, 3));
    }

    
    #[test]
    fn test_low() {
        assert_eq!(get_lowest_neighbour(&[0,1,0, 0,0,0, 0,0,0], 4, 3, 99), 1);
        assert_eq!(get_lowest_neighbour(&[0,0,0, 1,0,0, 0,0,0], 4, 3, 99), 1);
        assert_eq!(get_lowest_neighbour(&[0,0,0, 0,0,1, 0,0,0], 4, 3, 99), 1);
        assert_eq!(get_lowest_neighbour(&[0,0,0, 0,0,0, 0,1,0], 4, 3, 99), 1);
        assert_eq!(get_lowest_neighbour(&[0,1,0, 0,0,2, 0,0,0], 4, 3, 99), 1);
        assert_eq!(get_lowest_neighbour(&[0,1,0, 1,0,1, 0,1,0], 4, 3, 99), 1);
    }
}