from PySTAFv3 import *
import sys

try:
    handle = STAFHandle("MyTest")
except STAFException as e:
    print("Error registering with STAF, RC: %d" % e.rc)
    sys.exit(e.rc)

result = handle.submit("local", "ping", "ping")

if (result.rc != 0):
    print("Error submitting request, RC: %d, Result: %s" % (result.rc, result.result))

result = handle.submit("local", "var", "resolve string {STAF/Config/OS/Name}")

if (result.rc != 0):
    print("Error submitting request, RC: %d, Result: %s" % (result.rc, result.result))
else:
    print("OS Name: %s" % result.result)

rc = handle.unregister()

sys.exit(rc)
