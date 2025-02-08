## Notes

### Forward Discrete Fourier Transform (DFT)
```math
X_k = \sum_{n=0}^{N-1} x_n * e^{-i2\pi kn / N}
```

### Inverse Discrete Fourier Transform (IDFT)

```math
x_n = 1/N \sum_{k=0}^{N-1}X_{k}e^{i2\pi kn / N}
```

#### Why is complex numbers involved?

since fourier transforms deals with signals i.e. waves, best represented by the sine and cosine graphs.
Its **amplitude** determines how tall the wave is and its **phase** determines where the wave starts.
Complex numbers provide a neat way to represent the two pieces of information in a single number.

- the magnitude of complex number corresponds to the amplitude
- the angle of the complex number corresponds to the phase.

```math
e^{ix} = cos(x) + isin(x)
```

##### Negative frequencies
complex numbers also allows us to represent "negative frequencies"?

In simpler terms, when trying to describe the motion of a swing, you need to know how high it goes (amplitude) and where it is at, at any given moment (phase)
complex numbers conveniently encode both to store these information in 1 package.



## References
https://jakevdp.github.io/blog/2013/08/28/understanding-the-fft/#:~:text=The%20FFT%20is%20a%20fast,fftpack%20respectively.
