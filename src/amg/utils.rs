

pub(super) fn get_lowest_neighbour(arr: &[i32], i: usize, width: usize, default: i32) -> i32 {
    if i > arr.len() { return default; }
    let mut v = default;
    if arr[i] > 0 { v = arr[i]; }
    if i >= width {
        if i > 0 && arr[i-1] > 0 && arr[i-1] < v { v = arr[i-1]; }
        if arr[i-width] > 0 && arr[i-width] < v { v = arr[i-width]; }
    }
    if i < arr.len() - width {
        if i < arr.len()-1 && arr[i+1] > 0 && arr[i+1] < v { v = arr[i+1]; }
        if arr[i+width] > 0 && arr[i+width] < v { v = arr[i+width]; }
    }
    v
}

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
    if i < dir.len() { for j in 1..i {
        if dir[j] != dir[j-1] { return DiffNeigh::MultDiff; }
    }};
    return DiffNeigh::MultSame;
} 
