## Spiral traversal problem

1   2   3   4
5   6   7   8
9   10  11  12
13  14  15  16

BEGIN

when y++ and x++ | max_y: 3 and max_x: 3

sense: right
00  01  02  03  [y==3; stop] 
start: x = 0 | y = 0 
step: x | y++ 
final: x = 0 (x) | y = 3

stop when y == 3

sense: down
13  23  33  [x==3; stop] 
start: x = 1 (x_last(0) + 1) | y = y_last(3) # do in prev step
step: x++ | y 
final: x = 3 | y = 3 (y)

stop when x == 3

when y-- and x-- | min_y: 0 and min_x: 1

sense: left
32  31  30  [y==0; stop]
start: x = x_last(3) | y = 2 (y_last(3) - 1) # do in prev step
step: x | y--
final: x = 3 (x) | y = 0

stop when y == 0

sense: up
20  10  [x==1; stop]
start: x = 2 (x_last(3) - 1) | y = y_last(0) # do in prev step
step: x-- | y
final: x = 1 | y = 0 (y)

stop when x == 1

when y++ and x++ | max_y: 2 and max_x: 2

sense: right
11  12 [y==2; stop]
start: x = 1 (x_last(1)) | y = 1 (y_last(0) + 1) # do in prev step
step: x | y++
final: x = 1 (x) | y = 2

stop when y == 2

22  [x==2; stop]
start: x = 2 (x_last(1) + 1) | y = 2 (y_last(2))
step: x++ | y
final: x = 2 y = 2 (y)

stop when x == 2

when y-- and x-- | min_y: 1 and min_x: 2

21  [y==1; stop]
start: x = 2 x_last(2) | y = 1 (y_last(2) - 1)
step: x | y--
final: x = 2 (x) y = 1

stop when y == 1

here min_x == max_x or min_y == max_y  [break main loop]

END
