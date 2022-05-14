# HeicToPng

Converts HEIC files to PNG. Supports alpha channels and HDR files.

This is currently a proof of concept. If I decide to use it, I'd add tests, CI, linters and all the fun! Consider this library unstable.

## Installation

If [available in Hex](https://hex.pm/docs/publish), the package can be installed
by adding `heic_to_png` to your list of dependencies in `mix.exs`:

```elixir
def deps do
  [
    {:heic_to_png, "~> 0.1.0"}
  ]
end
```

make sure `libheif` is installed in your system, e.g. by:

```sh
brew install libheif
```

For unknown reasons to me, I needed to point `ld` to the library path of libheif manually by doing:

```sh
export LIBRARY_PATH=$LIBRARY_PATH:/opt/homebrew/Cellar/libheif/1.12.0_1/lib
```

## Usage

Note: converting big heic files is slow in dev/test environments. If you want to get a feel for realistic performance, run in MIX_ENV `prod` or `bench` as this will compile the Rust NIF with optimizations enabled.

E.g.: `MIX_ENV=bench iex -S mix`

```elixir
{:ok, heic_image} = File.read("test/test_files/RGBA-8bit.heic")
{:ok, png_image} = HeicToPng.Native.heic_to_png(heic_image)
File.write("out.png", png_image)
```

The docs can be found at [https://hexdocs.pm/heic_to_png](https://hexdocs.pm/heic_to_png).

## License

This project is MIT licensed, see LICENSE.md.

Files under test/test_files are taken from https://github.com/aduial/libheif-test