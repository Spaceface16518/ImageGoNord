initSidebarItems({"fn":[["prepare_freethreaded_python","Prepares the use of Python in a free-threaded context."],["with_embedded_python_interpreter","Executes the provided closure with an embedded Python interpreter."]],"macro":[["create_exception","Defines a new exception type."],["create_exception_type_object","`impl $crate::type_object::PyTypeObject for $name` where `$name` is an exception newly defined in Rust code."],["impl_exception_boilerplate","The boilerplate to convert between a Rust type and a Python exception."],["import_exception","Defines a Rust type for an exception defined in Python code."],["py_run","A convenient macro to execute a Python code snippet, with some local variables set."],["pyobject_native_type","Declares all of the boilerplate for Python types which can be inherited from (because the exact Python layout is known)."],["pyobject_native_type_base",""],["pyobject_native_type_core","Declares all of the boilerplate for Python types."],["pyobject_native_type_extract",""],["pyobject_native_type_info",""],["pyobject_native_type_named",""],["pyobject_native_type_sized",""],["wrap_pyfunction","Returns a function that takes a [Python] instance and returns a Python function."],["wrap_pymodule","Returns a function that takes a [Python] instance and returns a Python module."]],"mod":[["buffer","`PyBuffer` implementation"],["class","Python object protocols"],["conversion","Conversions between various states of Rust and Python types and their wrappers."],["exceptions","Exception types defined by Python."],["ffi","Raw FFI declarations for Python’s C API."],["impl_","Internals of PyO3 which are accessed by code expanded from PyO3’s procedural macros. Usage of any of these APIs in downstream code is implicitly acknowledging that these APIs may change at any time without documentation in the CHANGELOG and without breaking semver guarantees."],["marshal","Support for the Python `marshal` format. Not supported in limited API builds."],["once_cell","A write-once cell mediated by the Python GIL."],["panic","Helper to convert Rust panics to Python exceptions."],["prelude","A collection of items you most likely want to have in scope when working with pyo3"],["proc_macro","The proc macros, all of which are part of the prelude."],["pycell","Includes `PyCell` implementation."],["pyclass","`PyClass` and related traits."],["pyclass_init","Initialization utilities for `#[pyclass]`."],["pyclass_slots","This module contains additional fields for `#[pyclass]`. Mainly used by PyO3’s proc-macro code."],["type_object","Python type object information"],["types","Various types defined by the Python interpreter such as `int`, `str` and `tuple`."]],"struct":[["GILGuard","RAII type that represents the Global Interpreter Lock acquisition."],["GILPool","A RAII pool which PyO3 uses to store owned Python references."],["Py","A GIL-independent reference to an object allocated on the Python heap."],["PyAny","Represents any Python object."],["PyDowncastError","Error that indicates a failure to convert a PyAny to a more specific Python type."],["PyErr","Represents a Python exception that was raised."],["Python","Marker type that indicates that the GIL is currently held."],["PythonVersionInfo","Represents the major, minor, and patch (if any) versions of this interpreter."]],"trait":[["PyErrArguments","Helper conversion trait that allows to use custom arguments for lazy exception construction."],["PyNativeType","Types that are built into the Python interpreter."]],"type":[["PyObject","A commonly-used alias for `Py<PyAny>`."],["PyResult","Represents the result of a Python call."]]});