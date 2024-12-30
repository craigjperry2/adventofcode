program = [
    # 0, 1, 5, 4, 3, 0
    2, 4, 1, 2, 7, 5, 4, 3, 0, 3, 1, 7, 5, 5, 3, 0,
]

# A = 729
A = 61657405
B = 0
C = 0

ip = 0

output = []

combo = lambda r: {4: A, 5: B, 6: C}.get(r, r)

while ip < len(program):
    instruction, operand = program[ip], program[ip + 1]

    if instruction == 0:
        A //= 2 ** combo(operand)

    elif instruction == 1:
        B ^= combo(operand)

    elif instruction == 2:
        B = combo(operand) % 8

    elif instruction == 3:
        if A != 0:
            ip = operand - 2

    elif instruction == 4:
        B ^= C

    elif instruction == 5:
        output.append(str(combo(operand) % 8))

    elif instruction == 6:
        B = A // 2 ** combo(operand)

    elif instruction == 7:
        C = A // 2 ** combo(operand)

    ip += 2

print( ",".join(output))
