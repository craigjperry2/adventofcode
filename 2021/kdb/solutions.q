/
  Advent of Code 2021 Solutions in q
  Craig J Perry
\

/ day 1, part 1 = 1393i
d1p1:sum 1_ (>':) "I" $ read0 `:day1.txt

/ day 1, part 2 = 1359i
d1p2:sum 3_ (>':) 3 msum "I" $ read0 `:day1.txt

/ day 2, part 1 = 1762050i
t: select sum dist by dir from flip `dir`dist!("SI";" ") 0:`day2.txt
d2p1:t[`forward;`dist] * t[`down;`dist] - t[`up;`dist]

/ day 2, part 2 = skipped

/ day 3, part 1 = 1025636
t:flip (`a`b`c`d`e`f`g`h`i`j`k`l)!("IIIIIIIIIIII";1 1 1 1 1 1 1 1 1 1 1 1) 0:`day3.txt
g:floor 0.5+avg t
e:1-g
d3p1:2 sv value g * 2 sv value e

/ day 3, part 2 = skipped

/ day 4, part 1 = skipped
n:"I" $ "," vs first 1#read0 `day4.txt
b:flip (`a`b`c`d`e)!("IIIII";2 3 3 3 3)0:t where not""~/:t:2_read0`:day4.txt

