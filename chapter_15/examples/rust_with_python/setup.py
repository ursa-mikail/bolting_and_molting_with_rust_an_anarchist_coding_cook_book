from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    name="rust_with_python",
    version="0.1.0",
    rust_extensions=[RustExtension("rust_with_python", binding=Binding.PyO3)],
    packages=["rust_with_python"],
    # rust extensions are not zip safe
    zip_safe=False,
    # Require Python 3.7 or higher
    python_requires=">=3.7",
    # Add NumPy as a dependency
    install_requires=["numpy>=1.20.0"],
)