-- for each part number, sum IIF adjacent to a symbol

-- Load the staging table
CREATE TABLE day3 (
    line text not null CHECK ( char_length(line) >= 0 )
);
\COPY day3 FROM PROGRAM 'cat day03.txt';


-- How long is a line?
select char_length(line) from day3 limit 1;

-- How many lines are there?
select count(*) from day3;

-- ANSWER: 140 x 140


-- Attempt the solution in parts by gradually building upon CTEs

-- Enumerate the lines of raw import day3 table, e.g. add the y column:
--    y | line
--   --------------------
--    1 | '...123...&...'
--    2 | '.45....67...%'
WITH enumerated AS (
    SELECT row_number() over () AS y, line
    FROM day3
),

-- Split the lines into rows:
--  y | n
-- ---------
--  1 | .
--  1 | .
--  1 | .
--  1 | '1'
--  1 | '2'
--  1 | '3'
rows AS (
    SELECT y, regexp_split_to_table(line, '') AS n
    FROM enumerated
),

-- Enumerate the column of each n:
--  y | x | n
-- -------------
--  1 | 1 | .
--  1 | 2 | .
--  1 | 3 | .
--  1 | 4 | '1'
--  1 | 5 | '2'
--  1 | 6 | '3'
cols AS (
    SELECT y, row_number() over (partition by y) AS x, n
    FROM rows
),

-- Strip out the '.' locations:
--  y | x | n
-- -------------
--  1 | 4 | '1'
--  1 | 5 | '2'
--  1 | 6 | '3'
locations as (
    select y, x, n
    from cols
    where n <> '.'
),

-- Construct a table expression of only the digit locations:
--  y | x | n
-- -------------
--  1 | 4 | '1'
--  1 | 5 | '2'
--  1 | 6 | '3'
digits as (
    select *
    from locations
    where n >= '0'
),

-- Construct a table expression of only the symbol locations:
--  y | x  | n
-- --------------
--  1 | 10 | '&'
--  2 | 13 | '%'
symbols as (
    select *
    from locations
    where n < '0'
),

-- select all digits which are adjacent to a symbol in either the X or Y directions:
--  y | x | n
-- -------------
--  2 | 9 | '7'  -- & is diagonally up-right in this example
adjacent as (
    select d.*
    from digits d inner join symbols s
    on d.x between s.x - 1 and s.x + 1
        and d.y between s.y - 1 and s.y + 1
    order by s.y, s.x, d.y, d.x
),

-- find the location of the first digit in each adjacent:
--  r | y | x
-- -----------
--  0 | 2 | 8  -- the 6 of 67 is at y=2, x=8 in this example
leftbounds as (
    select row_number() over () as r, c.y, c.x::int + 1 as x
    from adjacent a inner join lateral (
        select y, case
            -- There's 3 key things going on in this case statement:

            -- some digits are delimited by start-of-line rather than a preceding symbol

            -- because I apply +1 to the found index for symbol delimited cases, I return 0
            -- in any start-of-line cases so that the +1 in the select cast above doesn't care

            -- i could have created an isdigit function that returns boolean if the parameter
            -- can be cast to int but i just decided to test 'IN (..., ..., ...)' instead. This
            -- covers the case when there's a numeric start of line, e.g.  '123.....' vs a
            -- symbol start of line '.123....' / '&123....' with a number to be parsed
            when x = 1 and n in ('0', '1', '2', '3', '4', '5', '6', '7', '8', '9') then 0
            else x
        end as x
        from cols

        -- x <= a.x rather than < because we handle the numeric start of line case rather than
        -- using an extra union query or some other approach
        where x <= a.x and y = a.y and (x = 1 or n not in ('0', '1', '2', '3', '4', '5', '6', '7', '8', '9'))

        -- group to get rid of the duplication caused by e.g. in the '67' case above, both 6 and
        -- 7 will generate a starting x,y of 2,8 because the starting digit is the 6 for both
        group by y, x, n

        -- order and limit to select the right-most symbol (or start of line) that's left of 'adjacent'
        order by x desc
        limit 1
    ) c on true
    group by c.y, c.x
    order by c.y, c.x
),

-- find the location of the last digit in each adjacent:
--  r | y | x
-- -----------
--  0 | 2 | 9  -- the 7 of 67 is at y=2, x=9 in this example
rightbounds as (
    select row_number() over () as r, c.y, c.x::int - 1 as x
    from adjacent a inner join lateral (
        select y, case
            when x = 140 and n in ('0', '1', '2', '3', '4', '5', '6', '7', '8', '9') then 141
            else x
        end as x
        from cols
        where x >= a.x and y = a.y and (x = 140 or n not in ('0', '1', '2', '3', '4', '5', '6', '7', '8', '9'))
        group by y, x, n
        order by x asc
        limit 1
    ) c on true
    group by c.y, c.x
    order by c.y, c.x
),
-- assert (select count(*) from leftbounds) = (select count(*) from rightbounds)

-- List the starting x,y and calculate the length of each left- & right- bounded pair
offsets as (
    select l.y, l.x, (r.x - l.x) + 1 as len
    from leftbounds l full outer join rightbounds r
    on l.r = r.r
),

-- Use the starting x,y and length to parse out the part numbers
part_numbers as (
    select o.*, substring (line from o.x for o.len)::int as part
    from offsets o, enumerated e
    where o.y = e.y
)

-- Calculate answer for star 1
select sum(part) from part_numbers;
