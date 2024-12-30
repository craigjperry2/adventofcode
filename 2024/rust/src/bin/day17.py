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

operations = {
    0: lambda op: A // (2 ** combo(op)),
    1: lambda op: B ^ combo(op),
    2: lambda op: combo(op) % 8,
    3: lambda op: ip if A == 0 else op - 2,
    4: lambda _: B ^ C,
    5: lambda op: output.append(str(combo(op) % 8)),
    6: lambda op: A // (2 ** combo(op)),
    7: lambda op: A // (2 ** combo(op)),
}

while ip < len(program):
    instruction, operand = program[ip], program[ip + 1]
    
    result = operations[instruction](operand)

    match instruction:
        case 0:
            A = result
        case 1 | 2 | 4 | 6:
            B = result
        case 7:
            C = result
        case 3:
            ip = result

    ip += 2

print(",".join(output))
