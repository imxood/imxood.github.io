t = {
    "a": 1,
    "b": 2
}

print(t)


def merge(obj):
    x, y = obj
    return '%s=%d' % (x, y)


print(list(map(merge, t.items())))

print(list(map(lambda item: '%s=%d' % (item[0], item[1]), t.items())))

print(list(map(lambda item: '%s=%d' % item, t.items())))


lst = {
    'a': 1,
    'b': {
        'c': 2
    }
}

test_lst = [ k for k, v in lst.items() if v == 1]
print('test_lst: ', test_lst)
