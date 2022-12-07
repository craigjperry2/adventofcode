import { assert } from "https://deno.land/std@0.167.0/testing/asserts.ts";

Deno.test("An example test", () => {
    assert(true);
});

// run with deno test --allow-read index.ts
Deno.test("Day 1 Part 1", async () => {
    const input = await loadInput(1);
    const result = input
        .split("\n\n")
        .map((batch) => batch
            .split("\n")
            .map(Number)
            .reduce((a, b) => a + b, 0))
        .reduce((a, b) => Math.max(a, b), 0);
    console.log(`Max calories: ${result}`);
});

Deno.test("Day 1 Part 2", async () => {
    const input = await loadInput(1);
    const result = input
        .split("\n\n")
        .map((batch) => batch
            .split("\n")
            .map(Number)
            .reduce((a, b) => a + b, 0))
        .sort((a, b) => b - a)
        .slice(0, 3)
        .reduce((a, b) => a + b, 0);
    console.log(`Max calories: ${result}`);
});

const loadInput = async (day: number): Promise<string> => {
    const data = await Deno.readFile(`resources/day${day}.txt`);
    return new TextDecoder("utf-8").decode(data);
};

// run with deno run --allow-read index.ts
const main = () => {
    console.log("Craig's Advent of Code 2022 in Typescript on Deno\n");
}

if (import.meta.main) {
  main();
}
