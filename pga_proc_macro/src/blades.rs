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

    fn set(&mut self, i: u8, value: bool) {
        if i > 3 {
            panic!("invalid index: {}", i);
        }

        let flag = 1 << i;
        if value {
            self.0 |= flag;
        } else {
            self.0 &= !flag;
        }
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

        let mut flips = 0;

        for b in 1..4 {
            if self.get(b) && rhs.get(b) {
                let lhs_count = (b + 1..4).filter(|i| self.get(*i)).count();
                let rhs_count = (0..b).filter(|i| rhs.get(*i)).count();
                flips += lhs_count;
                flips += rhs_count;
                self.set(b, false);
                rhs.set(b, false);
            }
        }

        for b in 0..4 {
            if self.get(b) {
                let lhs_count = (b + 1..4).filter(|i| self.get(*i)).count();
                let rhs_count = (0..b + 1).filter(|i| rhs.get(*i)).count();
                flips += lhs_count;
                flips += rhs_count;
            }
        }

        let basis = Basis(self.0 | rhs.0);
        let sign = if flips % 2 == 0 { Sign::Pos } else { Sign::Neg };
        Product::Value(basis, sign)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    pub const E0: Basis = Basis(1);
    pub const E1: Basis = Basis(1 << 1);
    pub const E2: Basis = Basis(1 << 2);
    pub const E3: Basis = Basis(1 << 3);
    pub const E12: Basis = Basis(0b_00000110);
    pub const E23: Basis = Basis(0b_00001100);
    pub const E023: Basis = Basis(0b_00001101);
    pub const E123: Basis = Basis(0b_00001110);

    #[test]
    fn iter_all_blades() {
        for b in Basis::iter() {
            println!("{}", b);
        }
        assert_eq!(15, Basis::iter().count());
    }

    #[test]
    fn set_true() {
        let mut basis = E1;
        basis.set(0, true);
        assert_eq!(Basis(0b_00000011), basis);
    }

    #[test]
    fn set_false() {
        let mut basis = Basis(0b_00000011);
        basis.set(0, false);
        assert_eq!(E1, basis);
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
}
