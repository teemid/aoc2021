defmodule Day2 do
  def read_input(file_name) do
    File.open!(file_name)
      |> IO.read(:all)
      |> String.split("\n")
      |> Enum.map(&Day2.parse_command/1)
  end

  def parse_command("forward " <> number), do: {:forward, String.to_integer(number)}
  def parse_command("down " <> number), do: {:down, String.to_integer(number)}
  def parse_command("up " <> number), do: {:up, String.to_integer(number)}

  def task1(input) do
    result = Enum.reduce(input, %{horizontal: 0, vertical: 0}, fn {command, number}, acc ->
      case command do
        :up -> %{horizontal: acc.horizontal, vertical: acc.vertical - number}
        :down -> %{horizontal: acc.horizontal, vertical: acc.vertical + number}
        :forward -> %{horizontal: acc.horizontal + number, vertical: acc.vertical}
      end
    end)

    result.horizontal * result.vertical
  end

  def task2(input) do
    result = Enum.reduce(input, %{horizontal: 0, vertical: 0, aim: 0}, fn {command, number}, acc ->
      case command do
        :up -> %{horizontal: acc.horizontal, vertical: acc.vertical, aim: acc.aim - number}
        :down -> %{horizontal: acc.horizontal, vertical: acc.vertical, aim: acc.aim + number}
        :forward -> %{horizontal: acc.horizontal + number, vertical: acc.vertical + number * acc.aim, aim: acc.aim}
      end
    end)

    result.horizontal * result.vertical
  end
end

[file_name] = System.argv()

input = Day2.read_input(file_name)

IO.puts("Task 1: " <> Integer.to_string(Day2.task1(input)))
IO.puts("Task 2: " <> Integer.to_string(Day2.task2(input)))
