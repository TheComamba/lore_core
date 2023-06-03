local ffi = require("ffi")
ffi.cdef [[void Sleep(int ms); int poll(struct pollfd *fds, unsigned long nfds, int timeout);]]
return function(s) ffi.C.poll(nil, 0, s * 1000) end
