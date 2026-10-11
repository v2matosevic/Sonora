# Media credits and what the pictures show

Sonora's code and its own artwork are MIT licensed. The music and the cover picture used in the
README pictures are third-party works with their own terms, listed here with where each came from.
Nothing from the media is committed to this repository except the pictures themselves.
Everything below was checked on 2026-10-11.

## What each picture is

| Picture | Kind | What it shows |
| --- | --- | --- |
| `docs/media/hero.jpg` | Mockup | The captures below placed in a drawn Windows-style window frame, over a blurred copy of the cover picture. It is not a screenshot. |
| `docs/screenshots/now-playing.png`, `crossfade-settings.png`, `crossfade-in-progress.png`, `light-theme.png` | Staged scenes rendered by Sonora's own interface | Sonora 0.2.1's real interface code, drawn offscreen by `apps/sonora/tests/showcase.rs`, with the facts of real files: title, artist, album, length, stream format and the embedded cover are read from the files by Sonora's own decoder. Staged by the test: the playback position, the crossfade state and a 48 kHz output device, because a still image cannot hold a running player. No audio was played. They are not photographs of a running window. |
| `docs/screenshots/sonora.png` | Earlier screenshot | The first public screenshot, made with generated placeholder media. Kept for the record, no longer shown in the README. |

## Music

All six demonstration tracks are one recording.

- Recording: *J.S. Bach: "Open" Goldberg Variations, BWV 988 (Piano)*, Kimiko Ishizaka (piano),
  Johann Sebastian Bach (composer), released 28 May 2012.
- Tracks used: 1 Aria, 2 Variatio 1 a 1 Clav., 3 Variatio 2 a 1 Clav., 4 Variatio 3 a 1 Clav.
  Canone all Unisuono, 5 Variatio 4 a 1 Clav., 8 Variatio 7 a 1 ovvero 2 Clav.
- License: Creative Commons CC0 1.0. The artist's own page says "All tracks are provided without
  copyright (Creative Commons Zero). All uses permitted."
  Primary sources: [the album on Kimiko Ishizaka's Bandcamp](https://kimikoishizaka.bandcamp.com/album/j-s-bach-open-goldberg-variations-bwv-988-piano)
  and the [Open Goldberg Variations project page](https://opengoldbergvariations.org/).
- Where the files came from: the MP3 copies on Wikimedia Commons, each file page marked CC0
  (read through the Commons API on the same day, and the first track's page by eye):
  [01 Aria](https://commons.wikimedia.org/wiki/File:Kimiko_Ishizaka_-_J.S._Bach-_-Open-_Goldberg_Variations,_BWV_988_%28Piano%29_-_01_Aria.mp3),
  [02](https://commons.wikimedia.org/wiki/File:Kimiko_Ishizaka_-_J.S._Bach-_-Open-_Goldberg_Variations,_BWV_988_%28Piano%29_-_02_Variatio_1_a_1_Clav.mp3),
  [03](https://commons.wikimedia.org/wiki/File:Kimiko_Ishizaka_-_J.S._Bach-_-Open-_Goldberg_Variations,_BWV_988_%28Piano%29_-_03_Variatio_2_a_1_Clav.mp3),
  [04](https://commons.wikimedia.org/wiki/File:Kimiko_Ishizaka_-_J.S._Bach-_-Open-_Goldberg_Variations,_BWV_988_%28Piano%29_-_04_Variatio_3_a_1_Clav._Canone_all_Unisuono.mp3),
  [05](https://commons.wikimedia.org/wiki/File:Kimiko_Ishizaka_-_J.S._Bach-_-Open-_Goldberg_Variations,_BWV_988_%28Piano%29_-_05_Variatio_4_a_1_Clav.mp3),
  [08](https://commons.wikimedia.org/wiki/File:Kimiko_Ishizaka_-_J.S._Bach-_-Open-_Goldberg_Variations,_BWV_988_%28Piano%29_-_08_Variatio_7_a_1_ovvero_2_Clav.mp3).
- Changes: the files were copied, re-tagged with the cover below and renamed (`ffmpeg -c copy`, the
  audio data is untouched). The audio is not published here.

| Local original (not committed) | SHA-256 |
| --- | --- |
| `01-aria.mp3` | `1b6c7d606d420f1b5b062b4fff8b1eb4f9c7efe11abd35bc33b2db8be7f35b73` |
| `02-variatio-1.mp3` | `40d52699d6064766e0d9857ba435b5c7aea53ecc46c3ce32a87ff7e82da4a639` |
| `03-variatio-2.mp3` | `2f5a6b4715db43af6f96b421ab22db223dbf7b2e37d8dc98a1f596c194041629` |
| `04-variatio-3.mp3` | `cdee9839928b626b0ed01300acbe0e5cfd7b6eea8f209c72f2d907d1e4b12a00` |
| `05-variatio-4.mp3` | `1c38912f9bca45fae689fb9d8b25f37577309f33a29bb8cd6f93f6c939dbb094` |
| `08-variatio-7.mp3` | `cebf7866dde5590f7e702abe603b68ad3c097f32793f862c0dcdc7feb323a5ee` |

## Cover picture

- Portrait of Johann Sebastian Bach by Elias Gottlob Haussmann, 1748 (second version of the 1746
  canvas), oil on canvas, Bach-Archiv Leipzig. Public domain: the painter died in 1774.
  [File on Wikimedia Commons](https://commons.wikimedia.org/wiki/File:Johann_Sebastian_Bach.jpg),
  1376 x 1786 px, SHA-256 `f57aa957609b859be8f1a45fed054bbf1022ac81e13b158f663c0c4f67f8b71b`.
- Changes: cropped square and embedded as the cover of the demonstration copies.
- This is not the album's own cover. Kimiko Ishizaka's official cover is a photograph of the pianist,
  and her page declares the tracks, not the artwork, to be CC0, so it is not used. A recording's
  license does not automatically cover its cover picture.

## Reproducing the pictures

The media stays in the ignored folder `demo-media/` (`originals`, `queue`, `captures`, `work`).

1. Download the six tracks and the portrait from the links above into `demo-media/originals`.
2. Crop the portrait to a 1000 x 1000 square, save it as `demo-media/work/cover-1000.jpg`, and embed
   it: `ffmpeg -i originals/01-aria.mp3 -i work/cover-1000.jpg -map 0:a -map 1:v -c copy -id3v2_version 3 "queue/01 Aria.mp3"`
   (the same for the other five).
3. Render the scenes: in `apps/sonora`, with `SONORA_SHOWCASE_MEDIA` and `SONORA_SHOWCASE_OUT` set to
   the `queue` and `captures` folders, run `cargo test --test showcase -- --ignored`.
4. Compose the hero: `python scripts/showcase/compose-hero.py` (needs Pillow and numpy).
