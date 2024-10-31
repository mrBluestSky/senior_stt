//! Run with:
//! cargo run --example read_wav <model path> <wav path>
//! e.g. "cargo run --example read_wav /home/user/stt/model /home/user/stt/test.wav"
//! (The WAV file must have signed 16-bit samples)
//!
//! Read the "Setup" section in the README to know how to link the vosk dynamic
//! libaries to the examples

use std::env;
use hound::WavReader;
use vosk::{Model, Recognizer};
use std::time::Duration;
use std::io::{self, Write};
use std::thread;

fn main() {
    vosk::set_log_level(vosk::LogLevel::Error);
    let mut args = env::args();
    args.next();

    let model_path = args.next().expect("A model path was not provided");
    let wav_path = args
        .next()
        .expect("A path for the wav file to be read was not provided");

    let mut reader = WavReader::open(wav_path).expect("Could not create the WAV reader");
    let samples = reader
        .samples()
        .collect::<hound::Result<Vec<i16>>>()
        .expect("Could not read WAV file");

    let model = Model::new(model_path).expect("Could not create the model");
    let mut recognizer = Recognizer::new(&model, reader.spec().sample_rate as f32)
        .expect("Could not create the recognizer");

    recognizer.set_max_alternatives(0);
    recognizer.set_words(false);
    recognizer.set_partial_words(false);

    for sample in samples.chunks(100) {
        recognizer.accept_waveform(sample);
        let _ = recognizer.partial_result();
    }
    let long_output = match recognizer.final_result().single() {
        Some(x) => {
            x.text.to_string()
        },
        None => {
            "ERROR: output is broken".to_string()
        },
    };
    println!("{:#?}", long_output);
    
    for output in long_output.split_whitespace() {
        // write the STT result into the Serial Port so that ESP32 can transmit it
        let mut port = serialport::new("/dev/ttyUSB0", 115_200)
            .timeout(Duration::from_millis(10))
            .open().expect("Failed to open port");

        // write to a port
        port.write(output.as_bytes()).expect("Write failed!");
        // wait until ESP32 does its job
        thread::sleep_ms(2500);

        // reading from a port
        let mut serial_buf: Vec<u8> = vec![0; 32];
        let t = port.read(serial_buf.as_mut_slice()).expect("Found no data!");
        io::stdout().write_all(&serial_buf[..t]).unwrap();
        io::stdout().flush().unwrap();
        println!("");

    }
}


