defmodule MyMathTest do
  use ExUnit.Case
  doctest MyMath

  test "add" do
    assert MyMath.add(1, 2) == 3
  end

  test "subtract" do
    assert MyMath.subtract(1, 2) == -1
  end

  test "multiply" do
    assert MyMath.multiply(3, 2) == 6
  end

  test "adder" do
    adder = MyMath.create_adder(1)
    assert MyMath.adder_add(adder, 2) == 3
    assert MyMath.adder_add(adder, 6) == 7
  end

  test "counter" do
    counter = MyMath.create_counter()
    assert MyMath.increment_counter(counter, 1) == 1
    assert MyMath.increment_counter(counter, 5) == 6
  end 
end
