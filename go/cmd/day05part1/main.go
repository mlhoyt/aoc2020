package main

import (
	"fmt"
	"strings"

	parser "github.com/mlhoyt/aoc2020/go/pkg/bspparser"
	"github.com/mlhoyt/aoc2020/go/pkg/utils"
)

func main() {
	input, err := utils.LoadInputFile("day05.txt")
	if err != nil {
		panic(err)
	}

	seats, _ := stringList(strings.Split(input, "\n")).MapToSeatList(newSeatFromString)

	result, _ := seats.Max(func(v parser.Seat) int {
		return v.ID()
	})

	fmt.Printf("%d\n", result)
}

type stringList []string

func (self stringList) MapToSeatList(f func(string) (parser.Seat, error)) (seatList, []error) {
	items := seatList{}
	errors := []error{}

	for _, v := range self {
		item, err := f(v)
		if err != nil {
			errors = append(errors, err)
		} else {
			items = append(items, item)
		}
	}

	return items, errors
}

func newSeatFromString(s string) (parser.Seat, error) {
	return parser.Parse(s)
}

type seatList []parser.Seat

func (self seatList) Max(f func(parser.Seat) int) (int, error) {
	if self == nil || len(self) == 0 {
		return -1, fmt.Errorf("seatList is nil or empty which can have no Max value")
	}

	result := f(self[0])
	for _, v := range self {
		iv := f(v)
		if iv > result {
			result = iv
		}
	}

	return result, nil
}
