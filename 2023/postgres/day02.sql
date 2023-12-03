-- Advent of Code 2023, Day 2, Part 1

-- Goal: Calculate the sum of valid game's id's

-- This means i want to be able write a query like:
select sum(game_id) from rounds where game_id not in (
    select distinct game_id from rounds where red > 12 or green > 13 or blue > 14
);

-- That suggests a table something like:
CREATE TABLE day2_rounds (
    game_id INTEGER not null,
    round_id INTEGER not null,
    red INTEGER DEFAULT 0 CHECK ( red >= 0 ),
    green INTEGER default 0 check ( green >= 0 ),
    blue INTEGER default 0 check ( blue >= 0 ),
    PRIMARY KEY (game_id, round_id)
);

-- But the provided data format isn't conducive to easy loading into a table like that.

-- So STEP 1: Create a staging table
CREATE TABLE day2_rounds_staging (
    game_id integer not null primary key,
    rounds text NOT NULL CHECK ( char_length(rounds) >= 0 )
);

-- STEP 2: Load the staging table, using some light GNU sed to split the Game ID from the details of the rounds
\COPY day2_rounds_staging FROM PROGRAM 'gsed "s/^Game \([[:digit:]]\+\):/\1\t/" day02.txt' WITH delimiter E'\t';

-- STEP 3: Develop a query to unnest the rounds and associate with a round_id
select game_id, row_number() over (partition by game_id) as round_id, round
from day2_rounds_staging, unnest(string_to_array(rounds, ';')) as round;

-- STEP 4: Refine it to split the rounds into their components
select
    game_id,
    row_number() over (partition by game_id) as round_id,
    substring(round from '(\d+) green') as green,
    substring(round from '(\d+) red') as red,
    substring(round from '(\d+) blue') as blue
from day2_rounds_staging, unnest(string_to_array(rounds, ';')) as round;

-- STEP 5: Create & populate the table
CREATE TABLE day2_rounds AS (
    select
        game_id,
        row_number() over (partition by game_id) as round_id,
        substring(round from '(\d+) green')::int as green,
        substring(round from '(\d+) red')::int as red,
        substring(round from '(\d+) blue')::int as blue
    from day2_rounds_staging, unnest(string_to_array(rounds, ';')) as round
);

-- STEP 6: Add the constraints that were sketched out before
ALTER TABLE day2_rounds
    ADD CONSTRAINT day2_rounds_game_id_check CHECK ( game_id >= 0 ),
    ADD CONSTRAINT day2_rounds_round_id_check CHECK ( round_id >= 0 ),
    ADD CONSTRAINT day2_rounds_red_check CHECK ( red >= 0 ),
    ADD CONSTRAINT day2_rounds_green_check CHECK ( green >= 0 ),
    ADD CONSTRAINT day2_rounds_blue_check CHECK ( blue >= 0 );

ALTER TABLE day2_rounds
    ADD PRIMARY KEY (game_id, round_id);

-- STEP 7: Run the query to get the answer
select sum(distinct game_id) from day2_rounds where game_id not in (
    select game_id from day2_rounds where red > 12 or green > 13 or blue > 14
);

-- SOLVED!

-- Time to learn from the query plans!
-- Bearing in mind the row sizes here are tiny and will affect what's learnable here...

EXPLAIN ANALYSE select sum(distinct game_id) from day2_rounds where game_id not in (
    select game_id from day2_rounds where red > 12 or green > 13 or blue > 14
);
-- QUERY PLAN
-- Aggregate  (cost=32.44..32.45 rows=1 width=8) (actual time=0.327..0.328 rows=1 loops=1)
--   ->  Sort  (cost=31.28..31.86 rows=232 width=4) (actual time=0.295..0.308 rows=200 loops=1)
--         Sort Key: day2_rounds.game_id
--         Sort Method: quicksort  Memory: 25kB
--         ->  Seq Scan on day2_rounds  (cost=12.38..22.17 rows=232 width=4) (actual time=0.179..0.276 rows=200 loops=1)
--               Filter: (NOT (hashed SubPlan 1))
--               Rows Removed by Filter: 263
--               SubPlan 1
--                 ->  Seq Scan on day2_rounds day2_rounds_1  (cost=0.00..12.10 rows=110 width=4) (actual time=0.007..0.112 rows=111 loops=1)
--                       Filter: ((red > 12) OR (green > 13) OR (blue > 14))
--                       Rows Removed by Filter: 352
-- Planning Time: 0.288 ms
-- Execution Time: 0.386 ms

-- Does adding a distinct to the subquery help?
EXPLAIN ANALYSE select sum(distinct game_id) from day2_rounds where game_id not in (
    select distinct game_id from day2_rounds where red > 12 or green > 13 or blue > 14
);
-- QUERY PLAN
-- Aggregate  (cost=33.34..33.35 rows=1 width=8) (actual time=0.360..0.361 rows=1 loops=1)
--   ->  Sort  (cost=32.18..32.76 rows=232 width=4) (actual time=0.328..0.341 rows=200 loops=1)
--         Sort Key: day2_rounds.game_id
--         Sort Method: quicksort  Memory: 25kB
--         ->  Seq Scan on day2_rounds  (cost=13.28..23.07 rows=232 width=4) (actual time=0.171..0.291 rows=200 loops=1)
--               Filter: (NOT (hashed SubPlan 1))
--               Rows Removed by Filter: 263
--               SubPlan 1
--                 ->  HashAggregate  (cost=12.38..13.10 rows=72 width=4) (actual time=0.114..0.123 rows=53 loops=1)
--                       Group Key: day2_rounds_1.game_id
--                       Batches: 1  Memory Usage: 24kB
--                       ->  Seq Scan on day2_rounds day2_rounds_1  (cost=0.00..12.10 rows=110 width=4) (actual time=0.008..0.082 rows=111 loops=1)
--                             Filter: ((red > 12) OR (green > 13) OR (blue > 14))
--                             Rows Removed by Filter: 352
-- Planning Time: 0.272 ms
-- Execution Time: 0.439 ms

-- What about an index on game_id?
CREATE INDEX ON day2_rounds (game_id);

EXPLAIN ANALYSE select sum(distinct game_id) from day2_rounds where game_id not in (
    select game_id from day2_rounds where red > 12 or green > 13 or blue > 14
);
-- QUERY PLAN
-- Aggregate  (cost=32.44..32.45 rows=1 width=8) (actual time=0.268..0.269 rows=1 loops=1)
--   ->  Sort  (cost=31.28..31.86 rows=232 width=4) (actual time=0.236..0.250 rows=200 loops=1)
--         Sort Key: day2_rounds.game_id
--         Sort Method: quicksort  Memory: 25kB
--         ->  Seq Scan on day2_rounds  (cost=12.38..22.17 rows=232 width=4) (actual time=0.114..0.217 rows=200 loops=1)
--               Filter: (NOT (hashed SubPlan 1))
--               Rows Removed by Filter: 263
--               SubPlan 1
--                 ->  Seq Scan on day2_rounds day2_rounds_1  (cost=0.00..12.10 rows=110 width=4) (actual time=0.007..0.074 rows=111 loops=1)
--                       Filter: ((red > 12) OR (green > 13) OR (blue > 14))
--                       Rows Removed by Filter: 352
-- Planning Time: 0.673 ms
-- Execution Time: 0.308 ms
DROP INDEX day2_rounds_game_id_idx;

-- What happens if there's an index on red/green/blue?
CREATE INDEX ON day2_rounds (red);
CREATE INDEX ON day2_rounds (green);
CREATE INDEX ON day2_rounds (blue);

EXPLAIN ANALYSE select sum(distinct game_id) from day2_rounds where game_id not in (
    select game_id from day2_rounds where red > 12 or green > 13 or blue > 14
);
-- QUERY PLAN
-- Aggregate  (cost=32.44..32.45 rows=1 width=8) (actual time=0.697..0.698 rows=1 loops=1)
--   ->  Sort  (cost=31.28..31.86 rows=232 width=4) (actual time=0.642..0.674 rows=200 loops=1)
--         Sort Key: day2_rounds.game_id
--         Sort Method: quicksort  Memory: 25kB
--         ->  Seq Scan on day2_rounds  (cost=12.38..22.17 rows=232 width=4) (actual time=0.141..0.250 rows=200 loops=1)
--               Filter: (NOT (hashed SubPlan 1))
--               Rows Removed by Filter: 263
--               SubPlan 1
--                 ->  Seq Scan on day2_rounds day2_rounds_1  (cost=0.00..12.10 rows=110 width=4) (actual time=0.019..0.090 rows=111 loops=1)
--                       Filter: ((red > 12) OR (green > 13) OR (blue > 14))
--                       Rows Removed by Filter: 352
-- Planning Time: 1.318 ms
-- Execution Time: 0.768 ms
DROP INDEX day2_rounds_red_idx;
DROP INDEX day2_rounds_green_idx;
DROP INDEX day2_rounds_blue_idx;

-- What about a composite index?
CREATE INDEX ON day2_rounds (red, green, blue);
EXPLAIN ANALYSE select sum(distinct game_id) from day2_rounds where game_id not in (
    select game_id from day2_rounds where red > 12 or green > 13 or blue > 14
);
-- QUERY PLAN
-- Aggregate  (cost=32.44..32.45 rows=1 width=8) (actual time=0.158..0.159 rows=1 loops=1)
--   ->  Sort  (cost=31.28..31.86 rows=232 width=4) (actual time=0.135..0.145 rows=200 loops=1)
--         Sort Key: day2_rounds.game_id
--         Sort Method: quicksort  Memory: 25kB
--         ->  Seq Scan on day2_rounds  (cost=12.38..22.17 rows=232 width=4) (actual time=0.068..0.121 rows=200 loops=1)
--               Filter: (NOT (hashed SubPlan 1))
--               Rows Removed by Filter: 263
--               SubPlan 1
--                 ->  Seq Scan on day2_rounds day2_rounds_1  (cost=0.00..12.10 rows=110 width=4) (actual time=0.004..0.044 rows=111 loops=1)
--                       Filter: ((red > 12) OR (green > 13) OR (blue > 14))
--                       Rows Removed by Filter: 352
-- Planning Time: 0.291 ms
-- Execution Time: 0.183 ms
DROP INDEX day2_rounds_red_green_blue_idx;


----------------------------------------------------------

-- PART 2

-- Seems like this can be tackled in 3 steps: min no. cubes per game, the power of those cubes and finally the sum

-- STEP 1: Min no. cubes per game
WITH cubes AS (
    select
        game_id,
        max(red) as red,
        max(green) as green,
        max(blue) as blue
    from
        day2_rounds
    group by game_id
),
-- STEP 2: Power of those cubes
powers AS (
    SELECT
        game_id,
        red * green * blue as power
    FROM cubes
    order by game_id
)
-- STEP 3: Sum of those powers
SELECT SUM(power) FROM powers;

-- SOLVED!
