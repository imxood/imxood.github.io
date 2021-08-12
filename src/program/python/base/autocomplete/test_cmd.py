import cmd
import sys
# try:
#     import gnu

class HelloWorld(cmd.Cmd):

    FRIENDS = ['XiaoMing', 'XiaoHong', 'XiaoFang', 'XiaoBai']

    def do_greet(self, name):
        print('Hello, ', name)

    def complete_greet(self, text, line, begidx, endidx):
        if not text:
            completions = self.FRIENDS[:]
        else:
            completions = [
                f
                for f in self.FRIENDS
                if f.startswith(text)
            ]
        return completions

    def do_EOF(self, line):
        return True

if __name__ == "__main__":
    HelloWorld().cmdloop()
