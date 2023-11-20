monkeys = {}

lines = open(0)

def sum(a, b): 
    return a + b

def subs(a, b):
    return a - b

def mul(a,b):
    return a * b

def div(a, b):
    return a / b

def eq(a, b):
    return a - b

for line in lines:
    monkeyKey = line.strip().split(": ")[0]
    value = line.strip().split(": ")[1]

    if " + " in value:
        op = value.split(" + ")
        monkeys[monkeyKey] = (op[0], op[1], sum)
    elif " - " in value:
        op = value.split(" - ")
        monkeys[monkeyKey] = (op[0], op[1], subs)
    elif " * " in value:
        op = value.split(" * ")
        monkeys[monkeyKey] = (op[0], op[1], mul)
    elif " / " in value:
        op = value.split(" / ")
        monkeys[monkeyKey] = (op[0], op[1], div)
    elif " = " in value:
        op = value.split(" = ")
        monkeys[monkeyKey] = (op[0], op[1], eq)
    else:
        monkeys[monkeyKey] = int(value)


tolerance = 0.1


def solve(monkey, humnValue):
    if monkey == "humn":
        return humnValue
    if isinstance(monkeys[monkey], int):
        return monkeys[monkey]
    
    op1, op2, fn = monkeys[monkey]

    op1 = solve(op1, humnValue)
    op2 = solve(op2, humnValue)
    result = fn(op1, op2)
    #monkeys[monkey] = result
    return result

previous_num = 5
target = solve("root", previous_num)
previous_error = abs(target)

my_number = 0
target = solve("root", my_number)
my_error = abs(target)

rate = 0.1

while (my_error > tolerance):

    try:
        gradient = (my_number - previous_num) // (my_error - previous_error)
    except ZeroDivisionError:
        gradient = 1 if my_error < previous_error else -1

    print("Gradient: ", gradient)
    print("Error: ", my_error)

    previous_num = my_number
    previous_error = my_error

    my_number -= rate * my_error * gradient

    target = solve("root", my_number)
    my_error = abs(target)


print("Result:", round(my_number))
