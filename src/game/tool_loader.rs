use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use editor::prelude::*;
use game_state::prelude::*;
use prefab_tool::prelude::*;

/// Plugin which loads all prefab tool assets and adds them to the tool library
pub struct ToolLoaderPlugin;

impl Plugin for ToolLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.configure_loading_state(
            LoadingStateConfig::new(PlayState::LoadAssets).load_collection::<PrefabToolAssets>(),
        )
        .add_systems(
            Update,
            handle_tool_assets.run_if(resource_added::<PrefabToolAssets>()),
        );
    }
}

#[derive(AssetCollection, Resource)]
pub(crate) struct PrefabToolAssets {
    #[asset(
        paths(
            "tools/tree_large/config.tool.ron",
            "tools/tree_small/config.tool.ron",
            "tools/apple/config.tool.ron",
            "tools/flower/config.tool.ron",
            "tools/bush/config.tool.ron",
        ),
        collection(typed)
    )]
    pub tools: Vec<Handle<PrefabToolAsset>>,
}

/// Add prefab tool assets to the tool library
fn handle_tool_assets(
    assets: Res<AssetServer>,
    prefab_tools: Res<PrefabToolAssets>,
    tool_assets: Res<Assets<PrefabToolAsset>>,
    mut tool_library: ResMut<ToolLibrary>,
) {
    for handle in prefab_tools.tools.iter() {
        if let Some(tool_def) = tool_assets.get(handle) {
            // Define the path to the scene asset
            let scene_path = handle
                .path()
                .unwrap()
                .path()
                .parent()
                .unwrap()
                .join("prefab.scn.ron");

            // Register the tool in the tool library
            tool_library.register_tool(ToolInfo {
                key: tool_def.key.clone(),
                tool: Tool::Prefab(PrefabConfig {
                    name: tool_def.name.clone(),
                    scene: assets.load(scene_path),
                    tool_type: match &tool_def.tool_type {
                        PrefabToolAssetType::Attachable(config) => {
                            PrefabToolType::Attachable(PrefabAttachableConfig {
                                distance: config.distance,
                                forward: config.forward,
                            })
                        }
                        PrefabToolAssetType::Object => PrefabToolType::Object,
                    },
                    initial_scale: tool_def.initial_scale,
                    scaling: tool_def.scaling.as_ref().map(|scaling| ToolScaling {
                        min: scaling.min,
                        max: scaling.max,
                    }),
                }),
                name: tool_def.name.clone(),
            });
        }
    }
}
