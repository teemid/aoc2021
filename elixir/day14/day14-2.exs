defmodule Day14 do
    def read_input(filename) do
        parts = File.open!(filename)
            |> IO.read(:all)
            |> String.split("\n\n")

        polymer_template = Enum.at(parts, 0)
        mapping = parts
            |> Enum.at(1)
            |> String.split("\n", trim: true)
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
        character_frequencies = polymer_template
            |> create_frequency_table()
            |> step(mapping, step_count)
            |> calculate_character_frequencies(polymer_template)

        max = get_max_frequency(character_frequencies)
        min = get_min_frequency(character_frequencies)

        max - min
    end

    def create_frequency_table(polymer_template) do
        length = String.length(polymer_template) - 2

        0..length
            |> Enum.map(fn e -> String.slice(polymer_template, e..e+1) end)
            |> Enum.reduce(%{}, fn e, acc -> Map.update(acc, e, 1, &(&1 + 1)) end)
    end

    defp step(frequencies, _mapping, 0) do
        frequencies
    end

    defp step(frequencies, mapping, step_count) do
        keys = Map.keys(frequencies)

        frequencies = do_replace(keys, mapping, frequencies)
        step(frequencies, mapping, step_count - 1)
    end

    def do_replace(keys, mapping, frequencies) do
        do_replace(keys, mapping, frequencies, %{})
    end

    defp do_replace([], _mapping, _old_frequencies, new_frequencies) do
        new_frequencies
    end

    defp do_replace([key | keys], mapping, old_frequencies, new_frequencies) do
        char = Map.get(mapping, key)
        a = String.at(key, 0)
        b = String.at(key, 1)

        replace_1 = a <> char
        replace_2 = char <> b

        original_frequency = Map.get(old_frequencies, key, 0)
        replace_1_frequency = Map.get(new_frequencies, replace_1, 0)
        replace_2_frequency = Map.get(new_frequencies, replace_2, 0)

        new_frequencies = new_frequencies
            |> Map.put(replace_1, original_frequency + replace_1_frequency)
            |> Map.put(replace_2, original_frequency + replace_2_frequency)

        do_replace(keys, mapping, old_frequencies, new_frequencies)
    end

    def calculate_character_frequencies(frequencies, polymer_template) do
        keys = Map.keys(frequencies)

        character_frequencies = calculate_character_frequencies(keys, frequencies, %{})

        first = String.first(polymer_template)
        last = String.last(polymer_template)

        # NOTE (Emil): Since we are operating on pairs of letters we count each letter twice
        # so we divide the count by two and then add in the letter at the front and back,
        # since they never change.
        character_frequencies
            |> halve_frequencies()
            |> Map.update(first, 0, fn e -> e + 1 end)
            |> Map.update(last, 0, fn e -> e + 1 end)
    end

    def halve_frequencies(character_frequencies) do
        halve_frequencies(Map.keys(character_frequencies), character_frequencies)
    end

    defp halve_frequencies([], character_frequencies), do: character_frequencies

    defp halve_frequencies([key | keys], character_frequencies) do
        halve_frequencies(keys, Map.update(character_frequencies, key, 0, fn e -> Integer.floor_div(e, 2) end))
    end

    defp calculate_character_frequencies([], _frequencies, character_frequencies) do
        character_frequencies
    end

    defp calculate_character_frequencies([key | keys], frequencies, character_frequencies) do
        original_frequency = Map.get(frequencies, key, 0)

        a = String.at(key, 0)
        b = String.at(key, 1)

        character_frequencies = character_frequencies
            |> update_character_frequencies(a, original_frequency)
            |> update_character_frequencies(b, original_frequency)

        calculate_character_frequencies(keys, frequencies, character_frequencies)
    end

    def update_character_frequencies(character_frequencies, character, original_frequency) do
        frequency = Map.get(character_frequencies, character, 0)
        Map.put(character_frequencies, character, original_frequency + frequency)
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
