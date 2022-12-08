package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

func contains(s []string, str string) bool {
	for _, v := range s {
		if v == str {
			return true
		}
	}
	return false
}

func main() {
	f, err := os.Open("input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()
	scanner := bufio.NewScanner(f)
	dirs := map[string] int{}
	// Each file should only count to the directory size once
	files := map[string] []string{}
	var path []string
	for scanner.Scan() {
		line := scanner.Text()
		if line[0:1] == "$" {
			// Command
			if (line[2:4] == "cd") {
				// Move up
				if (line[5:] == "..") {
					path = path[:len(path)-1]
				} else {
					path = append(path, line[5:])
				}
			}
		} else {
			// Listing
			strs := strings.Fields((line))

			if (strs[0] == "dir") {
				dirPath := strings.Join(path[:], "/") + "/" + strs[1]
				// Directory
				if _, ok := dirs[dirPath]; !ok {
					dirs[dirPath] = 0
				}
			} else {
				// File
				size, _ := strconv.Atoi(strs[0])
				file := strings.Join(path[:], "/") + "/" + strs[1]
				// Add size to all dir in path
				for i := range path {
					dirPath := strings.Join(path[0:i+1], "/")
					fileChecked := contains(files[dirPath], file)
					if (!fileChecked){
						dirs[dirPath] = dirs[dirPath] + size
						files[dirPath] = append(files[dirPath], file)
					}
				}
			}
		}
	}
	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}
	answerOne := 0
	totalSize := dirs["/"]
	freeSpace := 70000000 - totalSize
	target := 30000000 - freeSpace
	answerTwo := totalSize
	for _, size := range dirs {
		if (size <= 100000) {
			answerOne += size
		}
		if (size >= target && size < answerTwo) {
			answerTwo = size
		}
	}
	fmt.Printf("Part One Answer: %d\n", answerOne)
	fmt.Printf("Part Two Answer: %d\n", answerTwo)
}