pub fn rounded_integer_division(
    dividend: usize,
    divisor:  usize,
) -> usize {
    (dividend + (divisor / 2)) / divisor
}
