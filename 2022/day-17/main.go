package main

import (
	"fmt"
	"io/ioutil"
)


type Coord struct {
	x, y int
}

type Shape struct {
	coords []Coord
	left Coord
	bottom Coord
}

var rocks = [][]Coord{
	// ####
	{{0,0},{1,0},{2,0},{3,0}},
	//  # {1,2}
	// ### {0,1} {1,1} {2,1}
	//  # {1,0}
	{{1,0},{0,1},{1,1},{2,1},{1,2}},
	//   #
	//   #
	// ###
	{{0,0},{1,0},{2,0},{2,1},{2,2}},
	// #
	// #
	// #
	// #
	{{0,0},{0,1},{0,2},{0,3}},
	// ##
	// ##
	{{0,0},{0,1},{1,0},{1,1}},
}

func createRock(shape []Coord, top Coord) []Coord{
	rock := []Coord{}
	for _, c := range shape {
		rock = append(rock, Coord{c.x+top.x, c.y+top.y})
	}
	return rock
}

func add(c1 Coord, c2 Coord) Coord {
	return Coord{c1.x+c2.x, c1.y+c2.y}
}

var Down = Coord{0,-1}
var Left = Coord{-1,0}
var Right = Coord{1,0}

func printBoard(board map[Coord]bool, rock []Coord, tallestPt int, w int) {
	for i := tallestPt; i >= 0; i-- {
		line := "|"
		for j := 0; j < w; j++ {
			newS := "."
			for _, r := range rock {
				if r.x == j && r.y == i {
					newS = "@"
				}
			}
			if _, ok := board[Coord{j,i}]; ok {
				newS = "#"
			}
			line += newS
		}
		line += "|"
		fmt.Println(line)
	}
	fmt.Println("---------")
	fmt.Println("")
}

func moveRock(rock []Coord, dir Coord, board map[Coord]bool, width int) []Coord {
	newRock := []Coord{}
	for _, c := range rock {
		newC := add(c, dir)
		_, blocked := board[newC]
		// Can't move to new position:
		// Left - Right
		if (dir == Left || dir == Right) && (newC.x >= width || newC.x < 0 || blocked) {
			// Can't move, return existing rock
			return rock
		}
		// Down
		if dir == Down && (newC.y < 0 || blocked) {
			// Can't move, return existing rock - and we can't go down further
			return rock
		}
		newRock = append(newRock, newC)
	}
	// Able to move
	return newRock
}

func hitBottom(rock []Coord, board map[Coord]bool) bool {
	for _, c := range rock {
		newC := add(c, Down)
		_, blocked := board[newC]
		if newC.y < 0 || blocked {
			return true
		}
	}
	return false
}

func tallest(numRocks int, jets string) int {
	tallestPt := 4
	numRock := 0
	curTurn := 0
	w := 7
	board := map[Coord]bool{}
	curRock := createRock(rocks[0],Coord{2,tallestPt})
	for numRock < numRocks {
		curJetI := curTurn % len(jets)
		curJet := string(jets[curJetI])
		jetDir := Left
		if curJet == ">" {
			jetDir = Right
		}
		// Move rock from jets
		newRockJets := moveRock(curRock, jetDir, board, w)
		jetsAtBottom := hitBottom(newRockJets, board)
		if jetsAtBottom {
			// Need to create a new rock
			// Hit rock bottom
			for _, c := range newRockJets {
				board[c] = true
				if c.y > tallestPt {
					tallestPt = c.y
				}
			}
			numRock++
			curRock = createRock(rocks[numRock % len(rocks)], Coord{2,tallestPt+4})
		} else {
			// Continue moving down
			curRock = moveRock(newRockJets, Down, board, w)
		}
		curTurn++
	}
	// printBoard(board, []Coord{}, tallestPt, w)
	return tallestPt+1
}



func main() {
	input, err := ioutil.ReadFile("input.txt")
	if err != nil {
		panic(err)
	}
	jets := string(input)
	fmt.Printf("Part One Answer: %d\n", tallest(2022, jets))
}