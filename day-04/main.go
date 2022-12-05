package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

func isFullyContained(str1 string, str2 string) bool {
	// Equal
	if str1 == str2 { return true }
	str1Splits := strings.Split(str1, "-")
	oneLower, _ := strconv.Atoi(str1Splits[0])
	oneUpper, _ := strconv.Atoi(str1Splits[1])
	str2Splits := strings.Split(str2, "-")
	twoLower, _ := strconv.Atoi(str2Splits[0])
	twoUpper, _ := strconv.Atoi(str2Splits[1])
	// str1 contained
	if (oneLower >= twoLower && oneUpper <= twoUpper) {
		return true
	}
	// str2 contained
	if (twoLower >= oneLower && twoUpper <= oneUpper) {
		return true
	}
	// Neither contained
	return false
}

func isOverlap(str1 string, str2 string) bool {
	str1Splits := strings.Split(str1, "-")
	oneLower, _ := strconv.Atoi(str1Splits[0])
	oneUpper, _ := strconv.Atoi(str1Splits[1])
	str2Splits := strings.Split(str2, "-")
	twoLower, _ := strconv.Atoi(str2Splits[0])
	twoUpper, _ := strconv.Atoi(str2Splits[1])
	// No overlap
	if (oneUpper < twoLower || twoUpper < oneLower) {
		return false
	}
	// Has to be some overlap
	return true

}

func main() {
	f, err := os.Open("input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()
	scanner := bufio.NewScanner(f)
	countContained := 0
	countOverlap := 0
	for scanner.Scan() {
		line := scanner.Text()
		splits := strings.Split(line, ",")
		if isFullyContained(splits[0], splits[1]) {
			countContained += 1
		}
		if isOverlap(splits[0], splits[1]) {
			countOverlap += 1
		}
	}
	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}
	fmt.Printf("Part One Answer: %d\n", countContained)
	fmt.Printf("Part Two Answer: %d\n", countOverlap)
}