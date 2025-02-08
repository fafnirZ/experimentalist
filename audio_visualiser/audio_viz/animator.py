"""
https://realpython.com/python-wav-files/#animate-the-waveform-graph-in-real-time
"""

from typing import Any, Generator, Iterable
import numpy as np
from audio_viz.wav import Wav
import matplotlib.pyplot as plt


class Animator:
    @staticmethod
    def slide_window(window_seconds: int, wav: Wav) -> Generator[np.ndarray, Any, Any]:
        num_windows = round(wav.sound_duration / window_seconds)
        for i in range(num_windows):
            begin_seconds = i * window_seconds
            end_seconds = begin_seconds + window_seconds
            channels = wav.channels_sliced_data[begin_seconds:end_seconds]
            yield np.mean(tuple(channels), axis=0)

    @staticmethod
    def animate(windows: Iterable[np.ndarray]):
        fig, ax = plt.subplots(figsize=(16, 9))
        fig.canvas.manager.set_window_title("FILENAME")

        plt.tight_layout()
        plt.box(False)

        for window in windows:
            plt.cla()
            ax.set_xticks([])
            ax.set_yticks([])
            ax.set_ylim(-1.0, 1.0)
            plt.plot(window)
            # plt.pause(seconds)
