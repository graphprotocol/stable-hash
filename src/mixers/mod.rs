pub mod fld;
pub mod mul;

pub trait Mixer {
    const IDENTITY: Self;
    fn mix(&mut self, value: u128);
    fn combine(self, other: Self) -> Self;
    fn raw(&self) -> u128;
}
