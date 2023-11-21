package main

import (
	"fmt"
	"io/ioutil"
	"math"
	"strconv"
	"strings"
)

type Point struct {
	x int
	y int
}

func abs(i int) float64 {
	return math.Abs(float64(i))
}

func sign(x int) int {
	if x < 0 {
		return -1
	}
	if x == 0 {
		return 0
	}
	return 1
}

func followKnots(head Point, tail Point) Point {
	deltaX := head.x - tail.x
	deltaY := head.y - tail.y
	if abs(deltaX) > 1 || abs(deltaY) > 1 {
		return Point{x: tail.x + sign(deltaX), y: tail.y + sign(deltaY)}
	}
	return tail
}

func run(input []byte, knotCount int) int {
	// Initialize
	seen := make(map[Point]bool)
	rope := make([]Point, knotCount)
	// Starting point
	seen[rope[0]] = true
	for _, line := range strings.Split(string(input), "\n") {
		split := strings.Fields(line)
		dir := split[0]
		count, _ := strconv.Atoi(split[1])
		for i := 0; i < count; i++ {
			// Move the Head
			switch dir {
				case "U":
					rope[0].y = rope[0].y + 1
				case "D":
					rope[0].y = rope[0].y - 1
				case "R":
					rope[0].x = rope[0].x + 1
				case "L":
					rope[0].x = rope[0].x - 1
			}
			// Move the following knots
			for i := 1; i < len(rope); i++ {
				rope[i] = followKnots(rope[i-1], rope[i])
				// Track where the last knot has been
				if (i == len(rope) -1 ){
					seen[rope[i]] = true
				}
			}
		}
	}
	return len(seen)
}

func main() {
	input, err := ioutil.ReadFile("input.txt")
	if err != nil {
		panic(err)
	}
	fmt.Printf("Part One Answer: %d\n", run(input, 2))
	fmt.Printf("Part Two Answer: %d\n", run(input, 10))
}