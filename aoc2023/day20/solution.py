from collections import deque
import math

def press_button(modules, state, record_conjuction={}, turn=0):
    init_targets = modules["broadcaster"]

    q = deque([(None, t, 0) for t in init_targets])

    # counting initial pulse from broadcaster
    lows = 1
    highs = 0

    while q:
        mf, module, pulse = q.popleft()
        if pulse == 0:
            lows += 1
        else:
            highs += 1

            
        if module not in modules:
            continue

        m_type, targets = modules[module]

        if m_type == "%":
            if pulse == 1:
                continue
            else:
                n_pulse = int(not state[module])
                state[module] = n_pulse
                for t in targets:
                    q.append((module, t, n_pulse))
        else:
            conjuction = state[module]
            conjuction[mf] = pulse
            n_pulse = int(not all(conjuction.values()))
            if n_pulse == 0:
                record_conjuction.setdefault(module, ([],[]))[0].append(turn)
            else:
                record_conjuction.setdefault(module, ([],[]))[1].append(turn)
            for t in targets:
                q.append((module, t, n_pulse))

    return lows, highs


def part1(modules, state):
    total_lows = 0
    total_highs = 0

    for _ in range(1000):
        lows, highs = press_button(modules, state)
        total_lows += lows
        total_highs += highs
        

    print("Part 1: ", total_lows * total_highs)


def part2(modules, state):

    # Rx module comes from a conjuction "mg"
    # the conjuction depends on:
    # {'jg', 'rh', 'jm', 'hf'}
    # Finding when all those conjuction turns 1 is 
    # the problem to solve
    mg_conjuctions = ["jg", "rh", "jm", "hf"]

    record_conjuction = {}
    i = 0
    while i < 20000:
        press_button(modules, state, record_conjuction, i)
        i+= 1
    
    repetitions = []
    for key_conjuction in mg_conjuctions:
        _, ones = record_conjuction[key_conjuction]
        r = [b - a for a, b in zip(ones, ones[1:])]
        repetitions.append(r[0])

    result = math.lcm(*repetitions)
    print("Part 2: ", result)

if __name__ == "__main__":
    modules = {}
    state = {}
    for line in open(0).read().strip().splitlines():
        module, targets = line.split(" -> ")
        targets = targets.split(", ")
        if module[0] in "%&":
            t_module = module[0]
            m_name = "".join(module[1:])
            modules[m_name] = (t_module, targets)
            if t_module == "%":
                state[m_name] = 0  # low 0 - high 1
            else:
                # remember connected modules
                state[m_name] = {}
        else:
            modules["broadcaster"] = targets

    conjuctions = [m for m in modules if modules[m][0] == "&"]
    for m in modules:
        if m == "broadcaster":
            continue
        _, targets = modules[m]
        for t in targets:
            if t in conjuctions:
                state[t][m] = 0

    part2(modules, state)
