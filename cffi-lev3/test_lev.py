# https://cffi.readthedocs.io/en/latest/overview.html
# pip install cffi
from cffi import FFI                                                                                                                                  
ffi = FFI()
ffi.cdef("int levenshtein(char*,char*);") 
lib = ffi.dlopen("libcffi_lev3.so")


def lev(a,b): 
    s1 = ffi.new("char[]", a.encode('utf-8')) 
    s2 = ffi.new("char[]", b.encode('utf-8')) 
    return lib.levenshtein(s1,s2) 


def test_lev():
    assert lev("boo","zoo") == 1
    assert lev("a","bcd") == 3


