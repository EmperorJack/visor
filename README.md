# Visor Core

![Visor visuals screenshot](./assets/readme/visuals-screenshot.png)

Real-time engine for creative coding visuals in JavaScript or TypeScript.

- Designed with the aim to be user-friendly, fast, and extensible.
- Inspired by [Processing](https://processing.org/), [Nannou](https://nannou.cc/), and other creative coding frameworks.
- Powers the new version of the [Visor](https://www.visor.live/) VJing app (unreleased).

## Disclaimer ⚠️

The Visor engine is still very early in development. The APIs are minimal, likely to change, and documentation is sparse. Please keep this in mind if you use it!

If you think of a feature request, notice any bugs, or have any other feedback, please feel free to submit an issue on [Github](https://github.com/EmperorJack/visor-core/issues).

## Examples

The following illustrates a simple Visor sketch:

```js
const draw = createDraw();

export function setup() {
  console.log("Hello, world!");
}

export function update() {
  draw.clear();

  for (let i = -5; i <= 5; i++) {
    const x = i * 50;
    const y = Math.sin(time() + i) * 50;

    const hue = map(x, -250, 250, 0, 1);
    const color = hsv(hue, 0.8, 1);

    draw.ellipse().xy(x, y).wh(25, 25).fill(color);
  }
}
```

And the corresponding visuals:

![Visor sketch example](./assets/readme/sketch-example.png)

Examples that illustrate different API features can be found in the [examples](./examples/) folder.

## Running a Visor sketch

The recommended way to run Visor sketches is with the `visor_cli` command line interface. At this time, it is only available through `cargo`, which means you must have [Rust](https://www.rust-lang.org/) installed. Follow the instructions [here](https://www.rust-lang.org/tools/install) to install Rust.

With Rust installed, you can install `visor_cli` using `cargo`:

```sh
cargo install visor_cli
```

Now, to run a sketch:

```sh
visor_cli run ~/path/to/sketch.js
```

If you clone this repository onto your machine, you can run the examples directly:

```sh
visor_cli run ~/path/to/visor-core/examples/draw-shapes.ts
```

Use the `--help` flag to learn more about the `visor_cli` options or sub-commands:

```sh
visor_cli --help
visor_cli run --help
```

## Live coding

You can use the `--watch` flag with `visor_cli` to detect saved changes to a sketch file and hot reload it. E.g:

```sh
visor_cli run ~/path/to/sketch.js --watch
```

## TypeScript

To best make use of TypeScript in your sketches you can use the the Visor API type declarations. These can be generated with `visor_cli` by running:

```sh
visor_cli types ~/output/path/for/visor-types.d.ts
```

You can then reference these types in your TypeScript code. The recommended development environent for working on Visor sketches is [Deno](https://deno.com). Follow the instructions [here](https://docs.deno.com/runtime/getting_started/installation/) to install Deno.

If you are using Visual Studio Code, you can also use the [Deno extension](https://marketplace.visualstudio.com/items?itemName=denoland.vscode-deno) to make use of the Deno language server.

With Deno installed, you can create a `deno.json` file in your sketch folder to configure the types correctly:

```json
{
  "compilerOptions": {
    "lib": ["deno.ns"],
    "types": ["./visor-types.d.ts"]
  }
}
```

Please note that none of the Deno standard libraries (such as `fetch`) are available with the Visor engine by default. Specifying `lib` in your `deno.json` like above configures your type checker to understand this.

## How it works

The Visor engine is powered by [Deno Core](https://github.com/denoland/deno_core), which underpins the [Deno](https://deno.com/) JavaScript runtime. For Visor, this means the user can sketch in JavaScript or TypeScript, depsite the engine itself being written in Rust. This makes the engine both fast and reliable, while enabling user-friendly creative coding.

Graphics are enabled by using a mix of APIs from the [Nannou](https://nannou.cc/) creative coding framework and the underlying [wgpu](https://wgpu.rs/) graphics API. Windowing is supported by the [TAO](https://github.com/tauri-apps/tao) library.

## Crates

| Crate | Description |
| --- | --- |
| `visor_cli`  | Command line interface for running Visor sketches. Uses the Visor engine. |
| `visor_core` | Re-exports `visor_engine` and all of the core plugins. |
| `visor_engine` | The Visor engine. |
| `visor_plugin_draw` | Plugin for drawing shapes. |
| `visor_plugin_log` | Plugin for console logging. |
| `visor_plugin_math` | Plugin for useful math functions. |
| `visor_plugin_midi` | Plugin for connecting to MIDI devices and loading mappings. |
| `visor_plugin_state` | Plugin for persistent sketch state when hot reloading. |
| `visor_plugin_time` | Plugin for time and frame related functions. |

## Using the Visor engine in a Rust app

The Visor engine can be called from a Rust program. To see how this works, check out the implementation of `visor_cli` in the [source code](./visor_cli/src/run.rs). Note that you must provide your own event loop and windows using [TAO](https://github.com/tauri-apps/tao).

## Plugins

Plugins are the best way to extend the Visor engine with your own APIs. Plugins are written in both Rust and JavaScript or TypeScript. Check out the [counter_plugin](./examples/counter_plugin/) as an example, or look at the core plugins within the [crates](./crates/) folder.

Plugins can be built statically into a Rust app that uses the Visor engine, or they can be linked at runtime after building them as dynamic libraries e.g: `cdylib`. The latter enables custom plugins to be loaded into the `visor_cli` by passing their paths to the `--plugins` option.

## Contributing

This project is still in early stages and is not ready for code contributions yet. However, feedback is very welcome! The best way to contribute is through feature requests, bug reports, and general feedback. Please feel free to submit this as an issue on [Github](https://github.com/EmperorJack/visor-core/issues). Thanks!
