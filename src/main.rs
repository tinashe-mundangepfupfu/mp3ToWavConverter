use minimp3::{Decoder, Frame};
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <input.mp3> <output.wav>", args[0]);
        std::process::exit(1);
    }

    let input_file = &args[1];
    let output_file = &args[2];

    // Read the MP3 file
    let mut mp3_data = Vec::new();
    let mut file = File::open(&input_file).expect("Failed to open input file");
    file.read_to_end(&mut mp3_data).expect("Failed to read input file");

    // Create the MP3 decoder
    let mut decoder = Decoder::new(mp3_data.as_slice());

    let loop_duration = 10.0; // Duration in seconds to play output sound
    let mut samples_written = 0;

    let mut decoded_data = Vec::new();
    let mut sample_rate = 0;
    let mut channels = 0;

    // Decode MP3 frames
    loop {
        match decoder.next_frame() {
            Ok(Frame { data, sample_rate: frame_sample_rate, channels: frame_channels, .. }) => {
                if sample_rate == 0 {
                    sample_rate = frame_sample_rate;
                    channels = frame_channels;
                }
                decoded_data.extend_from_slice(&data);
            }
            Err(minimp3::Error::Eof) => break,
            Err(e) => {
                eprintln!("Error decoding MP3: {:?}", e);
                std::process::exit(1);
            }
        }
    }

    // Apply fade-in and fade-out effects
    let fade_duration = 1.0; // Fade duration in seconds
    apply_fade_in_fade_out(&mut decoded_data, channels, fade_duration, sample_rate);


    // Calculate target samples count for looped output
    let target_samples = (loop_duration * sample_rate as f64 * channels as f64).ceil() as usize;

    // Initialize WAV writer
    let spec = hound::WavSpec {
        channels: channels as _,
        sample_rate: sample_rate as _,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let path = Path::new(output_file);
    let mut wav_writer = hound::WavWriter::create(path, spec).expect("Failed to create WAV file");

    // Write samples to the WAV file, looping until the desired length is reached
    while samples_written < target_samples {
        for sample in &decoded_data {
            wav_writer.write_sample(*sample).expect("Failed to write to WAV file");
            samples_written += 1;
            if samples_written >= target_samples {
                break;
            }
        }
    }

    println!("Conversion completed.");
}

// Function that implements fade in and fade out to reduce the gap between the looping sounds.
// This example applies a 100ms (0.1s) linear fade-in at the beginning and a 100ms linear fade-out at
// the end of the audio. The apply_fade_in_fade_out function scales the audio samples by a linear factor
// that goes from 0 to 1 for the fade-in and from 1 to 0 for the fade-out.
//
// After making these changes, the gap between looping sounds should be less noticeable. However,
// note that this simple approach might not produce seamless loops in all cases, especially when the audio waveform is not continuous at the loop points.
// You can experiment with different fade durations and types of fade curves (e.g., logarithmic, exponential) to improve the results.

fn apply_fade_in_fade_out(data: &mut Vec<i16>, channels: usize, fade_duration: f64, sample_rate: i32) {
    let fade_samples = (fade_duration * sample_rate as f64).ceil() as usize;

    for i in 0..fade_samples {
        let factor = i as f64 / fade_samples as f64;
        for channel in 0..channels {
            let idx = i * channels + channel;
            data[idx] = (data[idx] as f64 * factor).round() as i16;
        }
    }

    let total_samples = data.len() / channels;
    for i in (total_samples - fade_samples..total_samples).rev() {
        let factor = (total_samples - i) as f64 / fade_samples as f64;
        for channel in 0..channels {
            let idx = i * channels + channel;
            data[idx] = (data[idx] as f64 * factor).round() as i16;
        }
    }
}

