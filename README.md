# musicgen

Generate synthetic music!

## Usage

Just build and run it, there are no parameters as of now.
Saves the generated file to `./output/gen_[seed].wav`.
The music is automatically being played back after the file has been saved.

## Dependencies

- `fundsp` for audio synthesis.
- `rand` for randomness.
- `cpal` for audio playback.
- `anyhow` for errors.
