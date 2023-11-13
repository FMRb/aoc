AFTER_DAYS = 256


def part1(data):
    lanterfish = list(map(int, data[0].split(',')))
    ocean = []
    for fs in range(9):
        ocean.append(0 + lanterfish.count(fs))

    i = 0
    while i < AFTER_DAYS:
        breed = ocean.pop(0)
        if breed > 0:
            ocean.append(breed)
            ocean[6] = ocean[6] + breed
        else:
            ocean.append(0)
        i += 1

    print(sum(ocean))
    return


'''
[0,1,1,2,1,0,0,0,0]
[1,1,2,1,0,0,0,0]
[1,2,1,0,0,1,0,1]
[2,1,0,1,2,1,1]


Initial state: 3,4,3,1,2
After  1 day:  2,3,2,0,1
After  2 days: 1,2,1,6,0,8
After  3 days: 0,1,0,5,6,7,8
After  4 days: 6,0,6,4,5,6,7,8,8
After  5 days: 5,6,5,3,4,5,6,7,7,8
After  6 days: 4,5,4,2,3,4,5,6,6,7
After  7 days: 3,4,3,1,2,3,4,5,5,6
After  8 days: 2,3,2,0,1,2,3,4,4,5
After  9 days: 1,2,1,6,0,1,2,3,3,4,8
After 10 days: 0,1,0,5,6,0,1,2,2,3,7,8
After 11 days: 6,0,6,4,5,6,0,1,1,2,6,7,8,8,8
After 12 days: 5,6,5,3,4,5,6,0,0,1,5,6,7,7,7,8,8
After 13 days: 4,5,4,2,3,4,5,6,6,0,4,5,6,6,6,7,7,8,8
After 14 days: 3,4,3,1,2,3,4,5,5,6,3,4,5,5,5,6,6,7,7,8
After 15 days: 2,3,2,0,1,2,3,4,4,5,2,3,4,4,4,5,5,6,6,7
After 16 days: 1,2,1,6,0,1,2,3,3,4,1,2,3,3,3,4,4,5,5,6,8
After 17 days: 0,1,0,5,6,0,1,2,2,3,0,1,2,2,2,3,3,4,4,5,7,8
After 18 days: 6,0,6,4,5,6,0,1,1,2,6,0,1,1,1,2,2,3,3,4,6,7,8,8,8,8
'''

if __name__ == '__main__':
    data = [x.strip() for x in open('data.in').readlines()]
    part1(data)
