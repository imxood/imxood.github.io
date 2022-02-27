from setuptools import setup, find_packages

setup(
    name="log_server",        # what you want to call the archive/egg
    version="0.1",
    description="log server",
    install_requires=['flask', 'humanize', 'pathlib2', 'python-magic'],
    packages=find_packages(),
    include_package_data=True,
    entry_points={
        "console_scripts": [        # command-line executables to expose
            "log_server=log_server.__main__:main",
        ],
        "gui_scripts": []
    },
    author="ma xu",
    author_email="imxood@163.com",
)
