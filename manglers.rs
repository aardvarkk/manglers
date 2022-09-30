use std::fs::OpenOptions;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;
use rand::prelude::*;

fn main() {
  const PERCENT_TWEAK: f32 = 0.00001;

  let mut file = OpenOptions::new().write(true).open("video.mp4").unwrap();
  let fsize = file.metadata().unwrap().len();
  let bytes_to_tweak = ((fsize as f32) * PERCENT_TWEAK).floor() as i32;

  println!("File Size: {0}", fsize);
  println!("Bytes To Tweak: {0}", bytes_to_tweak);

  let mut rng = rand::thread_rng();
  for _ in 0..bytes_to_tweak {
    let pos = rng.gen_range(0..fsize);
    let byte = rng.gen::<u8>();
    println!("Writing byte {0} at position {1}", byte, pos);

    file.seek(SeekFrom::Start(pos)).unwrap();
    file.write(&[byte]).unwrap();
  }
}
