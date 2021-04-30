macro_rules! rounded_integer_division {
    ($dividend: expr, $divisor: expr) => {
        ($dividend + ($divisor / 2)) / $divisor
    }
}
