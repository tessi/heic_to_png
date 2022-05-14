defmodule HeicToPng.Native do
  use Rustler, otp_app: :heic_to_png, crate: "heic_to_png"

  # When your NIF is loaded, it will override these functions.
  def heic_to_png(heic_image) when is_binary(heic_image), do: :erlang.nif_error(:nif_not_loaded)
end
