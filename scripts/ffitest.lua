local ffi = require("ffi")
ffi.cdef [[void Sleep(int ms); int poll(struct pollfd *fds, unsigned long nfds, int timeout);]]
print("Things are looking good.")
return function(s) ffi.C.poll(nil, 0, s * 1000) end
