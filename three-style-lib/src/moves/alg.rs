use super::{core::Inverse, Move};
use crate::error::Error;
use std::{fmt, ops::Add, str::FromStr};

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Alg(Vec<Move>);

impl Alg {
    pub fn new<T>(moves: T) -> Self
    where
        T: IntoIterator<Item = Move>,
    {
        Self(moves.into_iter().collect())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn iter(&self) -> impl Iterator<Item = &Move> {
        self.0.iter()
    }

    pub fn clean(self) -> Self {
        let mut groups: Vec<Vec<Option<Move>>> = vec![];

        for m in self.0 {
            if let Some(last_group) = groups.last_mut() {
                let aligned_move = last_group
                    .iter()
                    .flatten()
                    .last()
                    .filter(|l| l.kind == m.kind || m.kind == l.kind.inverse());

                if aligned_move.is_some() {
                    let existing_move = last_group.iter_mut().find_map(|n| {
                        n.and_then(|l| if l.kind == m.kind { Some((n, l)) } else { None })
                    });

                    if let Some((move_ref, move_val)) = existing_move {
                        *move_ref = move_val * m;
                    } else {
                        last_group.push(Some(m));
                    }

                    continue;
                }
            }

            groups.push(vec![Some(m)]);
        }

        Alg::new(groups.into_iter().flatten().flatten())
    }
}

impl Inverse for Alg {
    fn inverse(&self) -> Self {
        Self(self.0.iter().rev().map(Move::inverse).collect())
    }
}

impl IntoIterator for Alg {
    type Item = Move;
    type IntoIter = std::vec::IntoIter<Move>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl FromStr for Alg {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let moves = s
            .split_whitespace()
            .map(Move::from_str)
            .collect::<Result<Vec<_>, _>>();

        moves.map(Self::new)
    }
}

impl Add<Self> for Alg {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0.into_iter().chain(rhs).collect())
    }
}

impl Add<&Alg> for &Alg {
    type Output = Alg;

    fn add(self, rhs: &Alg) -> Self::Output {
        self.clone() + rhs.clone()
    }
}

impl Add<&Alg> for Alg {
    type Output = Self;

    fn add(self, rhs: &Alg) -> Self::Output {
        self + rhs.clone()
    }
}

impl fmt::Display for Alg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = self
            .0
            .iter()
            .map(|m| m.to_string())
            .collect::<Vec<_>>()
            .join(" ");

        write!(f, "{s}")
    }
}

#[macro_export]
macro_rules! alg {
    ($moves: expr) => {{
        use std::str::FromStr;
        use $crate::moves::Alg;
        Alg::from_str($moves).unwrap()
    }};
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::moves::{MoveCount, MoveKind};

    #[test]
    fn test_basics() {
        let alg = alg!("R U R' U'");
        let inverse = alg.inverse();

        assert_eq!(
            alg,
            Alg(vec![
                Move::new(MoveKind::R, MoveCount::Simple),
                Move::new(MoveKind::U, MoveCount::Simple),
                Move::new(MoveKind::R, MoveCount::Prime),
                Move::new(MoveKind::U, MoveCount::Prime)
            ])
        );

        assert_eq!(
            inverse,
            Alg(vec![
                Move::new(MoveKind::U, MoveCount::Simple),
                Move::new(MoveKind::R, MoveCount::Simple),
                Move::new(MoveKind::U, MoveCount::Prime),
                Move::new(MoveKind::R, MoveCount::Prime)
            ])
        );
    }

    #[test]
    fn test_move_cancelation() {
        let alg = alg!("U D2 D U'").clean();
        let expected = alg!("D'");
        assert_eq!(expected, alg);

        let alg = alg!("R U R' U D' U2").clean();
        let expected = alg!("R U R' U' D''");
        assert_eq!(expected, alg);

        let alg = alg!("U2 U2 D D' R").clean();
        let expected = alg!("R");
        assert_eq!(expected, alg);
    }
}
