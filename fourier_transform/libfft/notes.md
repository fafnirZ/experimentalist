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


### Continuing on DFT

$x_n -> X_k$ is a translation from configuration space to frequency space.

Looking at the forward discrete transform equation, it is a linear operation: a matrix vector multiplication of $\vec{X} = M.\vec{x}$

with matrix M given by:
$M_{kn} = e^{-i2\pi kn / N}$

### Explanation of all these components
$M_{kn}$ represents a specific element in the DFT matrix, `k` and `n` are indices, `k=row` `n=column`

$e$ is Euler's number (~2.71828)

$i$ imaginary number $\sqrt{-1}$

$\pi$.

$N$ size of input data being transformed (e.g. number of samples in a signal)

$k$ frequency index, representing a specific frequency component in the transformed output, ranges from 0 to N-1

$n$: time index, representing a specific sample in the input data. It also ranges from 0 to N-1


$-2\pi kn/N$ calculates an angle in radians:
- increases as k (frequency index) increases
- increases as n (time index) increases
- decreases as N (data size) increases

$e^{i2\pi kn/N}$ uses Euler's formula which states $e^{ix} = cos(x) + i sin(x)$ in our case x is the angle calculated in previous step.


## References
https://jakevdp.github.io/blog/2013/08/28/understanding-the-fft/#:~:text=The%20FFT%20is%20a%20fast,fftpack%20respectively.
