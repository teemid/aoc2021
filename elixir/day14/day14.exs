defmodule Day14 do
    def read_input(filename) do
        parts = File.open!(filename)
            |> IO.read(:all)
            |> String.split("\n\n")

        polymer_template = Enum.at(parts, 0)
        mapping = parts
            |> Enum.at(1)
            |> String.split("\n")
            |> Enum.map(&Day14.parse_line/1)
            |> Enum.reduce(%{}, fn {key, value}, acc -> add_to_mapping(acc, key, value) end)

        {polymer_template, mapping}
    end

    def parse_line(line) do
        parts = String.split(line, " -> ", trim: true)

        {Enum.at(parts, 0), Enum.at(parts, 1)}
    end

    defp add_to_mapping(acc, key, value), do: Map.put(acc, key, value)

    def task1(polymer_template, mapping, step_count) do
        frequencies = polymer_template
            |> step(mapping, step_count)
            |> count_frequencies()

        max = get_max_frequency(frequencies)
        min = get_min_frequency(frequencies)

        max - min
    end

    def step(polymer_template, _mapping, 0) do
        polymer_template
    end

    def step(polymer_template, mapping, step_count) do
        string_length = String.length(polymer_template)
        length = string_length - 2

        result = 0..length
            |> Enum.map(fn e -> String.slice(polymer_template, e..e+1) end)
            |> Enum.reduce("", fn e, acc -> acc <> polymer_template_replace(e, mapping) end)

        last_character = String.at(polymer_template, string_length - 1)
        polymer_template = result <> last_character
        step(polymer_template, mapping, step_count - 1)
    end

    def polymer_template_replace(template, mapping) do
        a = String.at(template, 0)
        replace_character = Map.get(mapping, template)

        a <> replace_character
    end

    def count_frequencies(polymer_template) do
        count_frequencies(polymer_template, %{})
    end

    defp count_frequencies("", frequencies) do
        frequencies
    end

    defp count_frequencies(polymer_template, frequencies) do
        length = String.length(polymer_template)

        c = String.at(polymer_template, 0)
        rest = String.slice(polymer_template, 1..length - 1)

        frequency = Map.get(frequencies, c, 0)
        frequencies = Map.put(frequencies, c, frequency + 1)

        count_frequencies(rest, frequencies)
    end

    def get_max_frequency(frequencies) do
        keys = Map.keys(frequencies)

        get_max_frequency(keys, frequencies, 0)
    end

    defp get_max_frequency([], _frequencies, max) do
        max
    end

    defp get_max_frequency([key | keys], frequencies, max) do
        frequency = Map.get(frequencies, key)
        if frequency > max do
            get_max_frequency(keys, frequencies, frequency)
        else
            get_max_frequency(keys, frequencies, max)
        end
    end

    def get_min_frequency(frequencies) do
        keys = Map.keys(frequencies)

        count = frequencies
            |> Map.values()
            |> Enum.sum()

        get_min_frequency(keys, frequencies, count)
    end

    defp get_min_frequency([], _frequencies, min) do
        min
    end

    defp get_min_frequency([key | keys], frequencies, min) do
        frequency = Map.get(frequencies, key)
        if frequency < min do
            get_min_frequency(keys, frequencies, frequency)
        else
            get_min_frequency(keys, frequencies, min)
        end

    end
end

[filename, step_count] = System.argv()
step_count = String.to_integer(step_count)

{polymer_template, mapping} = Day14.read_input(filename)
result1 = Day14.task1(polymer_template, mapping, step_count)
IO.inspect(result1)
