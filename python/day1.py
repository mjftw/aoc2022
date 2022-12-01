def solve(path):
    f = open(path)
    lines = f.readlines()

    elf_calories = []
    total = 0
    for line in lines:
        if line == "\n":
            elf_calories.append(total)
            total = 0
        else:
            total += int(line)

    answer_part_1 = max(elf_calories)
    answer_part_2 = sum(sorted(elf_calories, reverse=True)[0:3])

    print(f"Part 1 answer: {answer_part_1}")
    print(f"Part 2 answer: {answer_part_2}")
