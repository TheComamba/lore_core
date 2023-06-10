import sys
import os
from ctypes import CDLL, WinDLL
from typing import Union

def get_lib_name(base_path: str) -> Union[CDLL, WinDLL]:
    """Check the platform the code currently is running on and return the corresponding liblorecore file

    Raises:
        ValueError: If the platform is neither Windows, Lunux, nor Mac

    Returns:
        str: file name of the liblorecore
    """
    if sys.platform.startswith('win'):
        # Windows-specific code
        c_file_path = "liblorecore.dll"
        full_path = os.path.join(base_path, c_file_path)
        return WinDLL(full_path)
    if sys.platform.startswith('linux'):
        # Linux-specific code
        c_file_path = "liblorecore.so"
        full_path = os.path.join(base_path, c_file_path)
        return CDLL(full_path)
    if sys.platform.startswith('darwin'):
        # macOS-specific code
        c_file_path = "liblorecore.dylib"
        full_path = os.path.join(base_path, c_file_path)
        return CDLL(full_path)
    # Code for other or unknown operating systems
    raise ValueError("Unsupported OS")

def try_loading_lib() -> Union[CDLL, WinDLL]:
    """Goes through all possible path locations and tries to find the liblorecore file

    Raises:
        FileNotFoundError: If the file does not exist in any of the expected locations

    Returns:
        CDLL: object containing all the C functions
    """
    for start in ( "./", "../" ):
        for path in ( "./", "artifacts/", "target/debug/", "target/release/" ):
            base_path = os.path.join(start, path)
            if os.path.isfile(base_path):
                return get_lib_name(base_path)
    raise FileNotFoundError("Could not find library file")

# def main():
if __name__ == "__main__":
    my_funcs = try_loading_lib()
