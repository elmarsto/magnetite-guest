# Magnetite Guest (Fork this repo)

This is an example guest for Magnetite, which in turn are rust bindings for Obsidian.md plugin development. It works in conjunction with the [Magnetite Host](https://github.com/elmarsto/magnetite-host/)

The idea is the magnetite host can be cloned directly into your Obsidian vault's plugins directory without any changes other than symlinking, copying, mounting, `git submodule`ing, or otherwise manifesting their
guest (i.e. their Rust lib, i.e. presumably a fork of this repo, or equivalent) at `<magnetite-host-directory>/magnetite-guest`.

So, for example, if your plugin name was `foo`, you might want to move this directory into position such that its path becomes:

`~/my-badass-vault/.obsidian/plugins/foo/magnetite-guest`

i.e., this very README.md should be

`~/my-badass-vault/.obsidian/plugins/foo/magnetite-guest/README.md`

If you're releasing something, you can use git submodules to permanently associate your guest's repository with a forked/modded copy of the host.

If you're just developing something a symlink (or even just a copy) tucked into a well-placed clone is often all you need to test out your idea.