from ffilib import *
a = gen_random_copy(100000)
b = gen_random_copy(100000)
assert len(add_a_to_b_copy(a, b)) == len(b)
