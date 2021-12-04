defmodule Day3 do
  def read_input(filename) do
    File.open!(filename)
    |> IO.read(:all)
    |> String.split("\n")
    |> Enum.map(&Day3.parse_line/1)
  end

  def parse_line(line) do
    line
    |> String.codepoints()
    |> Enum.map(&String.to_integer/1)
  end

  def task1(input) do
    string_array = find_most_frequent(input)
    bitstring = Enum.join(string_array, "")

    inverse = invert_bitstring(bitstring)

    {gamma_rate, ""} = Integer.parse(bitstring, 2)
    {epsilon_rate, ""} = Integer.parse(inverse, 2)

    gamma_rate * epsilon_rate
  end

  def find_most_frequent(input) do
    half_length = length(input) / 2

    input
      |> Enum.zip()
      |> Enum.map(&Tuple.sum/1)
      |> Enum.map(&map_most_frequent(&1, half_length))
  end

  def find_least_frequent(input) do
    half_length = length(input) / 2

    input
      |> Enum.zip()
      |> Enum.map(&Tuple.sum/1)
      |> Enum.map(&map_least_frequent(&1, half_length))
  end

  def map_most_frequent(x, half_length) do
    if x >= half_length do
      "1"
    else
      "0"
    end
  end

  def map_least_frequent(x, half_length) do
    if x < half_length do
      "1"
    else
      "0"
    end
  end

  def invert_bitstring(bitstring) do
    bitstring
    |> String.codepoints()
    |> Enum.map(fn c ->
      if c == "0" do
        "1"
      else
        "0"
      end
    end)
    |> Enum.join("")
  end

  def task2(input) do
    oxygen_generator_rating_string = find_oxygen_generator_rating(input)
    co2_scrubber_rating_string = find_co2_scrubber_rating(input)

    {oxygen_generator_rating, ""} = Integer.parse(oxygen_generator_rating_string, 2)
    {co2_scrubber_rating, ""} = Integer.parse(co2_scrubber_rating_string, 2)

    oxygen_generator_rating * co2_scrubber_rating
  end

  def find_oxygen_generator_rating(input) do
    most_frequent = find_most_frequent(input)
    strings = Enum.map(input, fn row -> Enum.join(row, "") end)

    find_oxygen_generator_rating(strings, most_frequent, 0)
  end

  def find_oxygen_generator_rating([rating], _, _) do
    rating
  end

  def find_oxygen_generator_rating(input, most_frequent, current_index) do
    most_frequent_bit = Enum.at(most_frequent, current_index)
    filtered = Enum.filter(input, fn row -> String.at(row, current_index) == most_frequent_bit end)
    most_frequent = find_most_frequent_string(filtered)

    find_oxygen_generator_rating(filtered, most_frequent, current_index + 1)
  end

  def find_most_frequent_string(strings) do
    parsed = Enum.map(strings, &Day3.parse_line/1)
    find_most_frequent(parsed)
  end

  def find_co2_scrubber_rating(input) do
    least_frequent = find_least_frequent(input)
    strings = Enum.map(input, fn row -> Enum.join(row, "") end)

    find_co2_scrubber_rating(strings, least_frequent, 0)
  end

  def find_co2_scrubber_rating([rating], _, _) do
    rating
  end

  def find_co2_scrubber_rating(input, least_frequent, current_index) do
    least_frequent_bit = Enum.at(least_frequent, current_index)
    filtered = Enum.filter(input, fn row -> String.at(row, current_index) == least_frequent_bit end)
    least_frequent = find_least_frequent_string(filtered)

    find_co2_scrubber_rating(filtered, least_frequent, current_index + 1)
  end

  def find_least_frequent_string(strings) do
    parsed = Enum.map(strings, &Day3.parse_line/1)
    find_least_frequent(parsed)
  end
end

[filename] = System.argv()
input = Day3.read_input(filename)

result1 = Day3.task1(input)
result2 = Day3.task2(input)

IO.puts("Task 1: " <> Integer.to_string(result1))
IO.puts("Task 2: " <> Integer.to_string(result2))
