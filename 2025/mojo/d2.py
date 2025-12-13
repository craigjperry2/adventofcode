inp = """11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124"""

def main():
    ranges = map(lambda x: tuple(str.split(x, "-")), str.split(inp, ","))
    for (start, end) in ranges:
        for i in range(len(start), len(end) + 1):
            if i % 2 != 0:
                continue
            print(f"({start}, {end}) {i}")
    print(list(ranges))

if __name__ == "__main__":
    main()
