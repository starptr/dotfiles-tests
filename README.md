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

## Gotchas

When using Docker and GitHub Actions, watch out:

- `COPY` doesn't `chown` recursively. Make sure to set the target directory's ownership or otherwise it will be created under root.
- The host file for `COPY` is a relative path.
- The environment variables in `Dockerfile` are not passed from the shell. Use `--build-args` and `ARG` to explicitly pass them.
- The `-i` option in `docker exec -it` doesn't work, probably because logs aren't interactive
- Do not use a `container`. `docker` doesn't work inside a container because it is inside one.
