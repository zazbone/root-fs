from logging import root
import uproot
import numpy
from pathlib import Path

x = numpy.linspace(-10, 10, 1024)
with uproot.recreate(Path(__file__).parent / "dataset.root") as root_file:
    root_file["zeros"] = {"a": numpy.zeros(1024)}
    root_file["function/simple"] = {
        "x": x,
        "constant": numpy.full(1024, 0),
        "linear": 0.1 * x,
        "quad": x ** 2 - 0.4 * x,
    }
    root_file["function/random/distrib"] = {
        "integers0_10": numpy.random.randint(10, size=1024),
        "random": numpy.random.random(1024)
    }