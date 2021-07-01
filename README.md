# polka-dots

Test suite for [dotfiles](https://github.com/starptr/dotfiles).

## Development

After modifying dotfiles, run

```shell
cargo run -- deploy
```

To debug containers (i.e. keep containers running), run

```shell
cargo run -- deploy -i
```

To test the test suite itself manually (i.e. when the dotfiles have not been modified), run

```shell
cargo run -- deploy --skip-build
```

Flags can be combined.
