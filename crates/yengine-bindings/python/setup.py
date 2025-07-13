from setuptools import setup

setup(
    name="yengine",
    version="0.1.0",
    py_modules=["yengine"],
    package_data={"": ["*.dll", "*.so", "*.dylib"]},
    include_package_data=True,
)
