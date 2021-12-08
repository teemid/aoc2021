defmodule Day6 do
  def read_input(filename) do
    File.open!(filename)
      |> IO.read(:all)
      |> String.split(",")
      |> Enum.map(&String.to_integer/1)
  end

  def simulate_school(fish, number_of_days) do
    simulate_school(fish, number_of_days, length(fish))
  end

  defp simulate_school(_fish, 0, acc) do
    acc
  end

  defp simulate_school(fish, days_left, _acc) do
    fish = fish
      |> spawn_fish()
      |> simulate_day()

    simulate_school(fish, days_left - 1, length(fish))
  end

  def simulate_day(fish) do
    simulate_day(fish, [])
  end

  defp simulate_day([], acc) do
    acc
  end

  defp simulate_day([fish | rest_of_fish], acc) do
    simulate_day(rest_of_fish, [fish - 1 | acc])
  end

  def spawn_fish(fish) do
    spawn_fish(fish, [])
  end

  defp spawn_fish([], acc) do
    acc
  end

  defp spawn_fish([fish | rest_of_fish], acc) do
    case fish do
      0 -> spawn_fish(rest_of_fish, [9 | [7 | acc]])
      _ -> spawn_fish(rest_of_fish, [fish | acc])
    end
  end

  def task1() do
    [_, filename, number_of_days] = System.argv()
    number_of_days = String.to_integer(number_of_days)

    fish = Day6.read_input(filename)
    result = simulate_school(fish, number_of_days)

    IO.puts("Task 1: " <> Integer.to_string(result))
  end

  def task2() do
    [_, filename, number_of_days, worker_thread_count] = System.argv()
    number_of_days = String.to_integer(number_of_days)
    worker_thread_count = String.to_integer(worker_thread_count)

    fish = Day6.read_input(filename)
    result = simulate_school_parallel(fish, number_of_days, worker_thread_count)

    IO.puts("Task 2: " <> Integer.to_string(result))
  end

  # NOTE (Emil): This does not work for task 2, you run out of memory long before you get an answer.
  defp simulate_school_parallel(fish, number_of_days, worker_thread_count) do
    input_size = trunc((length(fish) / worker_thread_count) + 0.5)

    fish
      |> Enum.chunk_every(input_size)
      |> Enum.map(fn fish ->
          ref = make_ref()
          {ref, spawn(__MODULE__, :simulation_worker, [{ref, self()}, fish, number_of_days])}
      end)
      |> Enum.map(fn {ref, pid} ->
        receive do
          {^ref, ^pid, result} -> result
        end
      end)
      |> Enum.reduce(0, fn result, acc -> result + acc end)
  end

  def simulation_worker({ref, from}, fish, number_of_days) do
    result = simulate_school(fish, number_of_days)
    send(from, {ref, self(), result})
  end
end

defmodule Day6Part2 do
  def simulate_school(fish, number_of_days) do
    simulate_school(fish, number_of_days, length(fish))
  end

  defp simulate_school(_fish, 0, acc) do
    acc
  end

  defp simulate_school(fish, days_left, _acc) do
    fish = fish
      |> spawn_fish()
      |> simulate_day()

    simulate_school(fish, days_left - 1, count_fish(fish))
  end

  def count_fish(fish) do
    Enum.sum(fish)
  end

  def simulate_day([d0, d1, d2, d3, d4, d5, d6, d7, d8, d9]) do
    [d1, d2, d3, d4, d5, d6, d7 + d0, d8, d9, 0]
  end

  def spawn_fish([d0, d1, d2, d3, d4, d5, d6, d7, d8, 0]) do
    [d0, d1, d2, d3, d4, d5, d6, d7, d8, d0]
  end

  def task() do
    [_, filename, number_of_days] = System.argv()
    number_of_days = String.to_integer(number_of_days)

    fish = Day6.read_input(filename)
    fish = group_fish(fish)

    result = simulate_school(fish, number_of_days)

    IO.puts("Task 2: " <> Integer.to_string(result))
  end

  def group_fish(fish) do
    grouped_fish = List.duplicate(0, 10)

    fish = fish
      |> Enum.group_by(fn e -> e end)
      |> Enum.map(fn {key, value} -> {key, length(value)} end)

    Enum.reduce(fish, grouped_fish, fn {key, value}, acc -> List.replace_at(acc, key, value) end)
  end
end

[task_select | _] = System.argv()

case task_select do
  "1" -> Day6.task1()
  "2" -> Day6Part2.task()
end
