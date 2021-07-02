package main

import (
	"fmt"

	parser "github.com/mlhoyt/aoc2020/go/pkg/luggagerulesparser"
	"github.com/mlhoyt/aoc2020/go/pkg/types"
	"github.com/mlhoyt/aoc2020/go/pkg/utils"
)

func main() {
	input, err := utils.LoadInputFile("day07.txt")
	if err != nil {
		panic(err)
	}

	bagSpecs, err := parser.Parse(input)
	if err != nil {
		panic(err)
	}

	result := bagSpecs.CountContainedByBags(types.LuggageBagLabel{
		Modifier: "shiny",
		Color:    "gold",
	})

	fmt.Printf("%d\n", result)
}
