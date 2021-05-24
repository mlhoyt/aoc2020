package bspparser

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestBSPToInt(t *testing.T) {
	testcases := []struct {
		input  string
		output int
	}{
		{
			input:  "00",
			output: 0,
		},
		{
			input:  "01",
			output: 1,
		},
		{
			input:  "10",
			output: 2,
		},
		{
			input:  "11",
			output: 3,
		},
	}

	for _, tc := range testcases {
		t.Run(tc.input, func(t *testing.T) {
			actual := bspToInt(tc.input)
			assert.Equal(t, tc.output, actual)
		})
	}
}

func TestNewSeat(t *testing.T) {
	testcases := []struct {
		inputRow    string
		inputCol    string
		expectedRow int
		expectedCol int
		expectedID  int
	}{
		{
			inputRow:    "00",
			inputCol:    "0",
			expectedRow: 0,
			expectedCol: 0,
			expectedID:  0,
		},
		{
			inputRow:    "00",
			inputCol:    "1",
			expectedRow: 0,
			expectedCol: 1,
			expectedID:  1,
		},
		{
			inputRow:    "01",
			inputCol:    "0",
			expectedRow: 1,
			expectedCol: 0,
			expectedID:  2,
		},
		{
			inputRow:    "01",
			inputCol:    "1",
			expectedRow: 1,
			expectedCol: 1,
			expectedID:  3,
		},
		{
			inputRow:    "10",
			inputCol:    "0",
			expectedRow: 2,
			expectedCol: 0,
			expectedID:  4,
		},
		{
			inputRow:    "10",
			inputCol:    "1",
			expectedRow: 2,
			expectedCol: 1,
			expectedID:  5,
		},
		{
			inputRow:    "11",
			inputCol:    "0",
			expectedRow: 3,
			expectedCol: 0,
			expectedID:  6,
		},
		{
			inputRow:    "11",
			inputCol:    "1",
			expectedRow: 3,
			expectedCol: 1,
			expectedID:  7,
		},
	}

	for _, tc := range testcases {
		t.Run(tc.inputRow+tc.inputCol, func(t *testing.T) {
			actual := NewSeat(tc.inputRow, tc.inputCol)
			assert.Equal(t, tc.expectedRow, actual.Row())
			assert.Equal(t, tc.expectedCol, actual.Column())
			assert.Equal(t, tc.expectedID, actual.ID())
		})
	}
}
