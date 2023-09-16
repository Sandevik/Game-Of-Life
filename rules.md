# The Rules of Game Of Life;
## https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life

1. Any live cell with fewer than two live neighbours dies, as if by underpopulation
2. Any live cell with more than two or three live neighbours lives on to the next generation (next tick)
3. Any live cell with more than three neighbours dies, as if by overpopulation
4. Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction

Short:

1. Any live cell with two or three live neighbours survive
2. Any dead cell with exactly three live neighbours becomes a live cell
3. All other live cells die in the next generation, all dead cells stay dead.