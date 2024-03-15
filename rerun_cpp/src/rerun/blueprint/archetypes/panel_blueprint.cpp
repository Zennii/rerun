// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/re_types/definitions/rerun/blueprint/archetypes/panel_blueprint.fbs".

#include "panel_blueprint.hpp"

#include "../../collection_adapter_builtins.hpp"

namespace rerun::blueprint::archetypes {}

namespace rerun {

    Result<std::vector<DataCell>> AsComponents<blueprint::archetypes::PanelBlueprint>::serialize(
        const blueprint::archetypes::PanelBlueprint& archetype
    ) {
        using namespace blueprint::archetypes;
        std::vector<DataCell> cells;
        cells.reserve(2);

        if (archetype.expanded.has_value()) {
            auto result = DataCell::from_loggable(archetype.expanded.value());
            RR_RETURN_NOT_OK(result.error);
            cells.push_back(std::move(result.value));
        }
        {
            auto indicator = PanelBlueprint::IndicatorComponent();
            auto result = DataCell::from_loggable(indicator);
            RR_RETURN_NOT_OK(result.error);
            cells.emplace_back(std::move(result.value));
        }

        return cells;
    }
} // namespace rerun