defmodule HeicToPng do
  @moduledoc """
  TBD: Documentation for `HeicToPng`.
  """

  @doc """
  TBD
  """
  def heic_to_png(image) do
    HeicToPng.Native.heic_to_png(image)
  end
end
