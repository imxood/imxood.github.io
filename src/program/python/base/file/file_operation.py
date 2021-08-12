

def writeRowContent(filename, rowToInsert, content, rowToDel = 0, totleToDel = 0):
    with open(filename, 'r+') as f:
        lines = f.readlines()
        while totleToDel > 0:
            lines.pop(rowToDel-1)
            totleToDel -= 1
        lines.insert(rowToInsert-1, content)
        f.seek(0, 0)
        f.truncate()
        f.writelines(lines)
        f.flush()

temp = '''
11
22
33
44
55
66
'''

with open("test.txt", "w") as f:
    f.write(temp)


content = '''
line1
line2
'''
with open("test.txt", "r") as f:
    print(f.readlines())

writeRowContent("test.txt", 3, content, 3, 2)

with open("test.txt", "r") as f:
    print(f.readlines())
