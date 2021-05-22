package main

import (
	"fmt"
	"strings"

	"github.com/mlhoyt/aoc2020/go/pkg/utils"
)

func main() {
	input, err := utils.LoadInputFile("day04.txt")
	if err != nil {
		panic(err)
	}

	passports, _ := stringList(strings.Split(input, "\n\n")).MapToPassportList(newPassportFromString)

	passports = passports.Filter(func(v passport) bool {
		return v.IsValid()
	})

	fmt.Printf("%d\n", len(passports))
}

type stringList []string

func (self stringList) MapToPassportList(f func(string) (passport, error)) (passportList, []error) {
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

func newPassportFromString(s string) (passport, error) {
	s = strings.Replace(s, "\n", " ", -1)
	fields := strings.Split(s, " ")

	passport := passport{}
	for _, field := range fields {
		if strings.HasPrefix(field, "byr:") {
			passport.byr = &field
		}
		if strings.HasPrefix(field, "iyr:") {
			passport.iyr = &field
		}
		if strings.HasPrefix(field, "eyr:") {
			passport.eyr = &field
		}
		if strings.HasPrefix(field, "hgt:") {
			passport.hgt = &field
		}
		if strings.HasPrefix(field, "hcl:") {
			passport.hcl = &field
		}
		if strings.HasPrefix(field, "ecl:") {
			passport.ecl = &field
		}
		if strings.HasPrefix(field, "pid:") {
			passport.pid = &field
		}
		if strings.HasPrefix(field, "cid:") {
			passport.cid = &field
		}
	}

	return passport, nil
}

type passport struct {
	byr *string // (Birth Year)
	iyr *string // (Issue Year)
	eyr *string // (Expiration Year)
	hgt *string // (Height)
	hcl *string // (Hair Color)
	ecl *string // (Eye Color)
	pid *string // (Passport ID)
	cid *string // (Country ID) OPTIONAL
}

func (self passport) IsValid() bool {
	return self.byr != nil &&
		self.iyr != nil &&
		self.eyr != nil &&
		self.hgt != nil &&
		self.hcl != nil &&
		self.ecl != nil &&
		self.pid != nil
}

type passportList []passport

func (self passportList) Filter(f func(passport) bool) passportList {
	items := passportList{}

	for _, v := range self {
		if f(v) {
			items = append(items, v)
		}
	}

	return items
}
