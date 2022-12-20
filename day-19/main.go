package main

import (
	"fmt"
	"io/ioutil"
	"regexp"
	"strconv"
	"strings"
)

type GeodeRobot struct {
	ore, obsidian int
}

type ObsidianRobot struct {
	ore, clay int
}

type ClayRobot struct {
	ore int
}

type OreRobot struct {
	ore int
}

type BluePrint struct {
	id int
	OreRobot
	ClayRobot
	ObsidianRobot
	GeodeRobot
}

func parseBluePrints(input []string) []BluePrint {
	bps := []BluePrint{}
	reg := regexp.MustCompile(`\d+`)
	for _, line := range input {
		nums := reg.FindAllString(line, -1)
		id, _ := strconv.Atoi(nums[0])
		oreRobotOre, _ := strconv.Atoi(nums[1])
		OreRobot := OreRobot{oreRobotOre}
		clayRobotOre, _ := strconv.Atoi(nums[2])
		ClayRobot := ClayRobot{clayRobotOre}
		obsidianRobotOre, _ := strconv.Atoi(nums[3])
		obsidianRobotClay, _ := strconv.Atoi(nums[4])
		ObisidanRobot := ObsidianRobot{obsidianRobotOre, obsidianRobotClay}
		geodeRobotOre, _ := strconv.Atoi(nums[5])
		geodeRobotObisidan, _ := strconv.Atoi(nums[6])
		GeodeRobot := GeodeRobot{geodeRobotOre, geodeRobotObisidan}
		bp := BluePrint{id, OreRobot, ClayRobot, ObisidanRobot, GeodeRobot}
		bps = append(bps, bp)
	}
	return bps
}

// State: Number of robots, number of resources, time
// Maximize: Number of geode resource

type State struct {
	ore int
	clay int
	obsidian int
	geode int
	OreRobot int
	ClayRobot int
	ObsidianRobot int
	GeodeRobot int
	time int
}

type cachekey struct {
	o,c,b,g int
	time int
	or,cr,orr,gr int
}

func solveBluePrint(bp BluePrint, time int) int {
	visited := map[cachekey]bool{}
	start := State{OreRobot:1, time: time}
	queue := []State{start}
	max := 0
	for len(queue) > 0 {
		s := queue[0]
		queue = queue[1:]
		if s.time < 0 {
			continue
		}
		key := cachekey{
			s.ore, s.clay, s.obsidian, s.geode,
			time,
			s.OreRobot, s.ClayRobot, s.ObsidianRobot, s.GeodeRobot,
		}
		if _, ok := visited[key]; ok {
			continue
		}
		// Track max
		if (s.geode > max){
			max = s.geode
		}
		visited[key] = true
		// Deprecate time
		s.time--
		// Build Robots
		canBuildGeode := s.ore >= bp.GeodeRobot.ore && s.obsidian >= bp.GeodeRobot.obsidian
		canBuildObsidian := s.ore >= bp.ObsidianRobot.ore && s.clay >= bp.ObsidianRobot.clay
		canBuildClay := s.ore >= bp.ClayRobot.ore 
		canBuildOre := s.ore >= bp.OreRobot.ore	
		maxOreNeeded := bp.OreRobot.ore + bp.ClayRobot.ore + bp.GeodeRobot.ore + bp.ObsidianRobot.ore

		// Collect materials
		s.geode += s.GeodeRobot
		s.obsidian +=s .ObsidianRobot
		s.clay += s.ClayRobot
		s.ore += s.OreRobot

		if canBuildGeode {
			newS := s
			newS.ore -= bp.GeodeRobot.ore
			newS.obsidian -= bp.GeodeRobot.obsidian
			newS.GeodeRobot++
			if max < newS.geode + newS.time && newS.time >= 0 {
				queue = append(queue, newS)
			}
			continue
		}
		if canBuildObsidian && s.ObsidianRobot < bp.GeodeRobot.obsidian {
			newS := s
			newS.ore -= bp.ObsidianRobot.ore
			newS.clay -= bp.ObsidianRobot.clay
			newS.ObsidianRobot++
			if max < newS.geode + newS.time && newS.time >= 0 {
				queue = append(queue, newS)
			}
		}
		if canBuildClay && s.ClayRobot < bp.ObsidianRobot.clay {
			newS := s
			newS.ore -= bp.ClayRobot.ore
			newS.ClayRobot++
			if max < newS.geode + newS.time && newS.time >= 0 {
				queue = append(queue, newS)
			}
		}
		if canBuildOre && s.OreRobot < maxOreNeeded {
			newS := s
			newS.ore -= bp.OreRobot.ore
			newS.OreRobot++
			if max < newS.geode + newS.time && newS.time >= 0 {
				queue = append(queue, newS)
			}
		}
		if max < s.geode + s.time && s.time >= 0 {
			queue = append(queue, s)
		}
	}
	return max
}

func main() {
	input, err := ioutil.ReadFile("input.txt")
	if err != nil {
		panic(err)
	}
	blueprints := parseBluePrints(strings.Split(string(input), "\n"))
	partOneAnswer := 0
	for i, bp := range blueprints {
		res := solveBluePrint(bp, 24)
		partOneAnswer += (i+1)*res
	}
	fmt.Printf("Part One Answer: %d\n", partOneAnswer)
	partTwoAnswer := 1
	for _, bp := range blueprints[:3] {
		res := solveBluePrint(bp, 32)
		partTwoAnswer *= res
	}
	fmt.Printf("Part Two Answer: %d\n", partTwoAnswer)
}