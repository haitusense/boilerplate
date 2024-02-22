try:
  import boilerplatePy
  dir(boilerplatePy)
  boilerplatePy.hello_world()
except ModuleNotFoundError:
  print('Error')

try:
  import boilerplatePyo
  # if not use __init__.py
  # from boilerplatePyo import sum_as_string
  boilerplatePyo.cprintln("green", "dir", ", ".join(dir(boilerplatePyo)))
  boilerplatePyo.hello_world()
  boilerplatePyo.cprintln("green", "sum_usize", boilerplatePyo.sum_usize(1, 3))
  boilerplatePyo.test()
  boilerplatePyo.kwargs(a = 1 + 2, b = "hoge")
  boilerplatePyo.kwargs(a = 1 + 2, b = "hoge", c = 4)
  boilerplatePyo.kwargs("f", b = "hoge", c = 4)
except ModuleNotFoundError:
  print('Error')

def func2(*args, **kwargs):
  print(type(args))           # <class 'tuple'>
  print(type(kwargs))         # <class 'dict'>
  print('args = ', args)      # args =  (1, 3)
  print('kwargs = ', kwargs)  # kwargs =  {'b': 5, 'c': 3}
func2(1, 3, aarg = 2 + 3, barg = "3") # cが二つあると、SyntaxError: keyword argument repeated: c






