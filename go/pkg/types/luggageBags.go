package types

type LuggageBagLabel struct {
	Modifier string
	Color    string
}

type LuggageBagLabelWithCount struct {
	LuggageBagLabel
	Count int
}

type LuggageBagSpecs map[LuggageBagLabel][]LuggageBagLabelWithCount

func (self LuggageBagSpecs) CountContainedByBags(v LuggageBagLabel) int {
	tracedBags := map[LuggageBagLabel]bool{}

	targetBags := map[LuggageBagLabel]bool{
		v: true,
	}
	for len(targetBags) > 0 {
		nextTargetBags := map[LuggageBagLabel]bool{}
		for targetBag := range targetBags {
			for _, bag := range self.findContainedBy(targetBag) {
				nextTargetBags[bag] = true
				tracedBags[bag] = true
			}
		}

		targetBags = nextTargetBags
	}

	return len(tracedBags)
}

func (self LuggageBagSpecs) findContainedBy(v LuggageBagLabel) []LuggageBagLabel {
	matchesMap := map[LuggageBagLabel]bool{}
	for k, labels := range self {
		for _, label := range labels {
			if label.LuggageBagLabel == v {
				matchesMap[k] = true
			}
		}
	}

	matches := []LuggageBagLabel{}
	for label := range matchesMap {
		matches = append(matches, label)
	}

	return matches
}

func (self LuggageBagSpecs) CountContainsBags(v LuggageBagLabel) int {
	count := 0

	for _, contains := range self[v] {
		count += contains.Count + (contains.Count * self.CountContainsBags(contains.LuggageBagLabel))
	}

	return count
}
