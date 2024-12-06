


// Grid 
// constants; positions, adjascent positions, bounds, flags for branching & invariants
// Grid will contain GridCells
// Each GridCell is a line 
// GridCell is a vector of Grid Tiles
// Each Grid tile contains its own constants x,y,adjascent coords, Tile Type(Empty, Guard, Wall, e.t.c..)
/* 
....#.....     <- Grid Cell
....+---+[#]<- Grid Tile 
....|...|.
..#.|...|.
..+-+-+#|.
..|.|.|.|.
.#+-^-+-+.
.+----++#.
#+----++..
......#O..
*/