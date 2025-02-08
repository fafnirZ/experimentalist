from pydub import AudioSegment
import argparse
from pathlib import Path


parser = argparse.ArgumentParser(
                    prog='ProgramName',
                    description='What the program does',
                    epilog='Text at the bottom of help')


parser.add_argument("file_path")
args = parser.parse_args()

file_path = args.file_path
extension = Path(file_path).suffix
replacement_extension = ".wav"
replacement_path_str = file_path.replace(extension, replacement_extension)

# conversion
sound = AudioSegment.from_mp3(file_path)
sound.export(replacement_path_str, format="wav")