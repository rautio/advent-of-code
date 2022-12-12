package main

import (
	"fmt"
	"io/ioutil"
	"math"
	"strings"
)

type Point struct {
	x, y int
}

func parseGrid(lines []string) (map[Point]int, Point, Point){
	grid := make(map[Point]int)
	var start Point
	var end Point
	alphabet := "abcdefghijklmnopqrstuvwxyz"
	for i, line := range lines {
		for j, c := range strings.Split(line,"") {
			if (c == "S") {
				start = Point{j,i}
				grid[start] = strings.Index(alphabet, "a")
			} else if (c == "E") {
				end = Point{j,i}
				grid[end] = strings.Index(alphabet, "z")
			} else {
				grid[Point{j,i}] = strings.Index(alphabet, c)
			}
		}
	}
	return grid, start, end
}

func delta(x int, y int)int {
	return int(math.Abs(float64(x - y)))
}

func shortestPath(grid map[Point]int, start Point, end Point) int {
	distances := make(map[Point]int)
	distances[end] = 0
	queue := []Point{end}
	for (len(queue) > 0) {
		curPt := queue[0]
		queue = queue[1:]
		up := Point{curPt.x-1, curPt.y}
		down := Point{curPt.x+1, curPt.y}
		left := Point{curPt.x, curPt.y-1}
		right := Point{curPt.x, curPt.y+1}
		possiblePaths := []Point{up, down, left, right}
		for _, path := range possiblePaths {
			_, okPath := grid[path]
			_, seenPath := distances[path]
			if okPath && !seenPath && grid[curPt] <= grid[path] + 1 {
				distances[path] = distances[curPt] + 1
				queue = append(queue, path)
			}
		}
	}
	return distances[start]
}
func shortestPathLowStart(grid map[Point]int, end Point) int {
	distances := make(map[Point]int)
	distances[end] = 0
	queue := []Point{end}
	var shortest *Point
	for (len(queue) > 0) {
		curPt := queue[0]
		queue = queue[1:]
		up := Point{curPt.x-1, curPt.y}
		down := Point{curPt.x+1, curPt.y}
		left := Point{curPt.x, curPt.y-1}
		right := Point{curPt.x, curPt.y+1}
		possiblePaths := []Point{up, down, left, right}
		for _, path := range possiblePaths {
			_, okPath := grid[path]
			_, seenPath := distances[path]
			if okPath && !seenPath && grid[curPt] <= grid[path] + 1 {
				distances[path] = distances[curPt] + 1
				// First 'a' we find is the shortest
				if grid[path] == 0 && shortest == nil {
					shortest = &path
				}
				queue = append(queue, path)
			}
		}
	}
	return distances[*shortest]
}

func main() {
	input, err := ioutil.ReadFile("input.txt")
	if err != nil {
		panic(err)
	}
	lines := strings.Split(string(input), "\n")
	// First, parse the input
	grid, start, end := parseGrid(lines)
	fmt.Printf("Part One Answer: %d\n", shortestPath(grid, start, end))
	fmt.Printf("Part Two Answer: %d\n", shortestPathLowStart(grid, end))
}
