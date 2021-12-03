/
  Advent of Code 2021 Solutions in q
  Craig J Perry
\

/ day 1, part 1 = 1393i
d1p1:sum 1_ (>':) "I" $ read0 `:day1.txt

/ day 1, part 2 = 1359i
d1p2: sum 3_ (>':) 3 msum "I" $ read0 `:day1.txt

/ day 2, part 1 = 1762050i
t: select sum dist by dir from flip `dir`dist!("SI";" ") 0:`day2.txt
d2p1: t[`forward;`dist] * t[`down;`dist] - t[`up;`dist]

