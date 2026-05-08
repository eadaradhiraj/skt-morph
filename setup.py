# setup.py
from setuptools import setup, find_packages

setup(
    name="sktmorph",
    version="0.1.0",
    author="eadaradhiraj",
    description="Sanskrit Morphology Analyzer and Generator (SLP1)",
    packages=find_packages(exclude=["tests*", "scripts*", "data_raw*"]),
    include_package_data=True, # Important for bundling the SQLite DB
    package_data={
        "sktmorph": ["data/*.sqlite"],
    },
    entry_points={
        "console_scripts": [
            "sktmorph=sktmorph.cli:main",
        ],
    },
    python_requires=">=3.8",
)