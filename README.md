# game-of-the-amazons

An implementation of the [Game of the Amazons](https://en.wikipedia.org/wiki/Game_of_the_Amazons) written in Rust and Godot. Includes some rudimentary AI.

## `core`
This is where most of the Rust code lives, including the implementation of the game and the AI.

The main entrypoint to this crate is a CLI interface to the AI. The AI prints its moves to stdout and reads the opponents moves to stdin, one move per line. The moves are formatted in a modified chess notation: `a1-j10/j5` denotes moving an amazon from a1 to j10 and firing an arrow to j5.

The AI will assume it is playing white by default. If `--black` is specified when launching the CLI, it will assume black instead.

## UI
You will need Godot installed to build the UI.

The UI will default to two human players taking turns on the same computer. If `--black` or `--white` are specified when launching the UI, it will use the subsequent arguments to launch a CLI AI. For example, `amazons.x86_64 --black ../amazons_core --black --white ../amazons_core` will play two AIs against eachother (note that the second `--black` is an argument to the AI, not GUI).

Any compatible executable can be used with the UI.
