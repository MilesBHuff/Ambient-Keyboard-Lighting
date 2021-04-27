pub fn rounded_integer_division(dividend: usize, divisor: usize) -> usize {
    return (dividend + (divisor / 2)) / divisor;
}