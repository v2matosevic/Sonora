# Sonora 0.2.1 verification

Verified on Windows 11 x64. Scoped evidence is not a guarantee for every file, GPU or Windows configuration.

| App | Installed release checks |
| --- | --- |
| Sonora | 14 audio fixtures, five engine programmes, Windows media-session readback, four actual Windows device workloads; FLAC-to-MP3 crossfade had zero measured underruns |

All installed payload files match their manifests. Start-menu and uninstall records report 0.2.1. Portable archives passed CRC and every manifest hash. Strict release lint/build checks passed locally.

The 110 code tests passed on the development PC before metadata/licensing changes. Hosted Windows GPU snapshot rendering encountered a native renderer abort; CI and local GPU results are documented separately rather than claiming a hosted snapshot pass.

Not verified: owner-operated drag/drop, physical media keys/clipboard/fullscreen/mixed-DPI, long listening/A-V sync, perceptual quality, HDR display output, every rare profile or another GPU. The native computer-control helper was unavailable. Screenshots show actual UI with generated demonstration media.
