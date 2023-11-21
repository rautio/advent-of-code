package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"regexp"
	"strconv"
	"strings"
)

func parseBoxes(rawBoxes []string) []string {
	boxes := []string{"", "", "", "", "", "", "", "", ""}
	for i := len(rawBoxes) - 1; i >= 0; i-- {
		s := 1 
		line := rawBoxes[i]
		for s < len(line) {
			j := (s-1)/4
			if len(string(line[s])) != 0 {
				boxes[j] = boxes[j] + string(line[s])
			}
			s += 4
		}
	}
	return boxes
}

func partOneMove(count int, from int, to int, boxes []string) []string {
	for i := 0; i < count; i++ {
		str := strings.TrimSpace(boxes[from])
		c := str[len(str)-1]
		boxes[from] = str[:len(str)-1]
		boxes[to] = strings.TrimSpace(boxes[to]) + string(c)
	}
	return boxes
}

func partTwoMove(count int, from int, to int, boxes []string) []string {
	str := strings.TrimSpace(boxes[from])
	cs := str[len(str)-count:]
	boxes[from] = str[:len(str)-count]
	boxes[to] = strings.TrimSpace(boxes[to]) + string(cs)
	return boxes
}

func main() {
	f, err := os.Open("input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()
	scanner := bufio.NewScanner(f)
	boxSetup := false
	var rawBoxes []string
	var partOneBoxes []string
	var partTwoBoxes []string
	for scanner.Scan() {
		line := scanner.Text()
		if (line == "") {
			// End of box setup
			boxSetup = true;
			partOneBoxes = parseBoxes(rawBoxes[:len(rawBoxes)-1])
			partTwoBoxes = parseBoxes(rawBoxes[:len(rawBoxes)-1])
		} else if (!boxSetup) {
			// Setting up
			rawBoxes = append(rawBoxes, line);
		} else {
			// Moving boxes
			allNums := regexp.MustCompile("[0-9]+")
			nums := allNums.FindAllString(line, -1)
			count, _ := strconv.Atoi(nums[0])
			fromRaw, _ := strconv.Atoi(nums[1])
			from := fromRaw - 1
			toRaw, _ := strconv.Atoi(nums[2])
			to := toRaw - 1
			partOneBoxes = partOneMove(count, from, to, partOneBoxes)
			partTwoBoxes = partTwoMove(count, from, to, partTwoBoxes)
		}
	}
	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}
	answerOne := ""
	answerTwo := ""
	for _, s := range partOneBoxes {
		str := strings.TrimSpace((s))
		l := len(str)
		answerOne += string(str[l-1])
	}
	for _, s := range partTwoBoxes {
		str := strings.TrimSpace((s))
		l := len(str)
		answerTwo += string(str[l-1])
	}
	fmt.Printf("Part One Answer: %s\n", answerOne)
	fmt.Printf("Part Two Answer: %s\n", answerTwo)
}