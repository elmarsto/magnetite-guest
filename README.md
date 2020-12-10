# Magnetite Guest Reference Implementation (i.e., fork this repo!)

This is the reference implementation of a Magnetite Guest.

It works in conjunction with the [Magnetite Library](https://github.com/elmarsto/magnetite-lib/) and [Magnetite Host](https://github.com/elmarsto/magnetite-host/) to construe a system of [Rust](https://www.rust-lang.org/) bindings for [Obsidian.md](https://obsidian.md/) plugin development.

The idea being that the host does the "JavaScript part" and the Guest -- i.e., your fork of this very repo -- is the "Rust part".

Metaphorically, the Host, the JavaScript part, is the tape deck. You can leave it mostly as-is, unless you're releasing a new Obsidian plugin for public consumption. Otherwise, concentrate your attention here.

This repo is a blank cassette. Fork it and lay down something funcy :D

For example, if your plugin name was `foo`, you might want to:

```
VAULT=~/my-badass-vault/
PLUGINS=$VAULT/.obsidian/plugins/
cd $PLUGINS
git clone https://github.com/elmarsto/magnetite-host foo
cd foo
git submodule update --init --recursive
```

After which you will find this very README.md at `~/my-badass-vault/.obsidian/plugins/foo/magnetite-guest/README.md`


