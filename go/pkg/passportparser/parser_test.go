package passportparser

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestParserCompletePassport(t *testing.T) {
	input := `byr:1234 iyr:2468 eyr:1357 hgt:13cm hcl:#abcdef ecl:neon pid:123456789 cid:24681357`
	output, err := Parse(input)

	assert.Nil(t, err)
	assert.Equal(t, 1234, output.byr)
	assert.Equal(t, 2468, output.iyr)
	assert.Equal(t, 1357, output.eyr)
	assert.Equal(t, 13, output.hgt.value)
	assert.Equal(t, dimensionCM, output.hgt.units)
	assert.Equal(t, color("#abcdef"), output.hcl)
	assert.Equal(t, color("neon"), output.ecl)
	assert.Equal(t, 123456789, output.pid)
	assert.Equal(t, 24681357, output.cid)
	assert.True(t, output.IsValid())
}

func TestParserIncompletePassport(t *testing.T) {
	input := `byr:1470
iyr:2581`
	output, err := Parse(input)

	assert.Nil(t, err)
	assert.Equal(t, 1470, output.byr)
	assert.Equal(t, 2581, output.iyr)
	assert.Equal(t, 0, output.eyr)
	assert.Equal(t, 0, output.hgt.value)
	assert.Equal(t, dimension(""), output.hgt.units)
	assert.Equal(t, color(""), output.hcl)
	assert.Equal(t, color(""), output.ecl)
	assert.Equal(t, 0, output.pid)
	assert.Equal(t, 0, output.cid)
	assert.False(t, output.IsValid())
}

func TestParserRandom1(t *testing.T) {
	input := `hgt:71in eyr:2037
ecl:#8e276e hcl:z iyr:2019
byr:2022 pid:157cm`
	output, err := Parse(input)
	assert.Nil(t, err)
	assert.Equal(t, 2022, output.byr, "byr")
	assert.Equal(t, 2019, output.iyr, "iyr")
	assert.Equal(t, 2037, output.eyr, "eyr")
	assert.Equal(t, 71, output.hgt.value, "hgt.value")
	assert.Equal(t, dimensionIN, output.hgt.units, "hgt.units")
	assert.Equal(t, color("z"), output.hcl, "hcl")
	assert.Equal(t, color("#8e276e"), output.ecl, "ecl")
	assert.Equal(t, 1, output.pid, "pid")
	assert.Equal(t, 0, output.cid, "cid")
	assert.True(t, output.IsValid())
}
