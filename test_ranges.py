import pandas as pd
import numpy as np
import os
import sys, ctypes
from ctypes import c_char_p, c_int32, c_int64, Structure, POINTER, c_int32, c_size_t, pointer

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


def make_rles_same_length(self, other, identity, identity_type):

    self_lengths_series = self.lengths()
    other_lengths_series = other.lengths()

    self_length = self_lengths_series.sum()
    other_length = other_lengths_series.sum()

    if self_length < other_length:

        difference = other_length - self_length
        self_lengths_copy = self_lengths_series.copy().append(pd.Series(difference, dtype=identity_type))

        self_values_copy = self.values().append(pd.Series(identity, dtype=identity_type))

        return Rle(self_lengths_copy, self_values_copy), other

    elif other_length < self_length:

        difference = self_length - other_length
        other_lengths_copy = other_lengths.copy().append(pd.Series(difference, dtype=identity_type))

        other_values_copy = other.values().append(pd.Series(identity, dtype=identity_type))

        return self, Rle(other_lengths_copy, other_values_copy)

    else:

        return self, other


class Rle:

    def __init__(self, lengths, values):

        lengths_length, values_length = len(lengths), len(values)

        values_type = pd.Series(values).dtype

        if values_type in ["int32", "int64"]:
            rle_type = np.int32
        else:# elif values_type in ["float32" or "float64"]:
            raise ValueError(values_type + " not supported.")

        self.dtype = rle_type

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

    def lengths(self):

        self_lengths = lib.int_rle_lengths(self.ptr)
        self_lengths_size = lib.int_rle_lengths_size(self.ptr)
        lengths = pd.Series(np.fromiter(self_lengths, dtype=np.int32, count=self_lengths_size))
        return lengths.copy()

    def values(self):

        if self.dtype == np.int32:
            self_values_size = lib.int_rle_values_size(self.ptr)
            self_values = lib.int_rle_values(self.ptr)
            values = pd.Series(np.fromiter(self_values, dtype=np.int32, count=self_values_size))
        else:
            raise ValueError(str(self.dtype) + " not supported.")

        return values.copy()

    def __add__(self, other):

        self, other = make_rles_same_length(self, other, 0, np.int32)

        self_lengths_size = lib.int_rle_lengths_size(self.ptr)
        self_lengths = lib.int_rle_lengths(self.ptr)

        other_lengths_size = lib.int_rle_lengths_size(other.ptr)
        other_lengths = lib.int_rle_lengths(other.ptr)

        self_lengths_series = pd.Series(np.fromiter(self_lengths, dtype=np.int32, count=self_lengths_size))
        other_lengths_series = pd.Series(np.fromiter(other_lengths, dtype=np.int32, count=other_lengths_size))

        self_length = self_lengths_series.sum()
        other_length = other_lengths_series.sum()

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
rle = Rle(np.array([1, 1]), [1, 1])
rle2 = Rle([2, 1, 1], [3, 1, 2])
# print(rle)
# print(rle2)
print(rle)
print(rle2)
print(rle + rle2)
