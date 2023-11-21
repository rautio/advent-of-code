package main

import (
	"fmt"
	"io/ioutil"
	"math"
	"regexp"
	"strconv"
	"strings"
)

type Point struct {
	x,y int
}

type Pair struct {
	sensor, beacon Point
}

func dist(pt1 Point, pt2 Point) int{
	dx := math.Abs(float64(pt2.x - pt1.x))
	dy := math.Abs(float64(pt2.y - pt1.y))
	return int(dx + dy)
}

func parse(input []string) ([]Pair, int, int){
	var pairs []Pair
	minX := 0
	maxX := 0
	for _, line := range input{
		numsReg := regexp.MustCompile(`=(-)?\d*`)
		var nums []int
		for _, s := range numsReg.FindAllString(line, -1){
			n, _ := strconv.Atoi(s[1:])
			nums = append(nums, n)
		}
		sensor := Point{nums[0], nums[1]}
		beacon := Point{nums[2], nums[3]}
		d := dist(sensor, beacon)
		if (sensor.x + d > maxX){
			maxX = sensor.x + d
		}
		if (beacon.x > maxX){
			maxX = beacon.x
		}
		if (sensor.x - d < minX){
			minX = sensor.x - d
		}
		if (beacon.x < minX){
			minX = beacon.x
		}
		pairs = append(pairs, Pair{sensor, beacon})
	}
	return pairs, maxX, minX
}

func findEmpty(pairs []Pair, maxX int, minX int, row int) int {
	count := 0
	for i := minX; i <= maxX; i++ {
		pt := Point{i, row}
		for _, pair := range pairs {
			sensor := pair.sensor
			beacon := pair.beacon
			maxD := dist(sensor, beacon)
			if dist(pt, sensor) <= maxD && pt != beacon && pt != sensor {
				count++
				break;
			}
		}
	}
	return count
}

func findBeacon(pairs []Pair, max int) Point {
	for y := 0; y <= max; y++ {
		for x := 0; x <= max; x++ {
			pt := Point{x,y}
			inrange := false
			for _, pair := range pairs {
				distToSensor := dist(pair.sensor, pt)
				sensorStrength := dist(pair.sensor, pair.beacon)
				if distToSensor <= sensorStrength {
					inrange = true
					// Can safely skip ahead
					x = x + sensorStrength - distToSensor
					break
				}
			}
			if (!inrange) {
				return pt
			}
		}
	}
	return Point{0,0}
}

func main() {
	input, err := ioutil.ReadFile("input.txt")
	if err != nil {
		panic(err)
	}
	pairs, maxX, minX := parse(strings.Split(string(input), "\n"))
	fmt.Printf("Part One Answer: %d\n", findEmpty(pairs, maxX, minX, 2000000))
	mult := 4000000
	beacon := findBeacon(pairs, mult)
	fmt.Printf("Part Two Answer: %d\n", beacon.x * mult + beacon.y)
}