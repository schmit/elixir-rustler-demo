# ElixirRustlerDemo

ElixirRustlerDemo is a little playground to understand how to get started using [Rustler](https://github.com/rustler/rustler) with Elixir.

In particular, we explore how to:

- Write a simple Rustler NIF function and call it in Elixir
- Use a Rustler Resource that can hold state with an Adder
- Use a Rustler Resource that we can mutate with a Counter

Files:

- The rust code is defined in native.math_nif/src/lib.rs
- The Elixir code is defined in lib/my_math.ex
- More importantly, tests in `test/my_math_test.exs` demonstrates these functions in action

## Installation

If [available in Hex](https://hex.pm/docs/publish), the package can be installed
by adding `elixir_rustler_demo` to your list of dependencies in `mix.exs`:

```elixir
def deps do
  [
    {:elixir_rustler_demo, "~> 0.1.0"}
  ]
end
```

Documentation can be generated with [ExDoc](https://github.com/elixir-lang/ex_doc)
and published on [HexDocs](https://hexdocs.pm). Once published, the docs can
be found at <https://hexdocs.pm/elixir_rustler_demo>.

