-- Advent of Code 2023, Day 1, Part 1

-- Create a table to hold the input data
-- Postgres has a feature called "foreign data wrapper" that allows us to query the raw file as if it were a table
-- but i'd rather load the data into a table
CREATE TABLE day1
(
    -- From the problem statement, i know each row is not null and not empty
    -- but i can't say if duplicates are allowed or not
    calibration TEXT NOT NULL CHECK ( char_length(calibration) > 0 )
);


-- Load the input data into the table, in psql i'd do something like:
\COPY day1 FROM PROGRAM 'cat ../data/day01.txt';

-- but since i'm test driving datagrip, i right clicked on the data file and selected import to database.
-- It offered to create a table for me but i declined.


-- SOLVING THE PROBLEM - PART 1
-- I can see i need a sum of the calibration values but but i'm going to start by extracting the calibration values:

SELECT calibration,
       substring(calibration from '\d') as first
FROM day1;

-- looks good, now i need to extract the last digit and concatenate
SELECT calibration,
       substring(calibration from '\d') ||
       substring(reverse(calibration) from '\d') as value
FROM day1;

-- I can solve the problem now:
select sum((substring(calibration from '\d') ||
            substring(reverse(calibration) from '\d'))::int) as result
from day1;
-- The only gotcha here is subtring returns a string, so (...)::int cast needed for sum

-- SOLVED

-------------------------------------------------------------------------------

-- SOLVING THE PROBLEM - PART 2
-- This query is run once so i'm going to use a CTE to replace the words with numbers

WITH translated_calibrations AS (
-- i can't use translate() because it replaces single characters not words
-- at this scale (1k rows) anything works, these 10 sequential loops over the string are not a problem. At larger scale though...
-- NB: this is O(10n) not O(n^10) despite the nested syntax
SELECT REPLACE(REPLACE(REPLACE(REPLACE(REPLACE(REPLACE(REPLACE(REPLACE(REPLACE(
        calibration,
        'one', '1'), 'two', '2'), 'three', '3'), 'four', '4'), 'five', '5'), 'six', '6'), 'seven', '7'), 'eight', '8'), 'nine', '9')
    as calibration
FROM day1
)
SELECT sum(
            (substring(calibration from '\d') || substring(reverse(calibration) from '\d'))::int
       ) as result
FROM translated_calibrations;

-- PROBLEM
-- The above doesn't work - too low a value.
-- Looking at the examples i see this case: 'zoneight234' == 14
-- with my attempt above, if i replace one first, then 'z1ight234' will never become 'z18234' like it should

-- SOLUTION
-- First, find the first digit or word-digit
-- Then translate the word-digit to a digit
-- Repeat the same starting from the end of the string (i.e. all reversed)
-- Concatenate the two results
-- Sum the results as before

WITH translated_calibrations AS (
    SELECT (
                REPLACE(REPLACE(REPLACE(REPLACE(REPLACE(REPLACE(REPLACE(REPLACE(REPLACE(
                    substring(calibration from '\d|one|two|three|four|five|six|seven|eight|nine'),
                'one', '1'), 'two', '2'), 'three', '3'), 'four', '4'), 'five', '5'), 'six', '6'), 'seven', '7'), 'eight', '8'), 'nine', '9')
               ||
                REPLACE(REPLACE(REPLACE(REPLACE(REPLACE(REPLACE(REPLACE(REPLACE(REPLACE(
                    substring(reverse(calibration) from '\d|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin'),
                'eno', '1'), 'owt', '2'), 'eerht', '3'), 'ruof', '4'), 'evif', '5'), 'xis', '6'), 'neves', '7'), 'thgie', '8'), 'enin', '9')
            )::int
        as calibration
    FROM day1
)
SELECT sum(calibration) from translated_calibrations;

-- SOLVED

-------------------------------------------------------------------------------

-- I'm interested to see the explain analyze for the above query:

EXPLAIN ANALYZE WITH translated_calibrations AS (
    SELECT (
                REPLACE(REPLACE(REPLACE(REPLACE(REPLACE(REPLACE(REPLACE(REPLACE(REPLACE(
                    substring(calibration from '\d|one|two|three|four|five|six|seven|eight|nine'),
                'one', '1'), 'two', '2'), 'three', '3'), 'four', '4'), 'five', '5'), 'six', '6'), 'seven', '7'), 'eight', '8'), 'nine', '9')
               ||
                REPLACE(REPLACE(REPLACE(REPLACE(REPLACE(REPLACE(REPLACE(REPLACE(REPLACE(
                    substring(reverse(calibration) from '\d|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin'),
                'eno', '1'), 'owt', '2'), 'eerht', '3'), 'ruof', '4'), 'evif', '5'), 'xis', '6'), 'neves', '7'), 'thgie', '8'), 'enin', '9')
            )::int
        as calibration
    FROM day1
)
SELECT sum(calibration) from translated_calibrations;

-- QUERY PLAN
-- Aggregate  (cost=79.50..79.51 rows=1 width=8) (actual time=13.138..13.139 rows=1 loops=1)
--   ->  Seq Scan on day1  (cost=0.00..17.00 rows=1000 width=22) (actual time=0.016..0.188 rows=1000 loops=1)
-- Planning Time: 0.196 ms
-- Execution Time: 13.198 ms

-- We have to table scan day1 anyway, so there's no benefit to indexing the table, so fancier approaches like hash indexes won't help
-- In any case, the Seq Scan on day1 turns out to be super cheap, especially at this size of only 1k rows

-- I can't really see any more detail since the cost of the regexp extraction, the replacements and the type casting all get rolled up into the SUM() Aggregate
-- looking up the manual, i can request more detail with:

EXPLAIN (ANALYZE, VERBOSE, COSTS, BUFFERS) WITH translated_calibrations AS ( ...

-- QUERY PLAN
-- Aggregate  (cost=79.50..79.51 rows=1 width=8) (actual time=13.589..13.590 rows=1 loops=1)
-- "  Output: sum(((replace(replace(replace(replace(replace(replace(replace(replace(replace(""substring""(day1.calibration, '\d|one|two|three|four|five|six|seven|eight|nine'::text), 'one'::text, '1'::text), 'two'::text, '2'::text), 'three'::text, '3'::text), 'four'::text, '4'::text), 'five'::text, '5'::text), 'six'::text, '6'::text), 'seven'::text, '7'::text), 'eight'::text, '8'::text), 'nine'::text, '9'::text) || replace(replace(replace(replace(replace(replace(replace(replace(replace(""substring""(reverse(day1.calibration), '\d|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin'::text), 'eno'::text, '1'::text), 'owt'::text, '2'::text), 'eerht'::text, '3'::text), 'ruof'::text, '4'::text), 'evif'::text, '5'::text), 'xis'::text, '6'::text), 'neves'::text, '7'::text), 'thgie'::text, '8'::text), 'enin'::text, '9'::text)))::integer)"
--   Buffers: shared hit=7
--   ->  Seq Scan on public.day1  (cost=0.00..17.00 rows=1000 width=22) (actual time=0.018..0.320 rows=1000 loops=1)
--         Output: day1.calibration
--         Buffers: shared hit=7
-- Planning Time: 0.207 ms
-- Execution Time: 13.633 ms

-- Not really much more detail in this case. I suppose i could go a few ways:
-- 1. Add more rows to exaggerate relative costs
-- 2. Compare alternative strategies, see their runtime & explain analyze outputs

-- As for other strategies, i'd love to know if i can abuse translate() to replace e.g. 'fi' with 5 and 'fo' with 4,
-- somehow tricking it into thinking 2 UTF-8 chars are 1 multibyte char

-- I also wonder if a regex replace would be faster than these nested replace() calls - 8 of them are noops currently
