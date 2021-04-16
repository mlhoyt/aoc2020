package main

import (
	"fmt"

	"github.com/mlhoyt/aoc2020/go/pkg/utils"
)

func main() {
	input, err := utils.LoadInputFile("day01.txt")
	if err != nil {
		panic(err)
	}

	fmt.Print(input)
}
