import sys

part = sys.argv[1]

count = 0
while True:
    try: inp = input()
    except: break
    if inp == '': break
    a, b = inp.split(',')
    sa, ea = map(int, a.split('-'))
    sb, eb = map(int, b.split('-'))

    if part == '1':
        if sa <= sb and eb <= ea:
            count += 1
        elif sb <= sa and ea <= eb:
            count += 1
    else:
        if sa <= sb and sb <= ea:
            count += 1
        elif sb <= sa and sa <= eb:
            count += 1
print(count)