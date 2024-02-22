import warnings
RED = '\033[31m'
GREEN = '\033[32m'
END = '\033[0m'

def foo():
  warnings.warn(GREEN + "この関数は廃止されました" + END, Warning, stacklevel=1)
  print(GREEN + 'hello world'+ END)

# foo()
# foo()

import boilerplatePyo
for key in ["normal", "error", "error_stack", "warning", "panic"]:
  try:
    print(f"-------- {key} --------")
    print(boilerplatePyo.panic(key))
    print(boilerplatePyo.panic(key))
    foo()
  except Exception as e:
    print(f"Caught an exception: {e}")

