rotations = """L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"""

rotations = open("d1.txt").read().strip()

def parse(rotations: str) -> list[int]:
    return [int(r[1:]) if r[0] == "R" else -int(r[1:]) for r in rotations.split("\n")]

def parse2(rotations: str) -> list[tuple[str, int]]:
    return [(r[0], int(r[1:])) for r in rotations.split("\n")]

def count_all_zeros(rotations: list[tuple[str, int]]) -> int:
    position = 50
    zeros = 0
    for (d, r) in rotations:
        if d == "R":  # Moving right
            # Count how many 100-boundaries we cross
            zeros += (position + r) // 100
            position = (position + r) % 100
        else:  # Moving left
            # Count how many times we go through 0
            # We hit 0 if position - r <= 0 and position > 0
            if position > 0 and r >= position:
                # First crossing at position steps, then every 100 after
                zeros += 1 + (r - position) // 100
            elif position == 0 and r > 0:
                zeros += r // 100
            position = (position - r) % 100
    return zeros

def count_zeros(rotations: list[int]) -> int:
    position = 50
    zeros = 0
    for r in rotations:
        position = (position + r) % 100
        zeros += position == 0
    return zeros

print(count_all_zeros(parse2(rotations)))
