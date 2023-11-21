package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
)

func main() {
	f, err := os.Open("input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()
	scanner := bufio.NewScanner(f)
	grid := make([][]int,0)
	for scanner.Scan() {
		line := scanner.Text()
		tmp := make([]int,0)
		for _, c := range line {
			i, _ := strconv.Atoi(string(c))
			tmp = append(tmp, i)
		}
		grid = append(grid, tmp)
	}
	countVisible := 2 * (len(grid) + len(grid[0])) - 4
	maxScore := 0
	for i := 1; i < len(grid)-1; i++ {
		for j :=1; j < len(grid[i])-1; j++ {
			// PART 1
			// left
			leftMax := 0
			for k := 0; k < j; k++ {
				if grid[i][k] >= leftMax {
					leftMax = grid[i][k]
				}
			}
			// right
			rightMax := 0
			for k := len(grid[i])-1; k > j; k-- {
				if grid[i][k] >= rightMax {
					rightMax = grid[i][k]
				}
			}
			// top
			topMax := 0
			for k := 0; k < i; k++ {
				if grid[k][j] >= topMax {
					topMax = grid[k][j]
				}
			}
			// bottom
			bottomMax := 0
			for k := len(grid)-1; k > i; k-- {
				if grid[k][j] >= bottomMax {
					bottomMax = grid[k][j]
				}
			}
			current := grid[i][j]
			if (leftMax < current || rightMax < current || topMax < current || bottomMax < current) {
				countVisible += 1
			}
			// PART 2
			// left
			leftScore := 0
			for k := j-1; k >= 0; k-- {
				leftScore += 1
				if (grid[i][k] >= grid[i][j]){
					break
				}
			}
			// right
			rightScore := 0
			for k := j+1; k < len(grid[i]); k++ {
				rightScore += 1
				if (grid[i][k] >= grid[i][j]){
					break
				}
			}
			// top
			topScore := 0
			for k := i-1; k >= 0; k-- {
				topScore += 1
				if (grid[k][j] >= grid[i][j]){
					break
				}
			}
			// bottom
			bottomScore := 0
			for k := i+1; k < len(grid); k++ {
				bottomScore += 1
				if (grid[k][j] >= grid[i][j]){
					break
				}
			}
			newScore := leftScore * rightScore * topScore * bottomScore
			if newScore > maxScore {
				maxScore = newScore
			}
		}
	}
	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}
	fmt.Printf("Part One Answer: %d\n", countVisible)
	fmt.Printf("Part Two Answer: %d\n", maxScore)
}