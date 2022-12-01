defmodule Day1a do
  @spec solve(String.t()) :: :ok
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

  @spec reducer(String.t(), {[non_neg_integer()], non_neg_integer()}) ::
          {[non_neg_integer()], non_neg_integer()}
  defp reducer(line, {all_elves_totals, current_elf_calories}) do
    case line do
      "\n" ->
        {[current_elf_calories | all_elves_totals], 0}

      calories_string ->
        {calories, _} = calories_string |> Integer.parse()
        {all_elves_totals, current_elf_calories + calories}
    end
  end
end
