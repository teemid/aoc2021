defmodule Day1 do
  def is_increased(_, nil), do: 0

  def is_increased(e, last) do
    if e - last > 0 do
      1
    else
      0
    end
  end

  def task1(file_name) do
    result =
      File.open!(file_name)
      |> IO.read(:all)
      |> String.split("\n")
      |> Enum.map(&String.to_integer/1)
      |> Enum.reduce(%{last: nil, count: 0}, fn e, acc ->
        count = is_increased(e, acc.last)
        %{last: e, count: acc.count + count}
      end)

    result.count
  end

  def task2(file_name) do
    result =
      File.open!(file_name)
      |> IO.read(:all)
      |> String.split("\n")
      |> Enum.map(&String.to_integer/1)
      |> Enum.chunk_every(3, 1)
      |> Enum.filter(fn e -> length(e) == 3 end)
      |> Enum.map(fn [a, b, c] -> a + b + c end)
      |> Enum.reduce(%{last: nil, count: 0}, fn e, acc ->
        count = is_increased(e, acc.last)
        %{last: e, count: acc.count + count}
      end)

    result.count
  end
end

[file_name] = System.argv()

IO.puts("Task 1: " <> Integer.to_string(Day1.task1(file_name)))
IO.puts("Task 2: " <> Integer.to_string(Day1.task2(file_name)))
