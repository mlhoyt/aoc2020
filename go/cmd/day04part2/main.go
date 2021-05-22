package main

import (
	"fmt"
	"strings"

	parser "github.com/mlhoyt/aoc2020/go/pkg/passportparser"
	"github.com/mlhoyt/aoc2020/go/pkg/utils"
)

func main() {
	input, err := utils.LoadInputFile("day04.txt")
	if err != nil {
		panic(err)
	}

	passports, _ := stringList(strings.Split(input, "\n\n")).MapToPassportList(newPassportFromString)

	passports = passports.Filter(func(v parser.Passport) bool {
		return v.IsValid()
	})

	fmt.Printf("%d\n", len(passports))
}

type stringList []string

func (self stringList) MapToPassportList(f func(string) (parser.Passport, error)) (passportList, []error) {
	items := passportList{}
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

func newPassportFromString(s string) (parser.Passport, error) {
	return parser.Parse(s)
}

type passportList []parser.Passport

func (self passportList) Filter(f func(parser.Passport) bool) passportList {
	items := passportList{}

	for _, v := range self {
		if f(v) {
			items = append(items, v)
		}
	}

	return items
}
