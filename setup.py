from setuptools import setup, find_packages

setup(
    name="skt_tool",
    version="0.1.0",
    packages=find_packages(),
    include_package_data=True,      # Tells it to read MANIFEST.in
    install_requires=[
        "indic-transliteration",
    ],
    entry_points={
        "console_scripts":[
            "skt=skt_tool.cli:main", # Enables running `skt analyze prABavat`
        ]
    }
)