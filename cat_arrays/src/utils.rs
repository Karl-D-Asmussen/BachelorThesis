
pub fn check_index(base : usize, index : isize) -> bool {
    index >= -(base as isize) && (index as usize) < base 
}

pub fn check_slice(base : usize, index : &(isize, Option<isize>, isize)) -> bool {
    check_index(base, index.0) &&
    check_index(base + 1, index.1.unwrap_or(base as isize))
}

pub fn slice_steps(base : usize, index : &(isize, Option<isize>, isize)) -> Option<usize> {
    let (lo, hi, stride) =
    if index.2 < 0 {
        (compute_index(base, index.1.unwrap_or(base as isize)), compute_index(base, index.0), -index.2 as usize)
    } else if index.2 > 0 {
        (compute_index(base, index.0), compute_index(base, index.1.unwrap_or(base as isize)), index.2 as usize)
    } else { return None };
    
    Some((hi - lo) / stride)
}

pub fn compute_index(base : usize, index : isize) -> usize {
    if index < 0 {
        (index + (base as isize)) as usize
    } else {
        index as usize
    }
}
