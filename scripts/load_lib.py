import sys
import os
from ctypes import CDLL, WinDLL
from typing import Union

def get_lib_name() -> str:
    """Check the platform the code currently is running on and return the corresponding liblorecore file

    Raises:
        ValueError: If the platform is neither Windows, Linux, nor Mac

    Returns:
        str: file name of the lorecore library
    """
    if sys.platform.startswith('win'):
        return "lorecore.dll"
    if sys.platform.startswith('linux'):
        return "liblorecore.so"
    if sys.platform.startswith('darwin'):
        return "liblorecore.dylib"
    raise ValueError("Unsupported OS")

def find_lib_path() -> Union[CDLL, WinDLL]:
    """Goes through all possible path locations and tries to find the lorecore library file

    Raises:
        FileNotFoundError: If the file does not exist in any of the expected locations

    Returns:
        str: path to the lorecore library file
    """
    lib_name = get_lib_name()
    for start in ( "./", "../" ):
        for folder in ( "./", "artifacts/", "target/debug/", "target/release/" ):
            path = os.path.join(start, folder, lib_name)
            if os.path.isfile(path):
                return path
    raise FileNotFoundError("Could not find library file")

def try_loading_lib() -> Union[CDLL, WinDLL]:
    """Tries to load the lorecore library file

    Raises:
        OSError: If the library file could not be loaded

    Returns:
        Union[CDLL, WinDLL]: The loaded library file
    """

    if sys.platform.startswith('win'):
        WinDLL(find_lib_path())
    else:
        CDLL(find_lib_path())

# def main():
if __name__ == "__main__":
    my_funcs = try_loading_lib()
