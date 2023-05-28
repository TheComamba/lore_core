local function libname(ffi)
    if ffi.os == "Windows" then
        return "liblorecore.dll"
    elseif ffi.os == "Linux" then
        return "liblorecore.so"
    elseif ffi.os == "OSX" then
        return "liblorecore.dylib"
    else
        error("Unsupported OS")
    end
end

local function try_loading_lib(ffi)
    for _, start in ipairs({ "./", "../" }) do
        for _, path in ipairs({ "./", "artifacts/", "target/debug/", "target/release/" }) do
            local fullpath = start .. path .. libname(ffi)
            print("Trying to find " .. fullpath)
            local f = io.open(fullpath, "r")
            if f then
                f:close()
                print("\nLoading " .. fullpath)
                ffi.load(path .. libname(ffi))
                return
            end
        end
    end
    error("Could not find library file")
end

local function main()
    print("This script tests loading the shared library. It will try to load the library from several locations. If it fails, you may need to run `cargo build` first.\n")

    print("Loading FFI")
    local ffi = require("ffi")
    if not ffi then
        error("Could not load FFI")
    end

    print("Loading header file")
    local header_file = io.open("lorecore_api.h", "r")
    if not header_file then
        header_file = io.open("../lorecore_api.h", "r")
    end
    if not header_file then
        error("Could not find header file")
    end
    local header = header_file:read "*all"
    header_file:close()

    print("Defining FFI")
    ffi.cdef(header)

    try_loading_lib(ffi)
end

main()
