package bspparser

import (
	"math"
)

type Seat struct {
	bspRow      string
	bspCol      string
	seatsPerRow int
	row         int
	col         int
}

func NewSeat(row string, col string) Seat {
	return Seat{
		bspRow:      row,
		bspCol:      col,
		seatsPerRow: int(math.Pow(2, float64(len(col)))),
		row:         bspToInt(row),
		col:         bspToInt(col),
	}
}

func (self Seat) Row() int {
	return self.row
}

func (self Seat) Column() int {
	return self.col
}

func (self Seat) ID() int {
	return (self.row * self.seatsPerRow) + self.col
}

func bspToInt(bsp string) int {
	max := int(math.Pow(2, float64(len(bsp))))
	result := 0
	for _, value := range bsp {
		if value == '1' {
			result += max / 2
		}

		max /= 2
	}

	return result
}
