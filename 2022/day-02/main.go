package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strings"
)

func partOne(playerOne string, playerTwo string) int {
	m := make(map[string]int)
	m["X"] = 1
	m["Y"] = 2
	m["Z"] = 3
	r := make(map[string]string)
	r["X"] = "A"
	r["Y"] = "B"
	r["Z"] = "C"
	roundScore := m[playerTwo]
	if (playerOne == r[playerTwo]){
		roundScore += 3
	} else if ((playerOne == "A" && playerTwo == "Y") || (playerOne == "B" && playerTwo == "Z") || (playerOne == "C" && playerTwo == "X")) {
		roundScore += 6
	}
	return roundScore
}


func partTwo(playerOne string, outcome string) int {
	m := make(map[string]int)
	m["A"] = 1
	m["B"] = 2
	m["C"] = 3
	win := make(map[string]string)
	win["A"] = "B"
	win["B"] = "C"
	win["C"] = "A"
	lose := make(map[string]string, len(win))
	for k, v := range win {
			lose[v] = k
	}
	roundScore := 0
	if (outcome == "Y"){
		roundScore += 3
		roundScore += m[playerOne]
	} else if (outcome == "Z") {
		roundScore += 6
		roundScore += m[win[playerOne]]
	} else {
		roundScore += m[lose[playerOne]]
	}
	return roundScore
}

func main() {
	f, err := os.Open("input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()
	scanner := bufio.NewScanner(f)
	answerOne := 0
	answerTwo := 0
	for scanner.Scan() {
		round := strings.Fields(scanner.Text());
		answerOne += partOne(round[0], round[1])
		answerTwo += partTwo(round[0], round[1])
	}
	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}
	fmt.Printf("Part One Score: %d\n", answerOne)
	fmt.Printf("Part Two Score: %d\n", answerTwo)
}