package main

import (
	"encoding/json"
	"fmt"
	"io/ioutil"
	"sort"
	"strings"
)


func equal(first, second any) int {
	a, okA := first.(float64)
	b, okB := second.(float64)
	if okA && okB {
		return int(a) - int(b)
	}

	var fList []any
	var sList []any

	switch first.(type) {
	case []any, []float64:
		fList = first.([]any)
	case float64:
		fList = []any{first}
	}

	switch second.(type) {
	case []any, []float64:
		sList = second.([]any)
	case float64:
		sList = []any{second}
	}

	for i := range fList {
		if len(sList) <= i {
			return 1
		}
		if r := equal(fList[i], sList[i]); r != 0 {
			return r
		}
	}
	if len(sList) == len(fList) {
		return 0
	}
	return -1
}

func main() {
	input, err := ioutil.ReadFile("input.txt")	
	if err != nil {
		panic(err)
	}

	var partOneAnswer int
	var all []any

	for idx, pairs := range strings.Split(string(input), "\n\n") {
		pair := strings.Split(pairs, "\n")

		var first any
		var second any
		if err := json.Unmarshal([]byte(pair[0]), &first); err != nil {
			panic(err)
		}
		if err := json.Unmarshal([]byte(pair[1]), &second); err != nil {
			panic(err)
		}
		all = append(all, first, second)

		if equal(first, second) <= 0 {
			partOneAnswer += idx + 1
		}
	}
	fmt.Printf("Part One Answer: %d\n", partOneAnswer)
	
	var add1 any
	var add2 any
	json.Unmarshal([]byte("[[2]]"), &add1)
	json.Unmarshal([]byte("[[6]]"), &add2)
	all = append(all, add1, add2)

	sort.Slice(all, func(i, j int) bool {
		return equal(all[i], all[j]) < 0
	})

	var partTwoAnswer int = 1
	for k, v := range all {
		str, _ := json.Marshal(v)
		if string(str) == "[[2]]" || string(str) == "[[6]]" {
			partTwoAnswer *= k + 1
		}
	}
	fmt.Printf("Part Two Answer: %d\n", partTwoAnswer)
}
