def hashing(step):
    cv = 0
    for s in step:
        cv = (ord(s) + cv) * 17 % 256
    return cv


def part1(sequence):
    total = 0
    for step in sequence:
        total += hashing(step)

    print("Part 1: ", total)


def part2(sequence):
    new_sequence = []
    for step in sequence:
        if "=" in step:
            label, focal = step.split("=")
            new_sequence.append((label, focal))
        elif "-" in step:
            label = step[:-1]
            new_sequence.append((label, -1))

    boxes = []
    for _ in range(256):
        boxes.append([])

    for label, focal in new_sequence:
        box_i = hashing(label)
        # Remove label operation
        if focal == -1:
            boxes[box_i] = [(l, f) for l, f in boxes[box_i] if label != l]
        else:
            for i, lens in enumerate(boxes[box_i]):
                if lens[0] == label:
                    boxes[box_i][i] = (label, focal)
                    break
            else:
                boxes[box_i].append((label, focal))

            boxes[box_i]

        # print(f"Round {label} -- {focal}")
        # print(boxes[:5])
        # print("End ROUND ----")
    total = 0
    for i, box in enumerate(boxes):
        total += sum([(i + 1) * (j + 1) * int(l[1]) for j, l in enumerate(box)])

    print("Part 2: ", total)


if __name__ == "__main__":
    sequence = open(0).read().strip().split(",")
    # part1(sequence)
    part2(sequence)
