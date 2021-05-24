package bspparser

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestParse(t *testing.T) {
	testcases := []struct {
		input       string
		expectedRow int
		expectedCol int
		expectedID  int
	}{
		{
			input:       "BFFFBBFRRR",
			expectedRow: 70,
			expectedCol: 7,
			expectedID:  567,
		},
		{
			input:       "FFFBBBFRRR",
			expectedRow: 14,
			expectedCol: 7,
			expectedID:  119,
		},
		{
			input:       "BBFFBBFRLL",
			expectedRow: 102,
			expectedCol: 4,
			expectedID:  820,
		},
	}

	for _, tc := range testcases {
		t.Run(tc.input, func(t *testing.T) {
			actual, err := Parse(tc.input)
			assert.Nil(t, err)
			assert.Equal(t, tc.expectedRow, actual.Row())
			assert.Equal(t, tc.expectedCol, actual.Column())
			assert.Equal(t, tc.expectedID, actual.ID())
		})
	}
}
