use crate::{
    error::Error,
    facelet::{Facelet, FaceletTarget},
    moves::{Alg, Inverse, Move},
};
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
#[allow(clippy::len_without_is_empty)]
pub struct Commutator {
    pub setup: Option<Alg>,
    pub interchange: Move,
    pub insertion: Alg,
    pub insertion_first: bool,
}

impl Commutator {
    /// Returns `true` if the commutator has no setup moves.
    pub fn is_pure(&self) -> bool {
        self.setup.is_none()
    }

    /// Returns the length of the commutator in its notation form.
    pub fn len(&self) -> usize {
        self.setup.as_ref().map_or(0, |s| s.len()) + self.insertion.len() + 1
    }

    /// Returns the non-reduced expanded algorithm.
    pub fn expand(&self) -> Alg {
        let interchange = Alg::new([self.interchange]);
        let (first, second) = if self.insertion_first {
            (&self.insertion, &interchange)
        } else {
            (&interchange, &self.insertion)
        };
        let middle = first + second + first.inverse() + second.inverse();

        match &self.setup {
            Some(setup) => setup + &middle + setup.inverse(),
            _ => middle,
        }
    }
}

impl fmt::Display for Commutator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let insertion = self.insertion.to_string();
        let interchange = self.interchange.to_string();
        let (first, second) = if self.insertion_first {
            (insertion, interchange)
        } else {
            (interchange, insertion)
        };
        let start = self
            .setup
            .as_ref()
            .map(|s| format!("[{}: ", s))
            .unwrap_or_default();
        let end = if self.setup.is_some() { "]" } else { "" };

        write!(f, "{start}[{first}, {second}]{end}")
    }
}

/// Wrapper around 3-cycle targets (stickers).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cycle<T> {
    targets: [T; 3],
}

impl<T> Cycle<T>
where
    T: Clone + Copy,
{
    pub fn new(first: T, second: T, third: T) -> Self {
        Self {
            targets: [first, second, third],
        }
    }

    pub fn first(&self) -> T {
        self.targets[0]
    }

    pub fn second(&self) -> T {
        self.targets[1]
    }

    pub fn third(&self) -> T {
        self.targets[2]
    }
}

impl<T> Cycle<T>
where
    T: Clone + Copy + FaceletTarget,
{
    pub fn to_facelets(&self) -> [Facelet; 3] {
        self.targets.map(|t| t.as_facelet())
    }
}

impl<T> Inverse for Cycle<T>
where
    T: Clone + Copy,
{
    fn inverse(&self) -> Self {
        Self {
            targets: [self.first(), self.third(), self.second()],
        }
    }
}

impl<T> fmt::Display for Cycle<T>
where
    T: Clone + Copy + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} - {} - {}", self.first(), self.second(), self.third())
    }
}

pub trait ThreeCycle: Sized {
    fn cycle<T>(self, cycle: Cycle<T>) -> Result<Self, Error>
    where
        T: Clone + Copy + FaceletTarget + fmt::Display;
}

#[cfg(test)]
mod tests {
    use super::Commutator;
    use crate::{alg, moves::Move};
    use std::str::FromStr;

    #[test]
    fn test_commutator_string() {
        let commutator = Commutator {
            setup: None,
            interchange: Move::from_str("U").unwrap(),
            insertion: alg!("R' D' R"),
            insertion_first: false,
        };
        let expected = "[U, R' D' R]";

        assert_eq!(expected, commutator.to_string());

        let commutator = Commutator {
            setup: Some(alg!("U")),
            interchange: Move::from_str("U").unwrap(),
            insertion: alg!("R' D' R"),
            insertion_first: false,
        };
        let expected = "[U: [U, R' D' R]]";

        assert_eq!(expected, commutator.to_string());
    }

    #[test]
    fn test_commutator_expand() {
        let commutator = Commutator {
            setup: Some(alg!("D")),
            interchange: Move::from_str("U").unwrap(),
            insertion: alg!("R' D' R"),
            insertion_first: true,
        };
        let expected = alg!("D R' D' R U R' D R U' D'");

        assert_eq!(expected, commutator.expand().reduce());
    }
}
