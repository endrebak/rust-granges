import numpy as np
import os
import sys, ctypes
from ctypes import c_char_p, c_uint32, Structure, POINTER, c_int32, c_size_t, pointer

class RleS(Structure):
    pass

prefix = {'win32': ''}.get(sys.platform, 'lib')
extension = {'darwin': '.dylib', 'win32': '.dll'}.get(sys.platform, '.so')
libpath = os.environ.get("LD_LIBRARY_PATH", "target/debug") + "/"
libpath = libpath + prefix + "ranges" + extension
try:
    lib = ctypes.cdll.LoadLibrary(libpath)
except OSError:
    print("Library not found at " + libpath)

# lib = ctypes.cdll.LoadLibrary("/Users/endrebakkenstovner/havpryd/code/rust-ffi-omnibus/examples/objects/target/debug/" +prefix + "objects" + extension)

lib.rle_new.restype = POINTER(RleS)

lib.rle_free.argtypes = (POINTER(RleS), )

# lib.rle_populate.argtypes = (POINTER(RleS), )

lib.rle_values.argtypes = (POINTER(RleS), )
lib.rle_values.restypes = POINTER(c_int32)

lib.rle_lengths.argtypes = (POINTER(RleS), )
lib.rle_lengths.restypes = POINTER(c_int32)

lib.rle_values_size.argtypes = (POINTER(RleS), )
lib.rle_values_size.restypes = c_int32

lib.rle_lengths_size.argtypes = (POINTER(RleS), )
lib.rle_lengths_size.restypes = c_int32
# lib.rle_population_of.restype =


class Rle:
    def __init__(self, lengths, values):

        lengths_length, values_length = len(lengths), len(values)

        assert lengths_length == values_length

        lengths_array = (c_int32 * len(lengths))(*lengths)
        values_array = (c_int32 * len(values))(*values)
        self.obj = lib.rle_new(lengths_array, c_size_t(lengths_length), values_array, c_size_t(values_length))

    def __enter__(self):
        return self

    def __exit__(self, exc_type, exc_value, traceback):
        lib.rle_free(self.obj)

    def __str__(self):

        lengths_size = lib.rle_lengths_size(self.obj)
        values_size = lib.rle_values_size(self.obj)
        print(values_size, "values_size")
        print(lengths_size, "lengths_size")
        assert lengths_size == values_size

        values_pointer = lib.rle_values(self.obj)
        print("values_pointer:", values_pointer)
        ar = ctypes.cast(values_pointer, ctypes.POINTER(ctypes.c_int32)).contents
        print(ar)

        # print(ar[0])
        # print([values[i] for i in range(values_size)])

        # dtype = ctypes.c_int32 * values_size
        # # array_pointer = ctypes.cast(lib.rle_values(self.obj), ctypes.POINTER(dtype))
        # p = pointer(c_int32(lib.rle_values(self.obj)))
        # a = np.fromiter(p, dtype=np.int32, count=values_size) # copy

        # print(a)

        # print(lib.rle_values(self.obj))
        # p = ctypes.POINTER(ctypes.c_int32)
        # print(ctypes.cast(lib.rle_values(self.obj), p))
        # print(ctypes.cast(lib.rle_values(self.obj), p).contents)
        # values = lib.rle_values(self.obj)
        # vals = [values[i] for i in range(values_size)]
        # addr = lib.rle_values(self.obj)
        # ArrayType = ctypes.c_int32*values_size
        # # addr = ctypes.addressof(contents)
        # a = np.frombuffer(ArrayType.from_address(addr))
        # print(a)

        # values = ctypes.cast(lib.rle_values(self.obj), ctypes.POINTER(ctypes.c_int32))
        # ar = np.fromiter(ctypes.POINTER(ctypes.c_int32), dtype=np.int32, count=values_size)
        # print(ar)


rle = Rle([1, 1, 2] * 10, [1, 1, 2] * 10)
print(rle)
# with ZipCodeDatabase() as database:
#     database.populate()
#     pop1 = database.population_of("90210")
#     pop2 = database.population_of("20500")
#     print(pop1 - pop2)
