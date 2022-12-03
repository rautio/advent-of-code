package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strings"
)

func commonRune(compOne string, compTwo string) rune {
	for _, c := range compOne {
		if (strings.ContainsRune(compTwo, c)) {
			return c
		}
	}
	return rune(0);
}

func commonRunes(strOne string, strTwo string) string {
	result := ""
	for _, c := range strOne {
		if (strings.ContainsRune(strTwo, c)) {
			result += string(c)
		}
	}
	return result

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
	priority := "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
	prevLine := ""
	prevMatches := ""
	for scanner.Scan() {
		line := scanner.Text()
		l := len(line)
		c := commonRune(line[0:l/2],line[l/2:l])
		answerOne += strings.IndexRune(priority, c) + 1
		if (prevLine == "") {
			// First line
			prevLine = line
		} else if (prevMatches == "") {
			// Second line
			prevMatches = commonRunes(prevLine, line)
		} else {
			// Third line, reset all
			e := commonRune(prevMatches, line)
			answerTwo += strings.IndexRune(priority, e) + 1
			prevLine = ""
			prevMatches = ""
		}
	}
	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}
	fmt.Printf("Part One Answer: %d\n", answerOne)
	fmt.Printf("Part Two Answer: %d\n", answerTwo)
}