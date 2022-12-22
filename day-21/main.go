package main

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

type Riddle struct {
	from, mon1, mon2, op string
}

func operation(n1 int, n2 int, op string) int {
	switch op {
	case "*":
		return n1 * n2
	case "+":
		return n1 + n2
	case "/":
		return n1 / n2
	case "-":
		return n1 - n2
	}
	return -1
}
func reverseOperation(n1 int, n2 int, op string) int {
	switch op {
	case "*":
		return n1 / n2
	case "+":
		return n1 - n2
	case "/":
		return n1 * n2
	case "-":
		return n1 + n2
	}
	return -1
}

func solveRoot(input []string) int {
	monkeyNumbers := make(map[string]int)
	riddles := []Riddle{}
	for _, line := range input{
		mon := line[:4]
		if len(line) == 17 {
			mon1 := line[6:10]
			mon2 := line[13:17]
			op := line[11:12]
			_, isMon1 := monkeyNumbers[mon1]
			_, isMon2 := monkeyNumbers[mon2]
			if isMon1 && isMon2 {
				monkeyNumbers[mon] = operation(monkeyNumbers[mon1], monkeyNumbers[mon2], op)
			} else {
				r := Riddle{mon, mon1, mon2, op}
				riddles = append(riddles, r)
			}
		} else {
			n, _ := strconv.Atoi(line[6:])
			monkeyNumbers[mon] = n
		}
		if _, ok := monkeyNumbers[mon]; ok {
			solved := []string{mon}
			for len(solved) > 0 {
				cur := solved[0]
				solved = solved[1:]
				for _, r := range riddles {
					_, isMon1 := monkeyNumbers[r.mon1]
					_, isMon2 := monkeyNumbers[r.mon2]
					if (r.mon1 == cur && isMon2) || (r.mon2 == cur && isMon1) {
						solved = append(solved, r.from)
						monkeyNumbers[r.from] = operation(monkeyNumbers[r.mon1], monkeyNumbers[r.mon2], r.op)
					}
				}
			}
		}
		// Check if riddles can be solved by knowing mon
		if _, ok := monkeyNumbers["root"]; ok {
			return monkeyNumbers["root"]
		}
	}
	return monkeyNumbers["root"]
}

func solveVal(m string, mn map[string]int, riddles map[string]Riddle) int {
	if m == "humn" {
		return -1
	}
	if _, ok := mn[m]; ok {
		return mn[m]
	}
	r := riddles[m]
	s1 := solveVal(r.mon1, mn, riddles)
	s2 := solveVal(r.mon2, mn, riddles)
	if s1 == -1 {
		return s1
	}
	if s2 == -1 {
		return s2
	}
	mn[m] = operation(s1, s2, r.op)
	return mn[m]
}

func solveHumn(target int, m string, mn map[string]int, riddles map[string]Riddle) int {
	if m == "humn" {
		return target
	}
	r := riddles[m]
	s1 := solveVal(r.mon1, mn, riddles)
	s2 := solveVal(r.mon2, mn, riddles)
	if s1 == -1 {
		mn[r.mon2] = s2
		newTarget := reverseOperation(target, s2, r.op)
		return solveHumn(newTarget, r.mon1, mn, riddles)
	}
	if s2 == -1 {
		mn[r.mon1] = s1
		newTarget := reverseOperation(target, s1, r.op)
		if (r.op == "-") {
			newTarget = operation(s1, target, r.op)
		}
		return solveHumn(newTarget, r.mon2, mn, riddles)
	}
	return -1
}

func solveHuman(input []string) int {
	monkeyNumbers := make(map[string]int)
	riddles := map[string]Riddle{}
	for _, line := range input{
		mon := line[:4]
		if len(line) == 17 {
			mon1 := line[6:10]
			mon2 := line[13:17]
			op := line[11:12]
			r := Riddle{mon, mon1, mon2, op}
			riddles[mon] = r
		} else {
			n, _ := strconv.Atoi(line[6:])
			monkeyNumbers[mon] = n
		}
	}
	humanVal := -1
	s1 :=  solveVal(riddles["root"].mon1, monkeyNumbers, riddles)
	s2 :=  solveVal(riddles["root"].mon2, monkeyNumbers, riddles)
	if (s1 == -1) {
		humanVal = solveHumn(s2, riddles["root"].mon1, monkeyNumbers, riddles)
	}
	if (s2 == -1) {
		humanVal = solveHumn(s1, riddles["root"].mon2, monkeyNumbers, riddles)
	}
	return humanVal
}

func main() {
	input, err := ioutil.ReadFile("input.txt")
	if err != nil {
		panic(err)
	}
	fmt.Printf("Part One Answer: %d\n", solveRoot(strings.Split(string(input), "\n")))
	fmt.Printf("Part Two Answer: %d\n", solveHuman(strings.Split(string(input), "\n")))
}