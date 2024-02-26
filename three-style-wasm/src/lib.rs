use std::str::FromStr;
use three_style_lib::{
    commutator::{
        finder::{find_corner_commutators, find_edge_commutators},
        types::{Commutator as InnerCommutator, Cycle},
    },
    moves::MoveKind,
    sticker::{Corner, Edge},
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Commutator {
    inner: InnerCommutator,
}

impl From<InnerCommutator> for Commutator {
    fn from(value: InnerCommutator) -> Self {
        Self { inner: value }
    }
}

#[wasm_bindgen]
impl Commutator {
    pub fn format(&self) -> String {
        self.inner.to_string()
    }

    pub fn expand(&self) -> String {
        self.inner.expand().to_string()
    }

    pub fn length(&self) -> usize {
        self.inner.expand().len()
    }
}

#[wasm_bindgen(js_name = "searchCornerCommutators")]
pub fn search_corner_commutators(
    corners: Vec<String>,
    gen: String,
    depth: u8,
) -> Result<Vec<Commutator>, JsError> {
    let corners = corners
        .into_iter()
        .map(|c| Corner::from_str(&c))
        .collect::<Result<Vec<_>, _>>()?;
    let cycle = Cycle::new(corners[0], corners[1], corners[2]);
    let allowed_moves = gen
        .chars()
        .map(|c| MoveKind::from_str(&c.to_string()))
        .collect::<Result<Vec<_>, _>>()?;
    let commutators = find_corner_commutators(cycle, &allowed_moves, depth);
    let results = commutators.into_iter().map(Commutator::from).collect();

    Ok(results)
}

#[wasm_bindgen(js_name = "searchEdgeCommutators")]
pub fn search_edge_commutators(
    edges: Vec<String>,
    gen: String,
    depth: u8,
) -> Result<Vec<Commutator>, JsError> {
    let edges = edges
        .into_iter()
        .map(|c| Edge::from_str(&c))
        .collect::<Result<Vec<_>, _>>()?;
    let cycle = Cycle::new(edges[0], edges[1], edges[2]);
    let allowed_moves = gen
        .chars()
        .map(|c| MoveKind::from_str(&c.to_string()))
        .collect::<Result<Vec<_>, _>>()?;
    let commutators = find_edge_commutators(cycle, &allowed_moves, depth);
    let results = commutators.into_iter().map(Commutator::from).collect();

    Ok(results)
}
