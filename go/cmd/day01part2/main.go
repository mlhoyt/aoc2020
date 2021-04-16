package main

import (
	"fmt"
	"strconv"
	"strings"

	"github.com/mlhoyt/aoc2020/go/pkg/utils"
)

func main() {
	input, err := utils.LoadInputFile("day01.txt")
	if err != nil {
		panic(err)
	}

	numbers, err := stringListToNumberList(strings.Split(input, "\n"))
	if err != nil {
		panic(err)
	}

	groups := numberListToNumberGroups(numbers)

	answers := filterNumberList(groups, func(p numberList) bool {
		return p[0]+p[1]+p[2] == 2020
	})

	for _, answer := range answers {
		fmt.Printf("%d * %d * %d = %d\n", answer[0], answer[1], answer[2], (answer[0] * answer[1] * answer[2]))
	}
}

type number int64

func stringListToNumberList(ss []string) ([]number, error) {
	ns := []number{}
	for _, s := range ss {
		n, err := strconv.ParseInt(s, 10, 64)
		if err != nil {
			return nil, err
		}

		ns = append(ns, number(n))
	}

	return ns, nil
}

type numberList []number

func newNumberList(ns ...number) numberList {
	return ns
}

func numberListToNumberGroups(ns []number) []numberList {
	results := []numberList{}
	for ai, a := range ns {
		for bi, b := range ns[(ai + 1):] {
			for _, c := range ns[(bi + 1):] {
				results = append(results, newNumberList(a, b, c))
			}
		}
	}

	return results
}

func filterNumberList(xs []numberList, f func(numberList) bool) []numberList {
	results := []numberList{}
	for _, x := range xs {
		if f(x) {
			results = append(results, x)
		}
	}

	return results
}
