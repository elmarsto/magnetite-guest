# Magnetite Guest Reference Implementation

This is the reference implementation of a Magnetite Guest.

It works in conjunction with the [Magnetite Library](https://github.com/elmarsto/magnetite-lib/) and [Magnetite Host](https://github.com/elmarsto/magnetite-host/) to construe a system of [Rust](https://www.rust-lang.org/) bindings for [Obsidian.md](https://obsidian.md/) plugin development.

Metaphorically, the Host -- the JavaScript part -- is the tape deck. When you're just starting out, leave it as-is.

This repo is a blank cassette. Fork it and lay down something funcy :D

```
cd <vault>/.obsidian/plugins/
git clone https://github.com/elmarsto/magnetite-host <your-plugin-name>
cd <your-plugin-name>
git submodule add -f https://github.com/<your-gh-handle>/<your-fork-of-this-repo>
git submodule update --init --recursive
```

After which you will find this very README.md at, say, `~/my-badass-vault/.obsidian/plugins/my-badass-plugin/magnetite-guest/README.md`!


