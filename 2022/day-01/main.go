package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"sort"
	"strconv"

	"github.com/dustin/go-humanize"
)

func main() {
	f, err := os.Open("input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()
	scanner := bufio.NewScanner(f)
	maxCalories := 0
	elfWithMost := 0
	totalCalories := 0
	numElves := 0
	currentCalories := 0
	numSnacks := 0
	elves := make(map[int]int)

	for scanner.Scan() {
		if (scanner.Text() == "") {
			if (currentCalories > maxCalories) {
				maxCalories = currentCalories
				elfWithMost = numElves
			}
			elves[numElves] = currentCalories
			currentCalories = 0
			numElves += 1
		} else {
			calories, err := strconv.Atoi( scanner.Text())
			if err != nil {
				log.Fatal("Not a number!")
			}
			totalCalories += calories
			currentCalories += calories
			numSnacks += 1
		}
	}
	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}
	// Second part: Sum of calories from top 3 elves
	v := make([]int, 0, len(elves))
	for  _, value := range elves {
		v = append(v, value)
	}
	sort.Sort(sort.Reverse(sort.IntSlice(v)))
	
	fmt.Printf("Max Calories: %d\n", maxCalories)
	fmt.Printf("From Elf: %s\n", humanize.Ordinal(elfWithMost+1))
	fmt.Printf("Total Number of Elves: %d\n", numElves)
	fmt.Printf("Total Calories: %d\n", totalCalories)
	fmt.Printf("Total Number of snacks: %d\n", numSnacks)
	fmt.Printf("Calories from top 3 elves: %d\n", v[0] + v[1] + v[2])
}