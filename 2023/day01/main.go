package main

import (
	"fmt"
	"io/ioutil"
	"strings"
)

func find_digit(str) {

}

func main() {
	// Part 1
	input, err := ioutil.ReadFile("input.txt")
	if err != nil {
		panic(err)
	}
	lines := strings.Split(string(input), "\n")
	partOneSum := 0
	for _, line := range lines {
		string := ""
		string += find_digit(line)
	}
	fmt.Printf("Part One Answer: %d\n", partOneSum))
	// fmt.Printf("Part Two Answer: %d\n", shortestPathLowStart(grid, end))
}
