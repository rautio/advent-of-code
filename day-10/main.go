package main

import (
	"bufio"
	"fmt"
	"log"
	"math"
	"os"
	"strconv"
	"strings"
)


func main() {
	f, err := os.Open("input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()
	x := 1
	cycles := 1
	answerOne := 0
	crt := ""
	scanner := bufio.NewScanner(f)
	for scanner.Scan() {
		pos := cycles - 1
		line := scanner.Text()
		split := strings.Fields((line))
		cmd := split[0]
		if (math.Abs(float64(x-pos%40)) <= 1) {
			// Draw #
			crt += "#"
		} else {
			// Draw .
			crt += "."
		}
		if (cycles % 40 == 0) {
			crt += "\n"
		}
		if (cycles % 40 == 20){
			answerOne += cycles * x
		}
		if (cmd == "addx") {
			newX, _ := strconv.Atoi(split[1])
			cycles++
			pos++
			if (cycles % 40 == 20){
				answerOne += cycles * x
			}
			if (math.Abs(float64(x-pos%40)) <= 1) {
				// Draw #
				crt += "#"
			} else {
				// Draw .
				crt += "."
			}
			if (cycles % 40 == 0) {
				crt += "\n"
			}
			x += newX
		}
		cycles++;
	}
	fmt.Printf("Part One Answer: %d\n", answerOne)
	fmt.Printf("Part Two Answer:\n%s\n", crt)
}