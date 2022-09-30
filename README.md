# manglers

An ultra-basic file glitching tool

## Before & After

https://user-images.githubusercontent.com/1251092/193210373-a75f7bc6-13fc-49b9-894d-8332c72bb935.mp4

https://user-images.githubusercontent.com/1251092/193210386-2e237c6b-9d6b-461a-a6d7-5c94523b3771.mp4

## WARNING

- Running this tool will **overwrite** a number of bytes in the given file with **random data**!
- **Do not run this tool on important files!**

## Running

`cargo run`

## How it works

- Assumes a file named `video.mp4` exists in the same directory it's being run
- Overwrites a small percentage of bytes in the file (currently 0.001%) with random bytes
- ... that's it!

## Playing with it

- Change `INPUT_FILE` in the source to whatever file you want instead (or just name your file `video.mp4`!)
- Change `PERCENT_TWEAK`
  - 1.0 would change a number of bytes equal to the number of bytes in the file
  - 0.0 would make no changes
- Try it on some photos! Or anything you're excited to corrupt!

## Contribute

- Make it accept the input file name as a required argument
- Make it accept the percent tweak as an optional argument
- Make it copy the original file to a .bak version (by default? with a flag to disable?) before mutating it
