package main

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

type Coord struct {
	x,y,z int
}

func parseCubes(input []string) ([]Coord, Coord, Coord) {
	cubes := []Coord{}
	maxMinSet := false
	max := Coord{}
	min := Coord{}
	for _, line := range input {
		nums := strings.Split(strings.TrimSpace(line), ",")
		x, _ := strconv.Atoi(nums[0])
		y, _ := strconv.Atoi(nums[1])
		z, _ := strconv.Atoi(nums[2])
		coord := Coord{x,y,z}
		if (!maxMinSet) {
			max = coord
			min = coord
			maxMinSet = true
		} else {
			if (coord.x > max.x) {
				max.x = coord.x
			}
			if (coord.y > max.y) {
				max.y = coord.y
			}
			if (coord.z > max.z) {
				max.z = coord.z
			}
			if (coord.x < min.x) {
				min.x = coord.x
			}
			if (coord.y < min.y) {
				min.y = coord.y
			}
			if (coord.z < min.z) {
				min.z = coord.z
			}
		}
		cubes = append(cubes, coord)
	}
	return cubes, max, min
}

func calcSurfaceArea(cubes []Coord) int {
	total := 0
	taken := map[Coord]bool{}
	for _, coord := range cubes {
		newTotal := 6
		// 6 sides to a cube
		neighbors := []Coord{{0,0,1}, {0,1,0}, {1,0,0}, {0,0,-1}, {0,-1,0}, {-1,0,0}}
		// If neighbor exists, adjust totals
		for _, n := range neighbors {
			neighbor := Coord{coord.x+n.x, coord.y+n.y, coord.z+n.z}
			_, exists := taken[neighbor]
			if exists {
				newTotal--;
				total--;
			}
		}
		taken[coord] = true
		total += newTotal
	}
	return total

}

func pathExists(start Coord, end Coord, cubes []Coord) bool {

	return false
}

func calcInteriorArea(cubes []Coord, max Coord, min Coord) int {
	total := 0
	cubeMap := map[Coord]bool{}
	// Map all coordinates reachable from the outside.
	// The interior area will be the negative space of the outside air + cubes
	outsideAir := map[Coord]bool{}
	for _, c := range cubes {
		cubeMap[c] = true
	}
	start := Coord{min.x, min.y, min.z}
	queue := []Coord{start}
	neighbors := []Coord{{0,0,1}, {0,1,0}, {1,0,0}, {0,0,-1}, {0,-1,0}, {-1,0,0}}
	for len(queue) > 0 {
		coord := queue[0]
		queue = queue[1:]
		_, alredyChecked := outsideAir[coord]
		if !alredyChecked {
			outsideAir[coord] = true
			// Move to neighbors
			for _, n := range neighbors {
				neighbor := Coord{coord.x+n.x, coord.y+n.y, coord.z+n.z}
				_, alreadyChecked := outsideAir[neighbor]
				_, isCube := cubeMap[neighbor]
				if (!isCube && !alreadyChecked && neighbor.x >= min.x && neighbor.x <= max.x && neighbor.y >= min.y && neighbor.y <= max.y && neighbor.z >= min.z && neighbor.z <= max.z){
					queue = append(queue, neighbor)
				}
			}
		}
	}
	// Iterate through all coordinates, figure out which ones are interior, look for neighbors that are occupied
	for x := min.x+1; x < max.x; x++ {
		for y := min.y+1; y < max.y; y++ {
			for z := min.z+1; z < max.z; z++ {
				coord := Coord{x,y,z}
				_, isOutside := outsideAir[coord]
				_, isCube := cubeMap[coord]
				// If its not an outside air coordinate and not a cube
				// it must be an internal air pocket
				if !isOutside && !isCube {
					// Its an empty coordinate we haven't seen yet
					// so it has to be on the inside. Check its neighbors for valid cubes
					for _, n := range neighbors {
						neighbor := Coord{coord.x + n.x, coord.y+n.y, coord.z+n.z}
						_, isNeighborCube := cubeMap[neighbor]
						if isNeighborCube {
							// Each neighboring cube will show 1 side
							total++
						}
					}
				}
			}
		}
	}
	return total

}

func main() {
	input, err := ioutil.ReadFile("input.txt")
	if err != nil {
		panic(err)
	}
	cubes, max, min := parseCubes(strings.Split(string(input), "\n"))
	surfaceArea := calcSurfaceArea(cubes)
	interiorArea := calcInteriorArea(cubes, max, min)
	fmt.Printf("Part One Answer: %d\n", surfaceArea)
	fmt.Printf("Part Two Answer: %d\n", surfaceArea - interiorArea)
}
// Part 1: 4192
// Part 2: 2520