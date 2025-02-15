defmodule MyMath do
  use Rustler, otp_app: :my_math, crate: :math_nif

  def add(_a, _b), do: :erlang.nif_error(:nif_not_loaded)
  def subtract(_a, _b), do: :erlang.nif_error(:nif_not_loaded)
  def multiply(_a, _b), do: :erlang.nif_error(:nif_not_loaded)

  def create_adder(_x), do: :erlang.nif_error(:nif_not_loaded)
  def adder_add(_adder, _x), do: :erlang.nif_error(:nif_not_loaded)

  def create_counter(), do: :erlang.nif_error(:nif_not_loaded)
  def increment_counter(_counter, _x), do: :erlang.nif_error(:nif_not_loaded)
end
