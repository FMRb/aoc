package solution

type Solution interface {
	PartOne(input string) (string, error)
	PartTwo(input string) (string, error)
}
