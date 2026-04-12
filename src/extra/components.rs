use bevy::prelude::*;

#[derive(Clone, Debug, Default)]
pub enum Range<T> {
    Range(T, T),
    One(T),
    #[default]
    None,
}

#[derive(Clone, Debug, Default)]
pub enum Value<T> {
    Random(T, T),
    Amount(T),
    #[default]
    True,
}

#[derive(Clone, Debug)]
pub struct Noise<T, K> {
    pub octaves: Vec<NoiseLevel>,
    pub value_1: T,
    pub value_2: K,
}

#[derive(Clone, Debug)]
pub struct NoiseLevel {
    pub frequency: f32,
    pub amplitude: f32,
}
