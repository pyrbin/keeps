mod coord;
mod field;
mod grid;

pub use self::coord::*;
pub use self::field::*;
pub use self::grid::*;
use crate::prelude::*;

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set_to_stage(
            CoreStage::PostUpdate,
            ConditionSet::new()
                .with_system(coord_propagate_system)
                .with_system(maintain_grid_cache_system)
                .into(),
        );
    }
}
