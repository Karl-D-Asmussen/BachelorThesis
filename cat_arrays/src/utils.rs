
fn check_index(base : usize, index : isize) -> bool {
    index >= -(base as isize) && (index as usize) < base 
}

fn compute_index(base : usize, index : isize) -> usize {
    if index < 0 {
        (index + (base as isize)) as usize
    } else {
        index as usize
    }
}
