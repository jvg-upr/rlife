use std::ops::{Deref, DerefMut};

// minimum behavior needed to be able to implement life
pub trait Life {
    /// set state of cell
    fn set_cell(&mut self, x: usize, y: usize, is_alive: bool);

    /// width of the map
    fn width(&self) -> usize;

    /// height of the map
    fn height(&self) -> usize;

    /// state of cell
    fn is_alive(&self, x: usize, y: usize) -> bool;

    /// calculate the number of live neighbors of cell
    fn number_of_neighbors(&self, x: usize, y: usize) -> usize {
        let range = |v: usize| (v.saturating_sub(1)..=v.saturating_add(1));
        // cartesian product of iterators
        range(x)
            .map(|rx| range(y).map(move |ry| (rx, ry)))
            .flatten()
            // count neightbors, excluding itself
            .filter(|&(a, b)| !(a == x && b == y) && self.is_alive(a, b))
            .count()
    }

    /// calculate the next generation of the map
    fn next_generation(&self, other: &mut impl Life) {
        // cartesian product of iterators
        (0..Life::width(self))
            .map(|x| (0..Life::height(self)).map(move |y| (x, y)))
            .flatten()
            // rules of life
            .for_each(|(x, y)| match self.number_of_neighbors(x, y) {
                3 => other.set_cell(x, y, true),                // rule for life
                2 => other.set_cell(x, y, self.is_alive(x, y)), // rule for stagnation
                _ => other.set_cell(x, y, false),               // rule for death
            })
    }
}

// wrapper type to implement Life on [[bool; W]; H]
pub struct LifeGrid<const W: usize, const H: usize> {
    data: [[bool; W]; H],
}

impl<const W: usize, const H: usize> Default for LifeGrid<W, H> {
    fn default() -> Self {
        Self {
            data: [[false; W]; H],
        }
    }
}

// implement Deref and DerefMut so this new type can be used in the same contexts as [[bool; W]; H]
impl<const W: usize, const H: usize> Deref for LifeGrid<W, H> {
    type Target = [[bool; W]; H];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<const W: usize, const H: usize> DerefMut for LifeGrid<W, H> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

// implement Life for LifeGrid
impl<const W: usize, const H: usize> Life for LifeGrid<W, H> {
    fn set_cell(&mut self, x: usize, y: usize, is_alive: bool) {
        if let Some(item) = self.get_mut(y).and_then(|row| row.get_mut(x)) {
            *item = is_alive;
        }
    }

    fn is_alive(&self, x: usize, y: usize) -> bool {
        self.get(y)
            .and_then(|row| row.get(x))
            .copied()
            .unwrap_or(false)
    }

    fn width(&self) -> usize {
        W
    }

    fn height(&self) -> usize {
        H
    }
}
