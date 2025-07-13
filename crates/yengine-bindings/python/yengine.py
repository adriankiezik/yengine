import ctypes
import platform
from pathlib import Path

class Engine:
    def __init__(self):
        system = platform.system().lower()
        if system == "windows":
            lib_name = "yengine_bindings.dll"
        elif system == "darwin":
            lib_name = "libyengine_bindings.dylib"
        else:
            lib_name = "libyengine_bindings.so"

        project_root = Path(__file__).parent.parent.parent.parent
        lib_path = project_root / "target" / "release" / lib_name
        
        if not lib_path.exists():
            raise FileNotFoundError(f"Library not found at {lib_path}. Run 'cargo build --release' first.")
        
        self._lib = ctypes.CDLL(str(lib_path))

        self._lib.create_engine.restype = ctypes.c_void_p
        self._lib.run_engine.argtypes = [ctypes.c_void_p]
        self._lib.run_engine.restype = ctypes.c_int
        self._lib.shutdown_engine.argtypes = [ctypes.c_void_p]

        self._handle = self._lib.create_engine()

    def run(self):
        return self._lib.run_engine(self._handle) == 0

    def shutdown(self):
        if self._handle:
            self._lib.shutdown_engine(self._handle)
            self._handle = None
