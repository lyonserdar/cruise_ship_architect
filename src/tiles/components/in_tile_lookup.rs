use bevy::prelude::*;

/// Tag component for tiles that are in the TileLookup
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct InTileLookup;
