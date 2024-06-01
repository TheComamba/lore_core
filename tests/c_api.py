import ctypes
import sys
import os
import platform
import tempfile

lib_path = ""
# Check if the script was called with the "--release" argument
if "--release" in sys.argv:
    lib_path = "./target/release/deps/"
else:
    lib_path = "./target/debug/deps/"

# Check the current operating system and append the corresponding library name
if os.name == 'nt':
    lib_path += 'lorecore.dll'
elif os.name == 'posix':
    if platform.system() == 'Darwin':  # macOS
        lib_path += 'liblorecore.dylib'
    else:
        lib_path += 'liblorecore.so'
else:
    raise Exception('Unsupported operating system')

# Load the Rust library
rust_lib = ctypes.CDLL(lib_path)

# Define the C structs
class CEntityColumn(ctypes.Structure):
    _fields_ = [("label", ctypes.c_char_p),
                ("descriptor", ctypes.c_char_p),
                ("description", ctypes.c_char_p)]

class CHistoryItem(ctypes.Structure):
    _fields_ = [("timestamp", ctypes.c_longlong),
                ("year", ctypes.c_int),
                ("day", ctypes.c_int),
                ("content", ctypes.c_char_p),
                ("properties", ctypes.c_char_p)]

class CEntityRelationship(ctypes.Structure):
    _fields_ = [("parent", ctypes.c_char_p),
                ("child", ctypes.c_char_p),
                ("role", ctypes.c_char_p)]

# Define the Rust functions
write_entity_columns = rust_lib.write_entity_columns
write_entity_columns.argtypes = [ctypes.c_char_p, ctypes.POINTER(CEntityColumn), ctypes.c_int]
write_entity_columns.restype = ctypes.c_char_p

get_number_of_entity_columns = rust_lib.get_number_of_entity_columns
get_number_of_entity_columns.argtypes = [ctypes.c_char_p, ctypes.POINTER(ctypes.c_int)]
get_number_of_entity_columns.restype = ctypes.c_char_p

read_entity_columns = rust_lib.read_entity_columns
read_entity_columns.argtypes = [ctypes.c_char_p, ctypes.POINTER(CEntityColumn)]
read_entity_columns.restype = ctypes.c_char_p

write_history_items = rust_lib.write_history_items
write_history_items.argtypes = [ctypes.c_char_p, ctypes.POINTER(CHistoryItem), ctypes.c_int]
write_history_items.restype = ctypes.c_char_p

get_number_of_history_items = rust_lib.get_number_of_history_items
get_number_of_history_items.argtypes = [ctypes.c_char_p, ctypes.POINTER(ctypes.c_int)]
get_number_of_history_items.restype = ctypes.c_char_p

read_history_items = rust_lib.read_history_items
read_history_items.argtypes = [ctypes.c_char_p, ctypes.POINTER(CHistoryItem)]
read_history_items.restype = ctypes.c_char_p

write_relationships = rust_lib.write_relationships
write_relationships.argtypes = [ctypes.c_char_p, ctypes.POINTER(CEntityRelationship), ctypes.c_int]
write_relationships.restype = ctypes.c_char_p

get_number_of_relationships = rust_lib.get_number_of_relationships
get_number_of_relationships.argtypes = [ctypes.c_char_p, ctypes.POINTER(ctypes.c_int)]
get_number_of_relationships.restype = ctypes.c_char_p

read_relationships = rust_lib.read_relationships
read_relationships.argtypes = [ctypes.c_char_p, ctypes.POINTER(CEntityRelationship)]
read_relationships.restype = ctypes.c_char_p

get_current_timestamp = rust_lib.get_current_timestamp
get_current_timestamp.argtypes = []
get_current_timestamp.restype = ctypes.c_longlong

def test_write_entity_column():
    temp_path = tempfile.NamedTemporaryFile(delete=False)
    db_path = temp_path.name.encode('utf-8')
    column1 = CEntityColumn(b"testlabel1", b"testdescriptor1", b"testdescription1")
    column2 = CEntityColumn(b"testlabel2", b"testdescriptor2", b"testdescription2")
    columns = (CEntityColumn * 2)(column1, column2)

    result = write_entity_columns(db_path, columns, len(columns))
    assert result.decode('utf-8') == ""

    size = ctypes.c_int(0)
    result = get_number_of_entity_columns(db_path, ctypes.byref(size))
    assert result.decode('utf-8') == ""
    assert size.value == len(columns)

    read_columns = (CEntityColumn * size.value)()
    result = read_entity_columns(db_path, read_columns)
    assert result.decode('utf-8') == ""

    temp_path.close()
test_write_entity_column()

def test_write_history_items():
    temp_path = tempfile.NamedTemporaryFile(delete=False)
    db_path = temp_path.name.encode('utf-8')
    item1 = CHistoryItem(get_current_timestamp(), 2021, 29, b"testcontent1", b"testproperties1")
    item2 = CHistoryItem(get_current_timestamp(), 2021, 30, b"testcontent2", b"testproperties2")
    items = (CHistoryItem * 2)(item1, item2)

    result = write_history_items(db_path, items, len(items))
    assert result.decode('utf-8') == ""

    size = ctypes.c_int(0)
    result = get_number_of_history_items(db_path, ctypes.byref(size))
    assert result.decode('utf-8') == ""
    assert size.value == len(items)

    read_items = (CHistoryItem * size.value)()
    result = read_history_items(db_path, read_items)
    assert result.decode('utf-8') == ""

    temp_path.close()
test_write_history_items()

def test_write_relationships():
    temp_path = tempfile.NamedTemporaryFile(delete=False)
    db_path = temp_path.name.encode('utf-8')
    relationship1 = CEntityRelationship(b"testparent1", b"testchild1", b"testrole1")
    relationship2 = CEntityRelationship(b"testparent2", b"testchild2", b"testrole2")
    relationships = (CEntityRelationship * 2)(relationship1, relationship2)

    result = write_relationships(db_path, relationships, len(relationships))
    assert result.decode('utf-8') == ""

    size = ctypes.c_int(0)
    result = get_number_of_relationships(db_path, ctypes.byref(size))
    assert result.decode('utf-8') == ""
    assert size.value == len(relationships)

    read_in_relationships = (CEntityRelationship * size.value)()
    result = read_relationships(db_path, read_in_relationships)
    assert result.decode('utf-8') == ""

    temp_path.close()
test_write_relationships()

def test_get_current_timestamp():
    timestamp = get_current_timestamp()
    assert isinstance(timestamp, int)
test_get_current_timestamp()
