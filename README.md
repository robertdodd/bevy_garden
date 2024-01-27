# Bevy Garden

A simple `bevy` game where you plant trees in a garden, featuring a simple editor interface, common patterns
regarding state-management and multiple examples for interacting with bevy scenes.

---

![Example Screenshot Image](./images/screenshot.png)

## Usage

```shell
cargo run
```

## Features

This example showcases the following:

* [Prefab tool](crates/prefab_tool/src) to place objects defined in [asset files](assets/tools) (a scene file and
  config file for each tool).
* [Duplicate tool](crates/duplicate_tool/src) to capture an object (and its hierarchy) and place it around the scene.
* Tool library and tool stack.
* Workspace crate organization ([crates](crates) directory).
* Saving and loading, including undo/redo via ([crates/save](crates/save/src)).
* Shared components, state transitions etc. in [crates/game_state](crates/game_state/src).
* Treat objects made up of multiple entities as a single object. See [crates/bush](crates/bush/src), and see how the
  `FamilyChild` and `ExternalRelations` components are used throughout the code.
* Multiple examples working with `bevy` scenes:
  * Modifying scenes before spawning (centering, adding components etc.)
  * Capturing objects to scenes
  * Dealing with `Parent`/`Children` hierarchies in scenes
  * Spawning scenes directly into the world without using `DynamicSceneBundle`
  * Saving only the types you want
  * See [bevy_scene_utils](crates/bevy_scene_utils/src), [save](crates/save/src),
    [duplicate_tool](crates/duplicate_tool/src) and [prefab_tool](crates/prefab_tool/src/commands.rs).

## Third-Party Bevy Crates

The following third-party crates are used:

- [bevy_asset_loader](https://github.com/NiklasEi/bevy_asset_loader) - Load assets in loading states
- [bevy_common_assets](https://github.com/NiklasEi/bevy_common_assets) - Custom `ron` assets
- [bevy_mod_raycast](https://github.com/aevyrie/bevy_mod_raycast) - Ray casting and picking 

## Extending - Add a New Object and Tool

It's relatively easy to add a new object type. Here's a roughly what to do:

1. Add a crate for this object
   - E.g. duplicate the `crates/tree` crate
   - Use to [Cargo.toml](Cargo.toml)
   - Add plugin in [src/main.rs](src/main.rs)
2. Create tool asset files
   - E.g. duplicate `assets/tools/tree_large`
   - Modify `config.tool.ron` and `prefab.scn.ron` files to reference the new object
   - Add the assets to the [tool asset loader](src/game/tool_loader.rs).
3. `cargo run` and it should work!

## Credits

- [bevy_save](https://github.com/hankjordan/bevy_save) - I copied the idea of the `SaveableRegistry` and rollbacks
  from this project (see [save crate](crates/save/src)).

## License

All code in this repository is dual-licensed under either:

- MIT License (LICENSE-MIT or http://opensource.org/licenses/MIT)
- Apache License, Version 2.0 (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)

at your option. This means you can select the license you prefer.
