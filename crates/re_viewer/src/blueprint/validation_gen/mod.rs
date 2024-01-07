// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/rust/blueprint_validation.rs
use super::validation::validate_component;
pub use crate::blueprint::components::PanelView;
pub use re_entity_db::blueprint::components::EntityPropertiesComponent;
use re_entity_db::EntityDb;
pub use re_space_view::blueprint::components::QueryExpressions;
pub use re_types::blueprint::components::ActiveTab;
pub use re_types::blueprint::components::ColumnShares;
pub use re_types::blueprint::components::EntitiesDeterminedByUser;
pub use re_types::blueprint::components::IncludedContents;
pub use re_types::blueprint::components::IncludedQueries;
pub use re_types::blueprint::components::Name;
pub use re_types::blueprint::components::RowShares;
pub use re_types::blueprint::components::SpaceViewClass;
pub use re_types::blueprint::components::SpaceViewOrigin;
pub use re_types::blueprint::components::Visible;
pub use re_viewport::blueprint::components::AutoLayout;
pub use re_viewport::blueprint::components::AutoSpaceViews;
pub use re_viewport::blueprint::components::ContainerKind;
pub use re_viewport::blueprint::components::GridColumns;
pub use re_viewport::blueprint::components::IncludedSpaceViews;
pub use re_viewport::blueprint::components::RootContainer;
pub use re_viewport::blueprint::components::SpaceViewMaximized;
pub use re_viewport::blueprint::components::ViewportLayout;

/// Because blueprints are both read and written the schema must match what
/// we expect to find or else we will run into all kinds of problems.

pub fn is_valid_blueprint(blueprint: &EntityDb) -> bool {
    validate_component::<ActiveTab>(blueprint)
        && validate_component::<AutoLayout>(blueprint)
        && validate_component::<AutoSpaceViews>(blueprint)
        && validate_component::<ColumnShares>(blueprint)
        && validate_component::<ContainerKind>(blueprint)
        && validate_component::<EntitiesDeterminedByUser>(blueprint)
        && validate_component::<EntityPropertiesComponent>(blueprint)
        && validate_component::<GridColumns>(blueprint)
        && validate_component::<IncludedContents>(blueprint)
        && validate_component::<IncludedQueries>(blueprint)
        && validate_component::<IncludedSpaceViews>(blueprint)
        && validate_component::<Name>(blueprint)
        && validate_component::<PanelView>(blueprint)
        && validate_component::<QueryExpressions>(blueprint)
        && validate_component::<RootContainer>(blueprint)
        && validate_component::<RowShares>(blueprint)
        && validate_component::<SpaceViewClass>(blueprint)
        && validate_component::<SpaceViewMaximized>(blueprint)
        && validate_component::<SpaceViewOrigin>(blueprint)
        && validate_component::<ViewportLayout>(blueprint)
        && validate_component::<Visible>(blueprint)
}
