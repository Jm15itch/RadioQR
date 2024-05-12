use std::io::Cursor;
use rodio::{Decoder, OutputStream, source::Source};

mod math;
use math::sine_lut;

pub fn tone_from_str(s: &str, rate: usize) -> Box<[i16]> {

    // Duration for each nibble tone in ms
    const NIBBLE_DUR: usize = 2;

    let samples_per_ms: usize = rate/1000; 

    // the rate that the sine lookup table should be sampled
    let sine_rate: usize = match rate {
        48000 => 1,
        _ => 48000/rate,
    };

    // We define 250 milliseconds for the positional marker
    // and 5ms each for nibble (1/2 byte)
    let len = (samples_per_ms * (100 + s.as_bytes().len() * NIBBLE_DUR * 2));

    // Build the buffer that will be used to store audio data
    let mut output: Box<[i16]> = 
        (0..len)
        .map(|_| 0)
        .collect::<Box<[i16]>>();

    for id in 0..5 {
        let freq: usize = match id {
           1 => { 5000 }, 2 => { 3000 }, 3 => { 500 }, _ => { 1000 },   
        };
        for d in 0..(20*samples_per_ms) {
            output[id*(20*samples_per_ms) + d] = sine_lut[(sine_rate * freq * d) % 48000];
        } 
    }

    for i in 0..(s.as_bytes().len()*2) {
        let freq =  match match i%2 {
            0 => {
                s.as_bytes()[i/2] & 0b11110000
            }
            _ => {
                s.as_bytes()[i/2] >> 4
            }
        } {
            0 => 260,
            1 => 330,
            2 => 415,
            3 => 525,
            4 => 660,
            5 => 830,
            6 => 1050,
            7 => 1320,
            8 => 1660,
            9 => 2100,
            10 => 2640,
            11 => 3320,
            12 => 4180,
            13 => 5280,
            14 => 6644,
            15 => 7040,
            _ => 7902,
        };
        for d in 0..(NIBBLE_DUR*samples_per_ms) {
            output[100 * samples_per_ms + i*NIBBLE_DUR*samples_per_ms + d] = sine_lut[(sine_rate * freq * d) % 48000];
        }
    }

    output
}


pub fn str_from_tone(samples: &[u8], rate: usize) -> Option<String> {
    todo!()
}


fn main() {
    let tone = tone_from_str("This is an example of RadioQR encoding a string into a tonal sound", 48000);


    let mut G: Vec<f32> = Vec::new();

    for x in tone.iter() {
        G.push(*x as f32);
    }

    // Rodio testing
    let source = rodio::buffer::SamplesBuffer::new(1, 48000, G);

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    stream_handle.play_raw(source.convert_samples());

    loop {
        
    }
}
