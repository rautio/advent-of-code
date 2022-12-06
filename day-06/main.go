package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strings"
)

func isUnique(str string) bool {
	res := ""
	for _, c := range str {
		s := string(c)
		if strings.Contains(res, s) {
			return false
		} else {
			res += s
		}
	}
	return true
}

func main() {
	f, err := os.Open("input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()
	scanner := bufio.NewScanner(f)
	answerOne := -1
	answerTwo := -1
	for scanner.Scan() {
		line := scanner.Text()
		for i := 4; i < len(line); i++ {
			s := line[i-4:i]
			if isUnique(s) {
				answerOne = i
				break
			}
		}
		for i := 14; i < len(line); i++ {
			s := line[i-14:i]
			if isUnique(s) {
				answerTwo = i
				break
			}
		}
	}
	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}
	fmt.Printf("Part One Answer: %d\n", answerOne)
	fmt.Printf("Part Two Answer: %d\n", answerTwo)
}