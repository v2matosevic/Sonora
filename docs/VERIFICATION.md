# Sonora 0.2.1 verification

Verified on Windows 11 x64. Scoped evidence is not a guarantee for every file, GPU or Windows configuration.

| App | Installed release checks |
| --- | --- |
| Sonora | 14 audio fixtures, five engine programmes, Windows media-session readback, four actual Windows device workloads; FLAC-to-MP3 crossfade had zero measured underruns |

All installed payload files match their manifests. Start-menu and uninstall records report 0.2.1. Portable archives passed CRC and every manifest hash. Strict release lint/build checks passed locally.

All 110 code tests, including rendering and keyboard snapshots, passed on the development PC for 0.2.1. A hosted Windows snapshot run initially aborted in its native GPU test renderer; a subsequent complete hosted run passed all checks. The public workflow serializes tests to avoid concurrent GPU snapshot setup. Application playback uses its native OpenGL renderer, independently checked by installed-binary workloads.

Not verified: owner-operated drag/drop, physical media keys/clipboard/fullscreen/mixed-DPI, long listening/A-V sync, perceptual quality, HDR display output, every rare profile or another GPU. The native computer-control helper was unavailable. Screenshots show actual UI with generated demonstration media.
