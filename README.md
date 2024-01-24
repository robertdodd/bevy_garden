# Bevy Garden

An simple `bevy` game where you plant trees in a garden, featuring a simple editor interface, common patterns
regarding state-management and multiple examples for interacting with bevy scenes.

---

![Example Screenshot Image](./images/screenshot.png)

## Usage

```shell
cargo run
```

## Features

This example showcases the following:

* [Prefab tool](crates/prefab_tool/src) to place objects via [asset files](assets/tools).
* [Pointer tool](crates/pointer_tool/src), a tool that can't be implemented via an asset file.
* Tool library and tool stack.
* Workspace crate organization ([crates](crates) directory).
* Saving and loading, including undo/redo via ([crates/save](crates/save/src)).
* Shared components, state transitions etc. in [crates/game_state](crates/game_state/src).
* Multiple ways of spawning bevy scenes, including modifying scenes before spawning, spawning without using `DynamicSceneBundle`, and more. See [bevy_scene_utils](crates/bevy_scene_utils/src) and [prefab_tool](crates/prefab_tool/src/commands.rs).
* Treat objects made up of multiple entities as a single object. See [crates/bush](crates/bush/src), and see how the `FamilyChild` and `ExternalRelations` components are used throughout the code.

## Third-Party Bevy Crates

The following third-party crates are used:

- [bevy_asset_loader](https://github.com/NiklasEi/bevy_asset_loader) - Load assets in loading states
- [bevy_common_assets](https://github.com/NiklasEi/bevy_common_assets) - Custom `ron` assets
- [bevy_mod_raycast](https://github.com/aevyrie/bevy_mod_raycast) - Ray casting and picking 

## Extending - Add a New Object and Tool

It's relatively easy to add a new object type. Here's a roughly what to do:

1. Add a crate for this object
   - E.g. duplicate the `crates/tree` crate
   - Add to [Cargo.toml](Cargo.toml)
   - Add plugin in [src/main.rs](src/main.rs)
2. Create tool asset files
   - E.g. duplicate `assets/tools/tree_large`
   - Modify `config.tool.ron` and `prefab.scn.ron` files to reference the new object
   - Add tool to asset loader in [src/game/tool_loader.rs](src/game/tool_loader.rs).
3. `cargo run` and it should work!

## Credits

- [bevy_save](https://github.com/hankjordan/bevy_save) - I copied the idea of the `SaveableRegistry` used in
  `crates/save` from this project.

## License

All code in this repository is dual-licensed under either:

- MIT License (LICENSE-MIT or http://opensource.org/licenses/MIT)
- Apache License, Version 2.0 (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)

at your option. This means you can select the license you prefer.
