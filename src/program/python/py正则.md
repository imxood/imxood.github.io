# 正则学习

## 日常记录
    
    (?<=...), This is called a positive lookbehind assertion,  后向断言.
    
    ex1: 想要匹配'def', 但是希望前面是'abc', 
        >>> m = re.search('(?<=abc)def', 'abcdefaaaa')
        >>> m.group(0)
        'def'
        
    ex2: 想要匹配'\w+', 但是希望前面是'-'
        >>> m = re.search(r'(?<=-)\w+', 'spam-egg')
        >>> m.group(0)
        'egg'
        
    (?<!...) 与 (?<=...) 相反
