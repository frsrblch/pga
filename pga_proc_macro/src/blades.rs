use super::*;
use proc_macro2::{Ident, Span};
use std::fmt::{Display, Formatter};

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct Basis(pub u8);

pub const E0123: Basis = Basis(0b_00001111);

impl ToTokens for Basis {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append(self.ident());
    }
}

impl std::ops::Not for Basis {
    type Output = Self;
    fn not(self) -> Self {
        Basis(self.0 ^ 0b_00001111)
    }
}

impl Basis {
    pub fn iter() -> impl Iterator<Item = Self> + 'static {
        (1..(1 << 4)).map(Self)
    }

    pub fn grade(&self) -> u8 {
        self.get(0) as u8 + self.get(1) as u8 + self.get(2) as u8 + self.get(3) as u8
    }

    pub fn define(self) -> TokenStream {
        quote! {
            #[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
            pub struct #self(f64);

            impl From<f64> for #self {
                fn from(value: f64) -> Self {
                    Self(value)
                }
            }
        }
    }

    pub fn ident(&self) -> Ident {
        Ident::new(&self.to_string(), Span::call_site())
    }

    pub fn field(&self) -> Ident {
        if *self == Basis::default() {
            panic!("f64 should not be used as a field");
        }

        if *self == E0123 {
            panic!("E0123 should not be used as a field");
        }

        Ident::new(&self.to_string().to_lowercase(), Span::call_site())
    }

    pub fn get(&self, i: u8) -> bool {
        if i > 3 {
            panic!("invalid index: {}", i);
        }

        let flag = 1 << i;
        self.0 & flag == flag
    }

    fn flip(&mut self, i: u8) {
        if i > 3 {
            panic!("invalid index: {}", i);
        }

        let flag = 1 << i;
        self.0 ^= flag;
    }
}

impl Display for Basis {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        if *self == Basis::default() {
            return write!(f, "f64");
        }

        write!(f, "E")?;

        for i in 0..4 {
            if self.get(i) {
                let char = (i + '0' as u8) as char;
                write!(f, "{}", char)?;
            }
        }

        Ok(())
    }
}

impl std::ops::Mul for Basis {
    type Output = Product<Self>;

    fn mul(mut self, mut rhs: Self) -> Self::Output {
        if self.get(0) && rhs.get(0) {
            return Product::Zero;
        }

        let mut swaps = 0;

        // Move rhs bits to lhs
        for b in 0..4 {
            if rhs.get(b) {
                swaps += (b + 1..4).filter(|i| self.get(*i)).count();
                swaps += (0..b).filter(|i| rhs.get(*i)).count();

                self.flip(b);
                rhs.flip(b);
            }
        }

        let sign = if swaps % 2 == 0 { Sign::Pos } else { Sign::Neg };
        Product::Value(self, sign)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    pub const S: Basis = Basis(0);
    pub const E0: Basis = Basis(1);
    pub const E1: Basis = Basis(1 << 1);
    pub const E2: Basis = Basis(1 << 2);
    pub const E3: Basis = Basis(1 << 3);
    pub const E01: Basis = Basis(0b_00000011);
    pub const E02: Basis = Basis(0b_00000101);
    pub const E03: Basis = Basis(0b_00001001);
    pub const E12: Basis = Basis(0b_00000110);
    pub const E13: Basis = Basis(0b_00001010);
    pub const E23: Basis = Basis(0b_00001100);
    pub const E012: Basis = Basis(0b_00000111);
    pub const E013: Basis = Basis(0b_00001011);
    pub const E023: Basis = Basis(0b_00001101);
    pub const E123: Basis = Basis(0b_00001110);
    pub const E0123: Basis = Basis(0b_00001111);

    #[test]
    fn iter_all_blades() {
        for b in Basis::iter() {
            println!("{}", b);
        }
        assert_eq!(15, Basis::iter().count());
    }

    #[test]
    fn display() {
        assert_eq!("E023", E023.to_string());
    }

    #[test]
    fn mul_e1_e2() {
        let expected = Product::Value(E12, Sign::Pos);
        assert_eq!(expected, E1 * E2);
    }

    #[test]
    fn mul_e2_e1() {
        let expected = Product::Value(E12, Sign::Neg);
        assert_eq!(expected, E2 * E1);
    }

    #[test]
    fn mul_e2_e3() {
        let expected = Product::Value(E23, Sign::Pos);
        assert_eq!(expected, E2 * E3);
    }

    #[test]
    fn mul_e3_e2() {
        let expected = Product::Value(E23, Sign::Neg);
        assert_eq!(expected, E3 * E2);
    }

    #[test]
    fn mul_e12_e12() {
        let expected = Product::Value(Basis::default(), Sign::Neg);
        assert_eq!(expected, E12 * E12);
    }

    #[test]
    fn mul_e123_e123() {
        let expected = Product::Value(Basis::default(), Sign::Neg);
        assert_eq!(expected, E123 * E123);
    }

    #[test]
    fn mul_e123_e0123() {
        let expected = Product::Value(E0, Sign::Pos);
        assert_eq!(expected, E123 * E0123);
    }

    #[test]
    fn mul_e0123_e123() {
        let expected = Product::Value(E0, Sign::Neg);
        assert_eq!(expected, E0123 * E123);
    }

    #[test]
    fn mul_e123_e023() {
        let expected = Product::Value(Basis(0b_00000011), Sign::Pos);
        assert_eq!(expected, E123 * E023);
    }

    #[test]
    fn mul_e023_e123() {
        let expected = Product::Value(Basis(0b_00000011), Sign::Neg);
        assert_eq!(expected, E023 * E123);
    }

    #[test]
    fn mul_e12_e123() {
        let expected = Product::Value(E3, Sign::Neg);
        assert_eq!(expected, E12 * E123);
    }

    #[test]
    fn mul_e1_lhs() {
        use Sign::*;
        assert_eq!(Product::Value(E01, Neg), E1 * E0);
        assert_eq!(Product::Value(S, Pos), E1 * E1);
        assert_eq!(Product::Value(E12, Pos), E1 * E2);
        assert_eq!(Product::Value(E13, Pos), E1 * E3);
        assert_eq!(Product::Value(E0, Neg), E1 * E01);
        assert_eq!(Product::Value(E012, Neg), E1 * E02);
        assert_eq!(Product::Value(E013, Neg), E1 * E03);
        assert_eq!(Product::Value(E2, Pos), E1 * E12);
        assert_eq!(Product::Value(E3, Pos), E1 * E13);
        assert_eq!(Product::Value(E123, Pos), E1 * E23);
        assert_eq!(Product::Value(E02, Neg), E1 * E012);
        assert_eq!(Product::Value(E03, Neg), E1 * E013);
        assert_eq!(Product::Value(E0123, Neg), E1 * E023);
        assert_eq!(Product::Value(E23, Pos), E1 * E123);
        assert_eq!(Product::Value(E023, Neg), E1 * E0123);
    }

    #[test]
    fn mul_e1_rhs() {
        use Sign::*;
        assert_eq!(Product::Value(E012, Neg), E02 * E1);
        assert_eq!(Product::Value(E01, Pos), E0 * E1);
        assert_eq!(Product::Value(S, Pos), E1 * E1);
        assert_eq!(Product::Value(E12, Neg), E2 * E1);
        assert_eq!(Product::Value(E13, Neg), E3 * E1);
        assert_eq!(Product::Value(E0, Pos), E01 * E1);
        assert_eq!(Product::Value(E013, Neg), E03 * E1);
        assert_eq!(Product::Value(E2, Neg), E12 * E1);
        assert_eq!(Product::Value(E3, Neg), E13 * E1);
        assert_eq!(Product::Value(E123, Pos), E23 * E1);
        assert_eq!(Product::Value(E02, Neg), E012 * E1);
        assert_eq!(Product::Value(E03, Neg), E013 * E1);
        assert_eq!(Product::Value(E0123, Pos), E023 * E1);
        assert_eq!(Product::Value(E23, Pos), E123 * E1);
        assert_eq!(Product::Value(E023, Pos), E0123 * E1);
    }

    #[test]
    fn not_test() {
        assert_eq!(E0123, !S);
        assert_eq!(S, !E0123);
    }
}
