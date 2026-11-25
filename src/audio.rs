//! Simple sound playback on SDL3's core audio streams (no extra native libraries).
//!
//! Sounds are WAV files. Each voice is an `SDL_AudioStream` bound to the default playback
//! device, which gives per-voice gain and pitch (frequency ratio) for free.

use std::ffi::CString;
use std::path::Path;

use sdl3::sys::audio as sa;
use sdl3::AudioSubsystem;

/// Handle to a sound loaded into [`Audio`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SoundId(usize);

/// Handle to one sound-effect playback.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Voice {
    slot: usize,
    generation: u64,
}

struct Sound {
    spec: sa::SDL_AudioSpec,
    data: Vec<u8>,
    /// Base gain applied to every playback of this sound.
    volume: f32,
}

struct Stream {
    raw: *mut sa::SDL_AudioStream,
    sound: Option<SoundId>,
    generation: u64,
}

impl Stream {
    fn new(device: sa::SDL_AudioDeviceID) -> Result<Self, String> {
        // Any source format works here; it is replaced before each playback.
        let spec = sa::SDL_AudioSpec {
            format: sa::SDL_AUDIO_S16LE,
            channels: 2,
            freq: 44100,
        };
        let raw = unsafe { sa::SDL_CreateAudioStream(&spec, std::ptr::null()) };
        if raw.is_null() {
            return Err(sdl3::get_error().to_string());
        }
        if !unsafe { sa::SDL_BindAudioStream(device, raw) } {
            unsafe { sa::SDL_DestroyAudioStream(raw) };
            return Err(sdl3::get_error().to_string());
        }
        Ok(Self {
            raw,
            sound: None,
            generation: 0,
        })
    }

    fn queued(&self) -> i32 {
        unsafe { sa::SDL_GetAudioStreamQueued(self.raw) }
    }

    fn playing(&self) -> bool {
        self.sound.is_some() && self.queued() > 0
    }

    fn stop(&mut self) {
        unsafe { sa::SDL_ClearAudioStream(self.raw) };
        self.sound = None;
    }

    fn start(&mut self, id: SoundId, sound: &Sound, gain: f32, pitch: f32) {
        unsafe {
            sa::SDL_ClearAudioStream(self.raw);
            sa::SDL_SetAudioStreamFormat(self.raw, &sound.spec, std::ptr::null());
            sa::SDL_SetAudioStreamGain(self.raw, gain.max(0.0));
            sa::SDL_SetAudioStreamFrequencyRatio(self.raw, pitch.clamp(0.01, 100.0));
        }
        self.put(sound);
        self.sound = Some(id);
    }

    fn put(&self, sound: &Sound) {
        unsafe {
            sa::SDL_PutAudioStreamData(
                self.raw,
                sound.data.as_ptr().cast(),
                sound.data.len() as i32,
            );
        }
    }
}

impl Drop for Stream {
    fn drop(&mut self) {
        unsafe { sa::SDL_DestroyAudioStream(self.raw) };
    }
}

/// Sound effects plus one looping music track.
pub struct Audio {
    device: sa::SDL_AudioDeviceID,
    sounds: Vec<Sound>,
    sfx: Vec<Stream>,
    next_sfx: usize,
    generation: u64,
    music: Stream,
    _subsystem: AudioSubsystem,
}

impl Audio {
    /// Open the default playback device with `voices` simultaneous sound effects.
    pub fn new(sdl: &sdl3::Sdl, voices: usize) -> Result<Self, String> {
        let subsystem = sdl.audio().map_err(|e| e.to_string())?;
        let device =
            unsafe { sa::SDL_OpenAudioDevice(sa::SDL_AUDIO_DEVICE_DEFAULT_PLAYBACK, std::ptr::null()) };
        if device.0 == 0 {
            return Err(sdl3::get_error().to_string());
        }
        let mut sfx = Vec::new();
        for _ in 0..voices.max(1) {
            sfx.push(Stream::new(device)?);
        }
        let music = Stream::new(device)?;
        Ok(Self {
            device,
            sounds: Vec::new(),
            sfx,
            next_sfx: 0,
            generation: 0,
            music,
            _subsystem: subsystem,
        })
    }

    /// Load a WAV file. `volume` (0..1) is applied to every playback.
    pub fn load<P: AsRef<Path>>(&mut self, path: P, volume: f32) -> Result<SoundId, String> {
        let cpath = CString::new(path.as_ref().to_string_lossy().as_bytes())
            .map_err(|e| e.to_string())?;
        let mut spec = sa::SDL_AudioSpec {
            format: sa::SDL_AUDIO_S16LE,
            channels: 0,
            freq: 0,
        };
        let mut buf: *mut u8 = std::ptr::null_mut();
        let mut len: u32 = 0;
        let ok = unsafe { sa::SDL_LoadWAV(cpath.as_ptr(), &mut spec, &mut buf, &mut len) };
        if !ok {
            return Err(format!("{}: {}", path.as_ref().display(), sdl3::get_error()));
        }
        let data = unsafe { std::slice::from_raw_parts(buf, len as usize) }.to_vec();
        unsafe { sdl3::sys::stdinc::SDL_free(buf.cast()) };
        self.sounds.push(Sound { spec, data, volume });
        Ok(SoundId(self.sounds.len() - 1))
    }

    /// Length of a sound in seconds (at normal pitch).
    pub fn duration(&self, id: SoundId) -> f32 {
        let s = &self.sounds[id.0];
        let frame = bytes_per_frame(&s.spec);
        if frame == 0 || s.spec.freq == 0 {
            return 0.0;
        }
        (s.data.len() / frame) as f32 / s.spec.freq as f32
    }

    /// Play a sound effect at its base volume.
    pub fn play(&mut self, id: SoundId) -> Voice {
        self.play_ex(id, 1.0, 1.0)
    }

    /// Play a sound effect with extra `gain` and `pitch` (1.0 = normal, 2.0 = octave up).
    /// Uses a free voice, or the least recently started one if all are busy.
    pub fn play_ex(&mut self, id: SoundId, gain: f32, pitch: f32) -> Voice {
        let n = self.sfx.len();
        let slot = (0..n)
            .map(|i| (self.next_sfx + i) % n)
            .find(|&i| !self.sfx[i].playing())
            .unwrap_or(self.next_sfx);
        self.next_sfx = (slot + 1) % n;
        self.generation += 1;
        let sound = &self.sounds[id.0];
        let stream = &mut self.sfx[slot];
        stream.start(id, sound, sound.volume * gain, pitch);
        stream.generation = self.generation;
        Voice {
            slot,
            generation: self.generation,
        }
    }

    /// Is this voice still playing?
    pub fn voice_playing(&self, voice: Voice) -> bool {
        let s = &self.sfx[voice.slot];
        s.generation == voice.generation && s.playing()
    }

    /// Is this sound playing on any voice or as music?
    pub fn is_playing(&self, id: SoundId) -> bool {
        (self.music.sound == Some(id) && self.music.playing())
            || self.sfx.iter().any(|s| s.sound == Some(id) && s.playing())
    }

    /// Stop every playback of this sound (music included).
    pub fn stop(&mut self, id: SoundId) {
        for s in &mut self.sfx {
            if s.sound == Some(id) {
                s.stop();
            }
        }
        if self.music.sound == Some(id) {
            self.music.stop();
        }
    }

    /// Stop all sound effects (not the music).
    pub fn stop_all_sfx(&mut self) {
        for s in &mut self.sfx {
            s.stop();
        }
    }

    /// Loop `id` as music, replacing whatever was playing. No-op if it already is.
    pub fn play_music(&mut self, id: SoundId) {
        if self.music.sound == Some(id) && self.music.playing() {
            return;
        }
        let sound = &self.sounds[id.0];
        self.music.start(id, sound, sound.volume, 1.0);
        // Queue a second copy so the loop is seamless until the next `update`.
        self.music.put(sound);
    }

    pub fn stop_music(&mut self) {
        self.music.stop();
    }

    /// The sound currently looping as music.
    pub fn current_music(&self) -> Option<SoundId> {
        self.music.sound.filter(|_| self.music.playing())
    }

    /// Keeps the music looping. Called once per frame by [`crate::run`].
    pub fn update(&mut self) {
        if let Some(id) = self.music.sound {
            let sound = &self.sounds[id.0];
            if (self.music.queued() as usize) < sound.data.len() {
                self.music.put(sound);
            }
        }
    }

    /// Master volume (0..1) for the whole device.
    pub fn set_master_volume(&self, gain: f32) {
        unsafe {
            sa::SDL_SetAudioDeviceGain(self.device, gain.max(0.0));
        }
    }
}

impl Drop for Audio {
    fn drop(&mut self) {
        self.sfx.clear();
        unsafe { sa::SDL_CloseAudioDevice(self.device) };
    }
}

fn bytes_per_frame(spec: &sa::SDL_AudioSpec) -> usize {
    let bits = (spec.format.0 & 0xFF) as usize; // SDL_AUDIO_BITSIZE
    bits / 8 * spec.channels.max(0) as usize
}
