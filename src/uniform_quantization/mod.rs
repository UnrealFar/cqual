#![allow(dead_code)]

use std::{borrow::Borrow, collections::HashMap};

// TODO: Current implementation divides regions into linear blocks.
// Logarithmic division of regions would be nice.
/*
pub enum RegionDivide {
    Linear,
    Logarithmic,
}
 */
// Pass this thing into Qunatizer::new(r: u8, g: u8, b: u8, devide: RegionDivide)
// TODO: Support alpha channel

#[derive(Debug)]
// Region defines the different blocks in which rgb pixels are devided in
struct Regions {
    // Number of blocks
    r: u8,
    g: u8,
    b: u8,
    // Pixel maps to those blocks
    r_map: HashMap<u8, Vec<u8>>,
    g_map: HashMap<u8, Vec<u8>>,
    b_map: HashMap<u8, Vec<u8>>,
}

enum Color {
    R,
    G,
    B,
}

impl Regions {
    pub(crate) fn new(r: u8, g: u8, b: u8) -> Self {
        Self {
            r,
            g,
            b,
            r_map: HashMap::with_capacity(r as usize),
            g_map: HashMap::with_capacity(g as usize),
            b_map: HashMap::with_capacity(b as usize),
        }
    }

    // We insert the rgb values into maps independent of each other
    fn insert(&mut self, r: u8, g: u8, b: u8) {
        self.r_map
            .entry(self.index(r, &Color::R))
            .and_modify(|x| x.push(r))
            .or_insert(Vec::from([r]));
        self.g_map
            .entry(self.index(g, &Color::G))
            .and_modify(|x| x.push(g))
            .or_insert(Vec::from([g]));
        self.b_map
            .entry(self.index(b, &Color::B))
            .and_modify(|x| x.push(b))
            .or_insert(Vec::from([b]));
    }

    // Reset the pixel map
    fn reset(&mut self) {
        self.r_map = HashMap::with_capacity(self.r as usize);
        self.g_map = HashMap::with_capacity(self.g as usize);
        self.b_map = HashMap::with_capacity(self.b as usize);
    }

    // Calculate index of region based on the color value
    fn index(&self, value: u8, color: &Color) -> u8 {
        match color {
            Color::R => (value as usize * self.r as usize / 256) as u8,
            Color::G => (value as usize * self.g as usize / 256) as u8,
            Color::B => (value as usize * self.b as usize / 256) as u8,
        }
    }
}

#[derive(Default, Debug)]
// Struct that stores averae for each region
pub(crate) struct RegionAverage {
    r: HashMap<u8, u8>,
    g: HashMap<u8, u8>,
    b: HashMap<u8, u8>,
}

#[derive(Debug)]
pub struct Quantizer {
    regions: Regions,
    pub(crate) average: RegionAverage,
}

impl Quantizer {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self {
            regions: Regions::new(r, g, b),
            average: RegionAverage::default(),
        }
    }

    fn calc_avg(&mut self) {
        for (index, values) in self.regions.r_map.borrow() {
            let avg = usize::from(values.iter().map(|x| *x as usize).sum::<usize>()) / values.len();
            self.average.r.insert(*index, avg as u8);
        }
        for (index, values) in self.regions.g_map.borrow() {
            let avg = usize::from(values.iter().map(|x| *x as usize).sum::<usize>()) / values.len();
            self.average.g.insert(*index, avg as u8);
        }
        for (index, values) in self.regions.b_map.borrow() {
            let avg = usize::from(values.iter().map(|x| *x as usize).sum::<usize>()) / values.len();
            self.average.b.insert(*index, avg as u8);
        }
    }
}

impl crate::Quantize for Quantizer {
    fn quantize(&mut self, pixels: &[(u8, u8, u8, u8)]) {
        for (r, g, b, _) in pixels {
            self.regions.insert(*r, *g, *b);
        }
        self.calc_avg();
    }

    fn reset(&mut self) {
        self.regions.reset();
    }

    fn get_pallet(&self) {}

    fn get_quantized(&self, r: u8, g: u8, b: u8, a: u8) -> (u8, u8, u8, u8) {
        let (ri, gi, bi) = (
            self.regions.index(r, &Color::R),
            self.regions.index(g, &Color::G),
            self.regions.index(b, &Color::B),
        );
        (
            *self.average.r.get(&ri).unwrap_or(&0),
            *self.average.g.get(&gi).unwrap_or(&0),
            *self.average.b.get(&bi).unwrap_or(&0),
            a,
        )
    }
}
