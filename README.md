# manglers

An ultra-basic file glitching tool

## WARNING

- Running this tool will **overwrite** a number of bytes in the given file with **random data**!
- **Do not run this tool on important files!**

## Running

`cargo run`

## How it works

- Assumes a file named `video.mp4` exists in the same directory it's being run (can you tell I was using this for video glitching?)
- Overwrites a small percentage of bytes in the file (currently 0.001%) with random bytes
- ... that's it!

## Playing with it

- Change `video.mp4` in the source to whatever file you want instead (or just name your file `video.mp4`!)
- Change `PERCENT_TWEAK`
  - 1.0 would change a number of bytes equal to the number of bytes in the file
  - 0.0 would make no changes

## Contribute

- Make it accept the input file name as a required argument
- Make it accept the percent_tweak as an optional argument
- Make it copy the original file to a .bak version (by default? with a flag to disable?) before mutating it
