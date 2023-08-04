## Cuesheet-rs

The rust implementation of [Cue_sheet_maker](https://github.com/EruEri/cue-sheet-maker/)

A library to create cue sheet

# Example
A simple example on how to create a cuesheet
```rust
        let mut track = CueTrack::new(1, crate::CueTrackMode::AUDIO);
        let _ = track
            .add_performer("A performer")
            .add_composer("A composer")
            .add_title("A title")
            .add_index(1, crate::DurationFormat::MinSecMil(3, 30, 300));

        let mut track2 = CueTrack::new(2, crate::CueTrackMode::AUDIO);
        let _ = track2
            .add_performer("A performer 2")
            .add_composer("A composer 2")
            .add_title("A title 2")
            .add_index(1, crate::DurationFormat::MinSecMil(3, 30, 300));

        let mut sheet = CueSheet::new("A file", crate::CueFileFormat::WAVE);
        let _ = sheet
            .add_catalog("0123456789123")
            .add_performer("Album performer")
            .add_title("Album title")
            .add_track(track)
            .add_track(track2);

        let _ = sheet.export(true, "output.cue");
```

Produce

```
CATALOG 0123456789123
PERFORMER "Album performer"
TITLE "Album title"
FILE "A file" WAVE
  TRACK 01 AUDIO
    COMPOSER "A composer"
    PERFORMER "A performer"
    TITLE "A title"
    INDEX 01 00:00:00
  TRACK 02 AUDIO
    COMPOSER "A composer 2"
    PERFORMER "A performer 2"
    TITLE "A title 2"
    INDEX 01 03:30:22
```