package main

import (
	"fmt"
	"strings"

	parser "github.com/mlhoyt/aoc2020/go/pkg/policyandpasswordparser"
	"github.com/mlhoyt/aoc2020/go/pkg/utils"
)

func main() {
	input, err := utils.LoadInputFile("day02.txt")
	if err != nil {
		panic(err)
	}

	items, err := stringList(strings.Split(input, "\n")).MapToPolicyAndPasswordList(newPolicyAndPasswordFromString)
	if err != nil {
		panic(err)
	}

	items = items.Filter(policyAndPasswordIsValid)

	fmt.Printf("%d\n", len(items))
}

type stringList []string

func (self stringList) MapToPolicyAndPasswordList(f func(string) (policyAndPassword, error)) (policyAndPasswordList, error) {
	items := policyAndPasswordList{}
	for _, s := range self {
		item, err := f(s)
		if err != nil {
			return nil, err
		}

		items = append(items, item)
	}

	return items, nil
}

type policyAndPasswordList []policyAndPassword

func (self policyAndPasswordList) Filter(f func(policyAndPassword) bool) policyAndPasswordList {
	items := policyAndPasswordList{}
	for _, item := range self {
		if f(item) {
			items = append(items, item)
		}
	}

	return items
}

type policyAndPassword struct {
	policy   parser.Policy
	password parser.Password
}

func newPolicyAndPasswordFromString(s string) (policyAndPassword, error) {
	policy, password, err := parser.Parse(s)
	return policyAndPassword{policy, password}, err
}

func policyAndPasswordIsValid(x policyAndPassword) bool {
	minCheck := x.password[x.policy.RangeMin-1] == x.policy.Character
	maxCheck := x.password[x.policy.RangeMax-1] == x.policy.Character
	return minCheck != maxCheck
}
