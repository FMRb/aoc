result = 0
# for line in open(0):
#     first = ''
#     last = ''

#     for ch in reversed(list(line)):
#         if ch.isdigit():
#             last = ch
#             break       
#     for ch in list(line):
#         if ch.isdigit():
#             first = ch
#             break
#     result += int(first+last)
dn = {
    'one': '1',
    'two': '2',
    'three': '3',
    'four': '4',
    'five': '5',
    'six': '6',
    'seven': '7',
    'eight': '8',
    'nine': '9',
}
numbers = ['one', 'two', 'three', 'four', 'five', 'six', 'seven', 'eight', 'nine']
def getNumber(index, line, reverse=False):
    for n in numbers:
        if reverse:
            check = line[len(line)-len(n)-index:len(line)-index] 
            if n == check:
                return dn[n]
        else:    
            check = line[index:len(n)+index] 
            if n == line[index:len(n)+index]:
                return dn[n]

    return None

for line in open(0):
    line = line.strip()
    for index, ch in enumerate(reversed(list(line))):
        if ch.isdigit():
            last = ch
            break 
        n = getNumber(index, line, True)
        if n != None:
            last = getNumber(index, line, True)      
            break;
    for index,ch in enumerate(list(line)):
        if ch.isdigit():
            first = ch
            break
        if getNumber(index, line) != None:
            first = getNumber(index, line)
            break
    result += int(first+last)

print(result)