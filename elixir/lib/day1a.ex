defmodule Day1a do
  def solve(input_path) do
    {tail, head} =
      input_path
      |> File.stream!()
      |> Enum.reduce({[], 0}, &reducer/2)

    all = [head | tail]

    answer_part1 = Enum.max(all)

    answer_part2 =
      all
      |> Enum.sort(:desc)
      |> Enum.take(3)
      |> Enum.sum()

    IO.puts("Answer to part 1: #{answer_part1}")
    IO.puts("Answer to part 2: #{answer_part2}")
  end

  defp reducer(line, {all_elves_totals, current_elf_calories}) when line == "\n",
    do: {[current_elf_calories | all_elves_totals], 0}

  defp reducer(line, {all_elves_totals, current_elf_calories}) do
    {calories, _} = Integer.parse(line)
    {all_elves_totals, current_elf_calories + calories}
  end
end
