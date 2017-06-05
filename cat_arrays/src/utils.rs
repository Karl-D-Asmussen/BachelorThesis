
pub fn check_index(base: usize, index: isize) -> bool {
    index >= -(base as isize) && (index as usize) < base
}

pub fn check_slice(base: usize, index: &(isize, Option<isize>, isize)) -> bool {
    check_index(base, index.0) && check_index(base + 1, index.1.unwrap_or(base as isize)) &&
    index.2 != 0
}

pub fn slice_steps(base: usize, index: &(isize, Option<isize>, isize)) -> usize {
    let (lo, hi, stride) = if index.2 < 0 {
        (compute_index(base, index.1.unwrap_or(base as isize)),
         compute_index(base, index.0),
         -index.2 as usize)
    } else if index.2 > 0 {
        (compute_index(base, index.0),
         compute_index(base, index.1.unwrap_or(base as isize)),
         index.2 as usize)
    } else {
        return ::std::usize::MAX;
    };

    (hi - lo) / stride
}

pub fn compute_index(base: usize, index: isize) -> usize {
    if index < 0 {
        (index + (base as isize)) as usize
    } else {
        index as usize
    }
}

pub fn from_mixed_radix<'a, I, J>(it: I, jt: J) -> usize
    where I: Iterator<Item = &'a isize>,
          J: Iterator<Item = &'a usize>
{
    let mut scale = 1;
    let mut val = 0;
    for (i, j) in it.zip(jt) {
        val += compute_index(*j, *i) * scale;
        scale *= *j;
    }
    val
}

pub fn to_mixed_radix<'a, I>(mut total: usize, it: I, mut val: usize) -> Vec<isize>
    where I: Iterator<Item = &'a usize>
{
    let mut digits = Vec::new();
    for i in it {
        total /= *i;
        digits.push((val / total) as isize);
        val %= total;
    }
    digits
}
