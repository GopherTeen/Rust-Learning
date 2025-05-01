pub trait Gcd {
    fn gcd(self, other: Self) -> Self;
}

impl Gcd for i32 {
    fn gcd(self, other: Self) -> Self {
        let (mut a, mut b) = (self, other);
        while b != 0 {
            (a, b) = (b, a - b * (a / b));
        }
        a.abs()
    }
}
