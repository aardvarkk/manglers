use std::fs::OpenOptions;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;

fn main() {
  let mut file = OpenOptions::new().write(true).open("video.mp4").unwrap();
  file.seek(SeekFrom::Start(0));
  file.write(&[0x12]);
}
