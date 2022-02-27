# 识别c/c++注释
import os
import re
import glob


script_dir = os.path.dirname(os.path.abspath(__file__))


def stripcomments(text):
    return re.sub(r'//.*?\n|/\*.*?\*/', '', text, flags=re.S)


# 把行注释替换成块注释
# r'(//.*?\n)|(/\*.*?\*/)'
files = glob.glob(script_dir + '/../../c/**/*.c', recursive=True)
for f in files:
    pattern = re.compile(r'//(.*)\n')
    with open(f, 'r+') as fd:
        lines = fd.readlines()
        for index, line in enumerate(lines):
            match = pattern.search(line)
            if match:
                new_line = '{}/* {} */\n'.format(line[:match.start()], match.group(1).lstrip().rstrip())
                lines[index] = new_line
        fd.seek(0)
        fd.truncate()
        fd.writelines(lines)
        fd.flush()
