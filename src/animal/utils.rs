use std::collections::HashMap;

use bevy::prelude::*;

use crate::{
    animal::components::{AnimalAnimations, AnimalKind, AnimalLibrary},
    world::components::{AnimalRoam, StaticWorld},
};

pub fn animal_kind_from_static<'a, 'b>(
    static_world: &'a StaticWorld,
    library: &'b AnimalLibrary,
    kind: AnimalKind,
) -> Vec<(&'a AnimalRoam, &'b HashMap<usize, AnimalAnimations>)> {
    let mut animals = Vec::new();

    for roam in &static_world.animals {
        if let Some(animation) = library.animals.get(&roam.animal.kind)
            && roam.animal.kind == kind
        {
            animals.push((roam, animation));
        }
    }

    animals
}
