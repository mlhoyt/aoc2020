package main

import (
	"fmt"
	"sort"
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

	seatIDs := seats.MapToIntList(func(v parser.Seat) int {
		return v.ID()
	})

	seatIDs = seatIDs.Sort()

	// Identify isolated seat (ID-1 and ID+1 exist)
	for i := 1; i < len(seatIDs); i++ {
		if seatIDs[i-1] == seatIDs[i]-2 {
			fmt.Printf("isolated seat: %d\n", seatIDs[i]-1)
		}
	}
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

func (self seatList) MapToIntList(f func(parser.Seat) int) intList {
	items := intList{}
	for _, v := range self {
		items = append(items, f(v))
	}

	return items
}

type intList []int

func (self intList) Clone() intList {
	cloned := make(intList, len(self))
	for _, v := range self {
		cloned = append(cloned, v)
	}

	return cloned
}

func (self intList) Sort() intList {
	sorted := self.Clone()

	sort.Ints([]int(sorted))

	return sorted
}
