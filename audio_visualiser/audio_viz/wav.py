"""
Most code here is copied or heavily influenced from:
https://github.com/WarrenWeckesser/wavio/blob/main/wavio.py
-----
Author: Warren Weckesser
License: BSD 2-Clause:
Copyright (c) 2015-2022, Warren Weckesser
All rights reserved.
"""

from __future__ import annotations
from dataclasses import dataclass
from typing import BinaryIO
import wave as _wave
import numpy as _np


@dataclass
class Wav:
    raw_data: bytes
    samplerate: int
    sampwidth: int
    nchannels: int
    nframes: int

    # underlying wave reader from stdlib
    _wav: _wave.Wave_read

    @classmethod
    def read(cls, *, file: BinaryIO | str) -> Wav:
        with _wave.open(file) as wav:
            nchannels = wav.getnchannels()
            samplerate = wav.getframerate()
            sampwidth = wav.getsampwidth()
            nframes = wav.getnframes()
            data = wav.readframes(nframes)

        return cls(
            raw_data=data,
            samplerate=samplerate,
            sampwidth=sampwidth,
            nchannels=nchannels,
            nframes=nframes,
            _wav=wav,
        )

    @staticmethod
    def _wav2array(nchannels: int, sampwidth: int, data: bytes) -> _np.ndarray:
        """data must be the string containing the bytes from the wav file."""
        num_samples, remainder = divmod(len(data), sampwidth * nchannels)
        if remainder > 0:
            raise ValueError(
                "The length of data is not a multiple of " "sampwidth * num_channels."
            )
        if sampwidth > 4:
            raise ValueError("sampwidth must not be greater than 4.")

        if sampwidth == 3:
            a = _np.empty((num_samples, nchannels, 4), dtype=_np.uint8)
            raw_bytes = _np.frombuffer(data, dtype=_np.uint8)
            a[:, :, :sampwidth] = raw_bytes.reshape(-1, nchannels, sampwidth)
            a[:, :, sampwidth:] = (a[:, :, sampwidth - 1 : sampwidth] >> 7) * 255
            result = a.view("<i4").reshape(a.shape[:-1])
        else:
            # 8 bit samples are stored as unsigned ints; others as signed ints.
            dt_char = "u" if sampwidth == 1 else "i"
            a = _np.frombuffer(data, dtype="<%s%d" % (dt_char, sampwidth))
            result = a.reshape(-1, nchannels)
        return result

    @property
    def sound_duration(self) -> float:
        return self.nframes / self.samplerate

    @property
    def channels_sliced_data(self) -> _np.ndarray:
        return self._wav2array(self.nchannels, self.sampwidth, self.raw_data)
