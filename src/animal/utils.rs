use std::collections::HashMap;

use bevy::prelude::*;

use crate::{
    animal::components::{AnimalAnimations, AnimalKind, AnimalLibrary},
    world::components::{AnimalRoam, StaticWorld},
};

pub fn animal_kind_from_static<'a, 'b>(
    static_world: &'a StaticWorld,
    kind: AnimalKind,
) -> Vec<&'a AnimalRoam> {
    let mut animals = Vec::new();

    for roam in &static_world.animals {
        if roam.animal.kind == kind {
            animals.push(roam);
        }
    }

    animals
}
