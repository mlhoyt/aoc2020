package main

import (
	"fmt"
	"strings"

	"github.com/mlhoyt/aoc2020/go/pkg/utils"
)

func main() {
	input, err := utils.LoadInputFile("day06.txt")
	if err != nil {
		panic(err)
	}

	groups := [][]map[string]bool{}
	for _, groupString := range strings.Split(input, "\n\n") {
		group := []map[string]bool{}
		for _, memberString := range strings.Split(groupString, "\n") {
			member := map[string]bool{}
			for _, v := range memberString {
				member[string(v)] = true
			}

			group = append(group, member)
		}

		groups = append(groups, group)
	}

	answers := []int{}
	for _, group := range groups {
		combined := group[0]

		for _, member := range group {
			for answer := range combined {
				if _, ok := member[answer]; !ok {
					delete(combined, answer)
				}
			}
		}

		answers = append(answers, len(combined))
	}

	result := 0
	for _, answer := range answers {
		result += answer
	}

	fmt.Printf("%d\n", result)
}
