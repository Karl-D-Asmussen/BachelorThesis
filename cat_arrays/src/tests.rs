
#![cfg(test)]

use utils::*;

#[test]
fn mixed_radix() {
    assert_eq!(from_mixed_radix([6, 5, 4].iter().rev(), [10, 10, 10].iter().rev()),
               654);
    assert_eq!(to_mixed_radix(1000, [10, 10, 10].iter().rev(), 654),
               vec![6, 5, 4])
}
