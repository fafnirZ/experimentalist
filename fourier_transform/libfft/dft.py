"""
https://jakevdp.github.io/blog/2013/08/28/understanding-the-fft/#:~:text=The%20FFT%20is%20a%20fast,fftpack%20respectively.

note numpy.fft and scipy.fftpack are the fastest implementations.
regardless, this is for educational purposes.
"""

from abc import ABC, abstractmethod
from typing import Any


class DiscreteFourierTransform(ABC):
    
    @abstractmethod
    @classmethod
    def calculate(input: Any):
        pass

class ForwardDFT(DiscreteFourierTransform):

    @classmethod
    def calculate(input: Any):
        pass


class InverseDFT(DiscreteFourierTransform):

    @classmethod
    def calculate(input):
        pass