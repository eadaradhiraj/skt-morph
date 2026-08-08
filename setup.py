from setuptools import find_packages, setup

ROOT = __import__("pathlib").Path(__file__).parent
readme = (ROOT / "README.MD").read_text(encoding="utf-8")

setup(
    name="sktmorph",
    author="eadaradhiraj",
    description="Sanskrit Morphology Analyzer and Generator (SLP1)",
    long_description=readme,
    long_description_content_type="text/markdown",
    url="https://github.com/eadaradhiraj/skt-morph",
    license="MIT",
    packages=find_packages(exclude=["tests*", "scripts*", "data_raw*"]),
    include_package_data=True,
    package_data={"sktmorph": ["data/*.sqlite"]},
    entry_points={"console_scripts": ["sktmorph=sktmorph.cli:main"]},
    python_requires=">=3.8",
    extras_require={
        "devanagari": ["indic-transliteration>=2.3"],
        "dev": ["pytest>=7.0", "coverage>=7.0", "indic-transliteration>=2.3"],
    },
)
