pub mod wheel {
  #[derive(Debug, Clone, Copy)]
  pub enum Wheel<T> {
    NaN,
    Inf,
    NegInf,
    Num(T),
  }
  use Wheel::*;

  impl<T> Wheel<T> {
    pub fn sign(&self) -> Option<Ordering> where T: Neg<Output = T> + Clone + PartialOrd {
      self.clone().partial_cmp(&self.clone().neg())
    }

    pub fn num(self) -> Option<T> {
      match self {
        NaN | Inf | NegInf => None,
        Num(x) => Some(x),
      }
    }
  }

  impl<'a, T> Into<Option<&'a T>> for &'a Wheel<T> {
    fn into(self) -> Option<&'a T> {
      match self {
        NaN | Inf | NegInf => None,
        Num(x) => Some(x),
      }
    }
  }

  impl<T: Display> Display for Wheel<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
      match self {
        NaN => write!(f, "NaN"),
        Inf => write!(f, "Inf"),
        NegInf => write!(f, "-Inf"),
        Num(x) => x.fmt(f),
      }
    }
  }

  impl<T: PartialEq> PartialEq for Wheel<T> {
    fn eq(&self, other: &Self) -> bool {
      match (self, other) {
        (Inf, Inf) | (NegInf, NegInf) => true,
        (Num(x), Num(y)) => x.eq(y),
        _ => false,
      }
    }
  }

  impl<T: PartialEq + Eq> Eq for Wheel<T> {}

  impl<T: PartialOrd> PartialOrd for Wheel<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
      match (self, other) {
        (NaN, _) | (_, NaN) => None,
        (Inf, Inf) | (NegInf, NegInf) => Some(Equal),
        (Inf, _) | (_, NegInf) => Some(Greater),
        (_, Inf) | (NegInf, _) => Some(Less),
        (Num(x), Num(y)) => x.partial_cmp(y),
      }
    }
  }

  impl<T: PartialOrd + Ord> Ord for Wheel<T> {
    /// Panics if either self or other is NaN
    fn cmp(&self, other: &Self) -> Ordering {
      self.partial_cmp(other).unwrap()
    }
  }

  impl<T: Neg<Output = T>> Neg for Wheel<T> {
    type Output = Self;
    fn neg(self) -> Self {
      match self {
        NaN => NaN,
        Inf => NegInf,
        NegInf => Inf,
        Num(x) => Num(-x),
      }
    }
  }

  impl<T: Add<Output = T>> Add for Wheel<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
      match (self, other) {
        (NaN, _) | (_, NaN) | (Inf, NegInf) | (NegInf, Inf) => NaN,
        (Inf, _) | (_, Inf) => Inf,
        (NegInf, _) | (_, NegInf) => NegInf,
        (Num(x), Num(y)) => Num(x + y),
      }
    }
  }

  impl<T: Sub<Output = T>> Sub for Wheel<T> {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
      match (self, other) {
        (NaN, _) | (_, NaN) | (Inf, Inf) | (NegInf, NegInf) => NaN,
        (Inf, _) | (_, NegInf) => Inf,
        (NegInf, _) | (_, Inf) => NegInf,
        (Num(x), Num(y)) => Num(x - y),
      }
    }
  }

  impl<T: Clone + Neg<Output = T> + PartialOrd + Mul<Output = T>> Mul for Wheel<T> {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
      match (self, other) {
        (NaN, _) | (_, NaN) => NaN,
        (x @ Inf, y) | (x @ NegInf, y) => {
          match y.sign() {
            Some(Greater) => x,
            Some(Less) => -x,
            _ => NaN,
          }
        },
        (Num(x), Num(y)) => Num(x * y),
        (x, y) => y * x,
      }
    }
  }

  impl<T: Clone + Neg<Output = T> + Sub<Output = T> + PartialOrd + Div<Output = T>> Div for Wheel<T> {
    type Output = Self;
    fn div(self, other: Self) -> Self {
      match (self, other) {
        (NaN, _) | (_, NaN) | (Inf, Inf) | (Inf, NegInf) | (NegInf, Inf) | (NegInf, NegInf) => NaN,
        (x @ Num(_), Inf) | (x @ Num(_), NegInf) => x.clone() - x,
        (x @ Inf, y @ Num(_)) | (x @ NegInf, y @ Num(_)) => match y.sign() {
            Some(Less) => -x,
            _ => x,
        },
        (Num(x), Num(y)) => Num(x / y),
      }
    }
  }

  impl<T: Rem<Output = T> + Neg<Output = T>> Rem for Wheel<T> {
    type Output = Self;
    fn rem(self, other: Self) -> Self {
      match (self, other) {
        (NaN, _) | (_, NaN) | (Inf, _) | (NegInf, _) => NaN,
        (x, Inf) => x,
        (x, NegInf) => -x,
        (Num(x), Num(y)) => Num(x % y),
      }
    }
  }

  use std::fmt::{self, Debug, Display, Formatter};
  use std::cmp::{*, Ordering::*};
  use std::ops::*;
}
