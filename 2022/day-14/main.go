package main

import (
	"fmt"
	"io/ioutil"
	"math"
	"strconv"
	"strings"
)

type Point struct {
	x, y int
}

func notTaken(grid map[Point]string, pt Point) bool {
	if (grid[pt] != "#" && grid[pt] != "o"){
		return true
	}
	return false
}

func fallSand(grid map[Point]string, maxY int) int {
	sand := &Point{500,0}
	sandCount := 0
	for true {
		if notTaken(grid, Point{sand.x, sand.y+1}) {
			// Can move down
			sand.y++
		} else if notTaken(grid, Point{sand.x-1, sand.y+1}) {
			// Can move left
			sand.x--
			sand.y++

		} else if notTaken(grid, Point{sand.x+1, sand.y+1}){
			// Can move right
			sand.x++
			sand.y++

		} else {
			sandCount++
			if (sand.x == 500 && sand.y==0) {
				break;
			}
			// Stopped
			grid[Point{sand.x, sand.y}] = "o"
			sand = &Point{500,0}
		}
		if sand.y > maxY {
			break
		}
	}
	return sandCount

}

func main() {
	input, err := ioutil.ReadFile("input.txt")
	if err != nil {
		panic(err)
	}
	grid := make(map[Point]string)
	grid2 := make(map[Point]string)
	lines := strings.Split(string(input), "\n")
	maxY := 0
	maxX := 0
	minX := 1000
	for _, line := range lines {
		pts := strings.Split(line, " -> ")
		var prevPt *Point
		for _, pt := range pts {
			x,_ := strconv.Atoi(strings.Split(pt, ",")[0])
			y, _ := strconv.Atoi(strings.Split(pt, ",")[1])
			if y > maxY {
				maxY = y
			}
			if x > maxX {
				maxX = x
			}
			if x < minX {
				minX = x
			}
			if (prevPt != nil) {
				// Draw line
				if (prevPt.x != x) {
					sign := -1 * int(math.Abs(float64(prevPt.x - x))/float64(prevPt.x - x))
					for i := prevPt.x; i != x+sign; i+=sign {
						grid[Point{i,y}] = "#"
						grid2[Point{i,y}] = "#"
					}
				} else {
					sign := -1 * int(math.Abs(float64(prevPt.y - y))/float64(prevPt.y - y))
					for i := prevPt.y; i != y+sign; i+=sign {
						grid[Point{x,i}] = "#"
						grid2[Point{x,i}] = "#"
					}
				}
			}
			prevPt = &Point{x,y}
		}
	}
	for i:=0; i <= 1000; i ++{
		grid2[Point{i, maxY+2}] = "#"
	}
	fmt.Printf("Part One Answer: %d\n", fallSand(grid, maxY))
	fmt.Printf("Part Two Answer: %d\n", fallSand(grid2, maxY+2))
}