use crate::{
    error::Error,
    moves::core::{Inverse, Move, MoveKind},
};
use std::{collections::BTreeMap, fmt, ops::Add, str::FromStr};

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
        let mut groups: Vec<BTreeMap<MoveKind, Move>> = vec![];

        for m in self.0 {
            if let Some(last) = groups.last_mut() {
                let prev_value = last.remove(&m.kind);
                let has_prev_value = prev_value.is_some();
                let result = match prev_value {
                    Some(n) => n * m,
                    None => last.contains_key(&m.kind.inverse()).then_some(m),
                };

                if let Some(value) = result {
                    last.insert(m.kind, value);
                }
                if last.is_empty() {
                    groups.pop();
                }
                if result.is_some() || has_prev_value {
                    continue;
                }
            }

            groups.push([(m.kind, m)].into());
        }

        Alg::new(groups.into_iter().flat_map(|g| g.into_values()))
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
