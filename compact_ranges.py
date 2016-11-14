import pandas as pd
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


lib.rle_find_zero_lengths.argtypes = (POINTER(c_int32), c_int32)
lib.rle_find_zero_lengths.restype = c_int32

lib.int_rle_new.restype = POINTER(RleS)

lib.int_rle_free.argtypes = (POINTER(RleS), )

lib.int_rle_values.argtypes = (POINTER(RleS), )
lib.int_rle_values.restype = POINTER(c_int32)

lib.int_rle_lengths.argtypes = (POINTER(RleS), )
lib.int_rle_lengths.restype = POINTER(c_int32)

lib.int_rle_values_size.argtypes = (POINTER(RleS), )
lib.int_rle_values_size.restype = c_int32

lib.int_rle_lengths_size.argtypes = (POINTER(RleS), )
lib.int_rle_lengths_size.restype = c_int32

lib.int_rle_add.argtypes = (POINTER(RleS), POINTER(RleS))
lib.int_rle_add.restype = POINTER(RleS)


def format_rle(size, lengths_pointer, values_pointer):
    """
    Prints the lengths and values of the run length encoding
    """

    if size > 20:
        return "".join(["Lengths: [",
                ", ".join([str(i) for i in lengths_pointer[:10]]),
                ", ..., ",
                ", ".join([str(i) for i in lengths_pointer[size-10:size]]),
                "]\nValues: [",
                ", ".join([str(i) for i in values_pointer[:10]]),
                ", ..., ",
                ", ".join([str(i) for i in values_pointer[size-10:size]]),
                "]"])
    else:
        return "Lengths: " + str(lengths_pointer[:size]) + "\nValues: " + str(values_pointer[:size])


class Rle:

    def __init__(self, lengths, values):

        lengths_length, values_length = len(lengths), len(values)

        assert lengths_length == values_length

        lengths_array = (c_int32 * len(lengths))(*lengths)
        values_array = (c_int32 * len(values))(*values)

        contains_zero = lib.rle_find_zero_lengths(lengths_array, lengths_length)

        if contains_zero:
            raise ValueError("Lengths array contains zero values.")

        self.ptr = lib.int_rle_new(lengths_array, c_size_t(lengths_length), values_array, c_size_t(values_length))


    def __enter__(self):
        return self


    def __exit__(self, exc_type, exc_value, traceback):
        lib.int_rle_free(self.ptr)


    def __str__(self):

        lengths_size = lib.int_rle_lengths_size(self.ptr)
        values_size = lib.int_rle_values_size(self.ptr)

        assert lengths_size == values_size

        size = lengths_size

        values_pointer = lib.int_rle_values(self.ptr)
        lengths_pointer = lib.int_rle_lengths(self.ptr)

        return format_rle(size, lengths_pointer, values_pointer)


    def __add__(self, other):

        new_rle = lib.int_rle_add(self.ptr, other.ptr)

        lengths_size = lib.int_rle_lengths_size(new_rle)
        values_size = lib.int_rle_values_size(new_rle)

        assert lengths_size == values_size

        size = lengths_size

        values_pointer = lib.int_rle_values(new_rle)
        lengths_pointer = lib.int_rle_lengths(new_rle)

        lengths = np.fromiter(lengths_pointer, dtype=np.int32, count=size)
        values = np.fromiter(values_pointer, dtype=np.int32, count=size)

        return Rle(lengths, values)


# rle = Rle(np.array([1, 1, 1, 1] * 10), [1, 1, 2, 3] * 10)
# rle2 = Rle([1, 1, 1, 1] * 10, [3, 1, 2, 3] * 10)
# rle = Rle(np.array([0, 1]), [1, 1])
rle2 = Rle([2, 1, 1], [3, 1, 2])
# print(rle)
# print(rle2)
# print(rle + rle2)
