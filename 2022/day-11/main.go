package main

import (
	"fmt"
	"io/ioutil"
	"math/big"
	"regexp"
	"sort"
	"strconv"
	"strings"
)

// (oldWorry) => newWorry
type Operation func(*big.Int) big.Int
// (worryLevel) => monkey to throw to
type Test func(*big.Int) int

type Monkey struct {
	items []*big.Int
	operation Operation
	test Test
	countInspected int
}

// Returns a score of the level of monkey business going on
func playMonkeyBusiness(monkeys []Monkey, lcm, rounds int, reliefFactor bool) string {
	// Play rounds
	for i := 0; i < rounds*len(monkeys); i++ {
		m := monkeys[i % len(monkeys)]
		for _, item := range m.items {
			// Inspect item
			monkeys[i % len(monkeys)].countInspected++
			worryLevel := m.operation(item)
			// Decrease worry level
			if reliefFactor {
				worryLevel = *worryLevel.Div(&worryLevel, big.NewInt(3))
			}
			// Using the least common multiple of the divisiblity numbers to keep 
			// the overall worry levels lower for performance
			worryLevel.Mod(&worryLevel, big.NewInt(int64(lcm)))
			// Throw the item
			nextMon := m.test(&worryLevel)
			monkeys[nextMon].items = append(monkeys[nextMon].items, &worryLevel)
			monkeys[i % len(monkeys)].items = monkeys[i % len(monkeys)].items[1:]
		}
	}
	counts := []int{}
	for _, m := range monkeys {
		counts = append(counts, m.countInspected)
	}
	sort.Ints(counts)
	sort.Sort(sort.Reverse(sort.IntSlice(counts)))
	return big.NewInt(int64(counts[0])).Mul(big.NewInt(int64(counts[0])), big.NewInt(int64(counts[1]))).String()
}

func parseMonkeys(lines []string) ([]Monkey, int) {
	monkeys := []Monkey{}
	lcm := 1
	for i := 0; i < len(lines); i+=7 {
		// Read starting items
		items := []int{}
		itemRe := regexp.MustCompile(`[\d][\d]*`);
		itemMatch := itemRe.FindAllString(lines[i+1], -1)
		for _, j := range itemMatch {
			item, _ := strconv.Atoi(j)
			items = append(items, item)
		}
		// Read operation
		opRe := regexp.MustCompile(`[^\s]* [\+|*] [^\s]*`);
		opMatch := opRe.FindAllString(lines[i+2], -1)
		opStrs := strings.Fields(opMatch[0])
		op := func(worryLevel *big.Int)big.Int {
			var vals []*big.Int
			for i := 0; i < len(opStrs); i+=2 {
				if opStrs[i] == "old" {
					vals = append(vals, worryLevel)
				} else {
					newVal, _ := strconv.Atoi(opStrs[i])
					vals = append(vals, big.NewInt(int64(newVal)))
				}
			}
			if opStrs[1] == "*" {
				return *vals[0].Mul(vals[0], vals[1])
			}
			return *vals[0].Add(vals[0], vals[1])
		}
		// Next monkey to throw to
		endNumRe := regexp.MustCompile(`\d+$`)
		div, _ := strconv.Atoi(endNumRe.FindString(lines[i+3]))
		lcm *= div
		trueMon, _ := strconv.Atoi(endNumRe.FindString(lines[i+4]))
		falseMon, _ := strconv.Atoi(endNumRe.FindString(lines[i+5]))
		test := func(worryLevel *big.Int) int {
			mod := big.NewInt(0)
			mod.Mod(worryLevel, big.NewInt(int64(div)))
			if mod.Cmp(big.NewInt(0)) == 0 {
				return trueMon
			}
			return falseMon
		}
		bigItems := []*big.Int{}
		for _, item := range items {
			bigItems = append(bigItems, big.NewInt(int64(item)))
		}
		monkeys = append(monkeys, Monkey{items: bigItems, operation: op, test: test})
	}
	return monkeys, lcm
}

func main() {
	input, err := ioutil.ReadFile("input.txt")
	if err != nil {
		panic(err)
	}
	lines := strings.Split(string(input), "\n")
	// First, parse the input
	monkeys1, lcm1 := parseMonkeys(lines)
	monkeys2, lcm2 := parseMonkeys(lines)
	// lcm = Least Commmon Multiple found between the "test" functions
	fmt.Printf("Part One Answer: %s\n", playMonkeyBusiness(monkeys1, lcm1, 20, true))
	fmt.Printf("Part Two Answer: %s\n", playMonkeyBusiness(monkeys2, lcm2, 10000, false))
}