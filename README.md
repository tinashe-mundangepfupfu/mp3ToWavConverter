# mp3ToWavConverter

A Rust tool that converts MP3 files to WAV files, I use the minimp3 crate for MP3 decoding and the hound crate for WAV encoding.

It reads the input MP3 file, decodes the audio data, and writes the audio samples to a WAV file. 

Note that it expects two command-line arguments: the input MP3 file and the output WAV file.

You can now build and run the program with the following commands:

```
$ cargo build
$ cargo run -- input.mp3 output.wav
```

Make sure to replace input.mp3 and output.wav with the appropriate file names for your use case. 
The program will decode the MP3 file and save the output as a WAV file.

# TODO 

Add options to change the fade curve. I will need to update the fade-in and fade-out calculations in the apply_fade_in_fade_out function. 
I can modify the factor variable in the loops to achieve different fade curve types. Add examples for logarithmic and exponential. 
