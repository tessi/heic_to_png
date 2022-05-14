# NIF for Elixir.HeichToPng.Native

## To build the NIF module:

- Your NIF will now build along with your project.

## To load the NIF:

```elixir
defmodule HeichToPng.Native do
    use Rustler, otp_app: :heic_to_png, crate: "heic_to_png"

    # When your NIF is loaded, it will override this function.
    def add(_a, _b), do: :erlang.nif_error(:nif_not_loaded)
end
```
