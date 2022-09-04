use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AppState {
    AssetLoading,
    WorldGen,
    InGame,
}

#[derive(SystemLabel, Debug, Clone, Hash, Eq, PartialEq)]
pub enum SystemLabels {
    Input,
}
