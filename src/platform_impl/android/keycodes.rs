use android_activity::{
    input::{KeyAction, KeyEvent, KeyMapChar, Keycode},
    AndroidApp,
};

use crate::keyboard::{Key, KeyCode, KeyLocation, NativeKey, NativeKeyCode};

pub fn to_physical_keycode(keycode: Keycode) -> KeyCode {
    match keycode {
        Keycode::A => KeyCode::KeyA,
        Keycode::B => KeyCode::KeyB,
        Keycode::C => KeyCode::KeyC,
        Keycode::D => KeyCode::KeyD,
        Keycode::E => KeyCode::KeyE,
        Keycode::F => KeyCode::KeyF,
        Keycode::G => KeyCode::KeyG,
        Keycode::H => KeyCode::KeyH,
        Keycode::I => KeyCode::KeyI,
        Keycode::J => KeyCode::KeyJ,
        Keycode::K => KeyCode::KeyK,
        Keycode::L => KeyCode::KeyL,
        Keycode::M => KeyCode::KeyM,
        Keycode::N => KeyCode::KeyN,
        Keycode::O => KeyCode::KeyO,
        Keycode::P => KeyCode::KeyP,
        Keycode::Q => KeyCode::KeyQ,
        Keycode::R => KeyCode::KeyR,
        Keycode::S => KeyCode::KeyS,
        Keycode::T => KeyCode::KeyT,
        Keycode::U => KeyCode::KeyU,
        Keycode::V => KeyCode::KeyV,
        Keycode::W => KeyCode::KeyW,
        Keycode::X => KeyCode::KeyX,
        Keycode::Y => KeyCode::KeyY,
        Keycode::Z => KeyCode::KeyZ,

        Keycode::Keycode0 => KeyCode::Digit0,
        Keycode::Keycode1 => KeyCode::Digit1,
        Keycode::Keycode2 => KeyCode::Digit2,
        Keycode::Keycode3 => KeyCode::Digit3,
        Keycode::Keycode4 => KeyCode::Digit4,
        Keycode::Keycode5 => KeyCode::Digit5,
        Keycode::Keycode6 => KeyCode::Digit6,
        Keycode::Keycode7 => KeyCode::Digit7,
        Keycode::Keycode8 => KeyCode::Digit8,
        Keycode::Keycode9 => KeyCode::Digit9,

        Keycode::Numpad0 => KeyCode::Numpad0,
        Keycode::Numpad1 => KeyCode::Numpad1,
        Keycode::Numpad2 => KeyCode::Numpad2,
        Keycode::Numpad3 => KeyCode::Numpad3,
        Keycode::Numpad4 => KeyCode::Numpad4,
        Keycode::Numpad5 => KeyCode::Numpad5,
        Keycode::Numpad6 => KeyCode::Numpad6,
        Keycode::Numpad7 => KeyCode::Numpad7,
        Keycode::Numpad8 => KeyCode::Numpad8,
        Keycode::Numpad9 => KeyCode::Numpad9,

        Keycode::NumpadAdd => KeyCode::NumpadAdd,
        Keycode::NumpadSubtract => KeyCode::NumpadSubtract,
        Keycode::NumpadMultiply => KeyCode::NumpadMultiply,
        Keycode::NumpadDivide => KeyCode::NumpadDivide,
        Keycode::NumpadEnter => KeyCode::NumpadEnter,
        Keycode::NumpadEquals => KeyCode::NumpadEqual,
        Keycode::NumpadComma => KeyCode::NumpadComma,
        Keycode::NumpadDot => KeyCode::NumpadDecimal,
        Keycode::NumLock => KeyCode::NumLock,

        Keycode::DpadLeft => KeyCode::ArrowLeft,
        Keycode::DpadRight => KeyCode::ArrowRight,
        Keycode::DpadUp => KeyCode::ArrowUp,
        Keycode::DpadDown => KeyCode::ArrowDown,

        Keycode::F1 => KeyCode::F1,
        Keycode::F2 => KeyCode::F2,
        Keycode::F3 => KeyCode::F3,
        Keycode::F4 => KeyCode::F4,
        Keycode::F5 => KeyCode::F5,
        Keycode::F6 => KeyCode::F6,
        Keycode::F7 => KeyCode::F7,
        Keycode::F8 => KeyCode::F8,
        Keycode::F9 => KeyCode::F9,
        Keycode::F10 => KeyCode::F10,
        Keycode::F11 => KeyCode::F11,
        Keycode::F12 => KeyCode::F12,

        Keycode::Space => KeyCode::Space,
        Keycode::Escape => KeyCode::Escape,
        Keycode::Enter => KeyCode::Enter, // not on the Numpad
        Keycode::Tab => KeyCode::Tab,

        Keycode::PageUp => KeyCode::PageUp,
        Keycode::PageDown => KeyCode::PageDown,
        Keycode::MoveHome => KeyCode::Home,
        Keycode::MoveEnd => KeyCode::End,
        Keycode::Insert => KeyCode::Insert,

        Keycode::Del => KeyCode::Backspace, // Backspace (above Enter)
        Keycode::ForwardDel => KeyCode::Delete, // Delete (below Insert)

        Keycode::Copy => KeyCode::Copy,
        Keycode::Paste => KeyCode::Paste,
        Keycode::Cut => KeyCode::Cut,

        Keycode::VolumeUp => KeyCode::AudioVolumeUp,
        Keycode::VolumeDown => KeyCode::AudioVolumeDown,
        Keycode::VolumeMute => KeyCode::AudioVolumeMute,
        //Keycode::Mute => None, // Microphone mute
        Keycode::MediaPlayPause => KeyCode::MediaPlayPause,
        Keycode::MediaStop => KeyCode::MediaStop,
        Keycode::MediaNext => KeyCode::MediaTrackNext,
        Keycode::MediaPrevious => KeyCode::MediaTrackPrevious,

        Keycode::Plus => KeyCode::Equal,
        Keycode::Minus => KeyCode::Minus,
        // Winit doesn't differentiate both '+' and '=', considering they are usually
        // on the same physical key
        Keycode::Equals => KeyCode::Equal,
        Keycode::Semicolon => KeyCode::Semicolon,
        Keycode::Slash => KeyCode::Slash,
        Keycode::Backslash => KeyCode::Backslash,
        Keycode::Comma => KeyCode::Comma,
        Keycode::Period => KeyCode::Period,
        Keycode::Apostrophe => KeyCode::Quote,
        Keycode::Grave => KeyCode::Backquote,

        // Winit doesn't expose a SysRq code, so map to PrintScreen since it's
        // usually the same physical key
        Keycode::Sysrq => KeyCode::PrintScreen,
        // These are usually the same (Pause/Break)
        Keycode::Break => KeyCode::Pause,
        // These are exactly the same
        Keycode::ScrollLock => KeyCode::ScrollLock,

        Keycode::Yen => KeyCode::IntlYen,
        Keycode::Kana => KeyCode::Lang1,
        Keycode::KatakanaHiragana => KeyCode::KanaMode,

        Keycode::CtrlLeft => KeyCode::ControlLeft,
        Keycode::CtrlRight => KeyCode::ControlRight,

        Keycode::ShiftLeft => KeyCode::ShiftLeft,
        Keycode::ShiftRight => KeyCode::ShiftRight,

        Keycode::AltLeft => KeyCode::AltLeft,
        Keycode::AltRight => KeyCode::AltRight,

        Keycode::MetaLeft => KeyCode::SuperLeft,
        Keycode::MetaRight => KeyCode::SuperRight,

        Keycode::LeftBracket => KeyCode::BracketLeft,
        Keycode::RightBracket => KeyCode::BracketRight,

        Keycode::Power => KeyCode::Power,
        Keycode::Sleep => KeyCode::Sleep, // what about SoftSleep?
        Keycode::Wakeup => KeyCode::WakeUp,

        keycode => KeyCode::Unidentified(NativeKeyCode::Android(keycode.into())),
    }
}

/// Tries to map the `key_event` to a `KeyMapChar` containing a unicode character or dead key accent
///
/// This takes a `KeyEvent` and looks up its corresponding `KeyCharacterMap` and
/// uses that to try and map the `key_code` + `meta_state` to a unicode
/// character or a dead key that can be combined with the next key press.
pub fn character_map_and_combine_key(
    app: &AndroidApp,
    key_event: &KeyEvent<'_>,
    combining_accent: &mut Option<char>,
) -> Option<KeyMapChar> {
    let device_id = key_event.device_id();

    let key_map = match app.device_key_character_map(device_id) {
        Ok(key_map) => key_map,
        Err(err) => {
            log::warn!("Failed to look up `KeyCharacterMap` for device {device_id}: {err:?}");
            return None;
        }
    };

    match key_map.get(key_event.key_code(), key_event.meta_state()) {
        Ok(KeyMapChar::Unicode(unicode)) => {
            // Only do dead key combining on key down
            if key_event.action() == KeyAction::Down {
                let combined_unicode = if let Some(accent) = combining_accent {
                    match key_map.get_dead_char(*accent, unicode) {
                        Ok(Some(key)) => Some(key),
                        Ok(None) => None,
                        Err(err) => {
                            log::warn!("KeyEvent: Failed to combine 'dead key' accent '{accent}' with '{unicode}': {err:?}");
                            None
                        }
                    }
                } else {
                    Some(unicode)
                };
                *combining_accent = None;
                combined_unicode.map(KeyMapChar::Unicode)
            } else {
                Some(KeyMapChar::Unicode(unicode))
            }
        }
        Ok(KeyMapChar::CombiningAccent(accent)) => {
            if key_event.action() == KeyAction::Down {
                *combining_accent = Some(accent);
            }
            Some(KeyMapChar::CombiningAccent(accent))
        }
        Ok(KeyMapChar::None) => {
            // Leave any combining_accent state in tact (seems to match how other
            // Android apps work)
            None
        }
        Err(err) => {
            log::warn!("KeyEvent: Failed to get key map character: {err:?}");
            *combining_accent = None;
            None
        }
    }
}

pub fn to_logical(key_char: Option<KeyMapChar>, keycode: Keycode) -> Key {
    use android_activity::input::Keycode::*;

    let native = NativeKey::Android(keycode.into());

    match key_char {
        Some(KeyMapChar::Unicode(c)) => Key::Character(smol_str::SmolStr::from_iter([c])),
        Some(KeyMapChar::CombiningAccent(c)) => Key::Dead(Some(c)),
        None | Some(KeyMapChar::None) => match keycode {
            // Using `BrowserHome` instead of `GoHome` according to
            // https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/key/Key_Values
            Home => Key::BrowserHome,
            Back => Key::BrowserBack,
            Call => Key::Call,
            Endcall => Key::EndCall,

            //-------------------------------------------------------------------------------
            // These should be redundant because they should have already been matched
            // as `KeyMapChar::Unicode`, but also matched here as a fallback
            Keycode0 => Key::Character("0".into()),
            Keycode1 => Key::Character("1".into()),
            Keycode2 => Key::Character("2".into()),
            Keycode3 => Key::Character("3".into()),
            Keycode4 => Key::Character("4".into()),
            Keycode5 => Key::Character("5".into()),
            Keycode6 => Key::Character("6".into()),
            Keycode7 => Key::Character("7".into()),
            Keycode8 => Key::Character("8".into()),
            Keycode9 => Key::Character("9".into()),
            Star => Key::Character("*".into()),
            Pound => Key::Character("#".into()),
            A => Key::Character("a".into()),
            B => Key::Character("b".into()),
            C => Key::Character("c".into()),
            D => Key::Character("d".into()),
            E => Key::Character("e".into()),
            F => Key::Character("f".into()),
            G => Key::Character("g".into()),
            H => Key::Character("h".into()),
            I => Key::Character("i".into()),
            J => Key::Character("j".into()),
            K => Key::Character("k".into()),
            L => Key::Character("l".into()),
            M => Key::Character("m".into()),
            N => Key::Character("n".into()),
            O => Key::Character("o".into()),
            P => Key::Character("p".into()),
            Q => Key::Character("q".into()),
            R => Key::Character("r".into()),
            S => Key::Character("s".into()),
            T => Key::Character("t".into()),
            U => Key::Character("u".into()),
            V => Key::Character("v".into()),
            W => Key::Character("w".into()),
            X => Key::Character("x".into()),
            Y => Key::Character("y".into()),
            Z => Key::Character("z".into()),
            Comma => Key::Character(",".into()),
            Period => Key::Character(".".into()),
            Grave => Key::Character("`".into()),
            Minus => Key::Character("-".into()),
            Equals => Key::Character("=".into()),
            LeftBracket => Key::Character("[".into()),
            RightBracket => Key::Character("]".into()),
            Backslash => Key::Character("\\".into()),
            Semicolon => Key::Character(";".into()),
            Apostrophe => Key::Character("'".into()),
            Slash => Key::Character("/".into()),
            At => Key::Character("@".into()),
            Plus => Key::Character("+".into()),
            //-------------------------------------------------------------------------------
            DpadUp => Key::ArrowUp,
            DpadDown => Key::ArrowDown,
            DpadLeft => Key::ArrowLeft,
            DpadRight => Key::ArrowRight,
            DpadCenter => Key::Enter,

            VolumeUp => Key::AudioVolumeUp,
            VolumeDown => Key::AudioVolumeDown,
            Power => Key::Power,
            Camera => Key::Camera,
            Clear => Key::Clear,

            AltLeft => Key::Alt,
            AltRight => Key::Alt,
            ShiftLeft => Key::Shift,
            ShiftRight => Key::Shift,
            Tab => Key::Tab,
            Space => Key::Space,
            Sym => Key::Symbol,
            Explorer => Key::LaunchWebBrowser,
            Envelope => Key::LaunchMail,
            Enter => Key::Enter,
            Del => Key::Backspace,

            // According to https://developer.android.com/reference/android/view/KeyEvent#KEYCODE_NUM
            Num => Key::Alt,

            Headsethook => Key::HeadsetHook,
            Focus => Key::CameraFocus,

            Notification => Key::Notification,
            Search => Key::BrowserSearch,
            MediaPlayPause => Key::MediaPlayPause,
            MediaStop => Key::MediaStop,
            MediaNext => Key::MediaTrackNext,
            MediaPrevious => Key::MediaTrackPrevious,
            MediaRewind => Key::MediaRewind,
            MediaFastForward => Key::MediaFastForward,
            Mute => Key::MicrophoneVolumeMute,
            PageUp => Key::PageUp,
            PageDown => Key::PageDown,

            Escape => Key::Escape,
            ForwardDel => Key::Delete,
            CtrlLeft => Key::Control,
            CtrlRight => Key::Control,
            CapsLock => Key::CapsLock,
            ScrollLock => Key::ScrollLock,
            MetaLeft => Key::Super,
            MetaRight => Key::Super,
            Function => Key::Fn,
            Sysrq => Key::PrintScreen,
            Break => Key::Pause,
            MoveHome => Key::Home,
            MoveEnd => Key::End,
            Insert => Key::Insert,
            Forward => Key::BrowserForward,
            MediaPlay => Key::MediaPlay,
            MediaPause => Key::MediaPause,
            MediaClose => Key::MediaClose,
            MediaEject => Key::Eject,
            MediaRecord => Key::MediaRecord,
            F1 => Key::F1,
            F2 => Key::F2,
            F3 => Key::F3,
            F4 => Key::F4,
            F5 => Key::F5,
            F6 => Key::F6,
            F7 => Key::F7,
            F8 => Key::F8,
            F9 => Key::F9,
            F10 => Key::F10,
            F11 => Key::F11,
            F12 => Key::F12,
            NumLock => Key::NumLock,
            Numpad0 => Key::Character("0".into()),
            Numpad1 => Key::Character("1".into()),
            Numpad2 => Key::Character("2".into()),
            Numpad3 => Key::Character("3".into()),
            Numpad4 => Key::Character("4".into()),
            Numpad5 => Key::Character("5".into()),
            Numpad6 => Key::Character("6".into()),
            Numpad7 => Key::Character("7".into()),
            Numpad8 => Key::Character("8".into()),
            Numpad9 => Key::Character("9".into()),
            NumpadDivide => Key::Character("/".into()),
            NumpadMultiply => Key::Character("*".into()),
            NumpadSubtract => Key::Character("-".into()),
            NumpadAdd => Key::Character("+".into()),
            NumpadDot => Key::Character(".".into()),
            NumpadComma => Key::Character(",".into()),
            NumpadEnter => Key::Enter,
            NumpadEquals => Key::Character("=".into()),
            NumpadLeftParen => Key::Character("(".into()),
            NumpadRightParen => Key::Character(")".into()),

            VolumeMute => Key::AudioVolumeMute,
            Info => Key::Info,
            ChannelUp => Key::ChannelUp,
            ChannelDown => Key::ChannelDown,
            ZoomIn => Key::ZoomIn,
            ZoomOut => Key::ZoomOut,
            Tv => Key::TV,
            Guide => Key::Guide,
            Dvr => Key::DVR,
            Bookmark => Key::BrowserFavorites,
            Captions => Key::ClosedCaptionToggle,
            Settings => Key::Settings,
            TvPower => Key::TVPower,
            TvInput => Key::TVInput,
            StbPower => Key::STBPower,
            StbInput => Key::STBInput,
            AvrPower => Key::AVRPower,
            AvrInput => Key::AVRInput,
            ProgRed => Key::ColorF0Red,
            ProgGreen => Key::ColorF1Green,
            ProgYellow => Key::ColorF2Yellow,
            ProgBlue => Key::ColorF3Blue,
            AppSwitch => Key::AppSwitch,
            LanguageSwitch => Key::GroupNext,
            MannerMode => Key::MannerMode,
            Keycode3dMode => Key::TV3DMode,
            Contacts => Key::LaunchContacts,
            Calendar => Key::LaunchCalendar,
            Music => Key::LaunchMusicPlayer,
            Calculator => Key::LaunchApplication2,
            ZenkakuHankaku => Key::ZenkakuHankaku,
            Eisu => Key::Eisu,
            Muhenkan => Key::NonConvert,
            Henkan => Key::Convert,
            KatakanaHiragana => Key::HiraganaKatakana,
            Kana => Key::KanjiMode,
            BrightnessDown => Key::BrightnessDown,
            BrightnessUp => Key::BrightnessUp,
            MediaAudioTrack => Key::MediaAudioTrack,
            Sleep => Key::Standby,
            Wakeup => Key::WakeUp,
            Pairing => Key::Pairing,
            MediaTopMenu => Key::MediaTopMenu,
            LastChannel => Key::MediaLast,
            TvDataService => Key::TVDataService,
            VoiceAssist => Key::VoiceDial,
            TvRadioService => Key::TVRadioService,
            TvTeletext => Key::Teletext,
            TvNumberEntry => Key::TVNumberEntry,
            TvTerrestrialAnalog => Key::TVTerrestrialAnalog,
            TvTerrestrialDigital => Key::TVTerrestrialDigital,
            TvSatellite => Key::TVSatellite,
            TvSatelliteBs => Key::TVSatelliteBS,
            TvSatelliteCs => Key::TVSatelliteCS,
            TvSatelliteService => Key::TVSatelliteToggle,
            TvNetwork => Key::TVNetwork,
            TvAntennaCable => Key::TVAntennaCable,
            TvInputHdmi1 => Key::TVInputHDMI1,
            TvInputHdmi2 => Key::TVInputHDMI2,
            TvInputHdmi3 => Key::TVInputHDMI3,
            TvInputHdmi4 => Key::TVInputHDMI4,
            TvInputComposite1 => Key::TVInputComposite1,
            TvInputComposite2 => Key::TVInputComposite2,
            TvInputComponent1 => Key::TVInputComponent1,
            TvInputComponent2 => Key::TVInputComponent2,
            TvInputVga1 => Key::TVInputVGA1,
            TvAudioDescription => Key::TVAudioDescription,
            TvAudioDescriptionMixUp => Key::TVAudioDescriptionMixUp,
            TvAudioDescriptionMixDown => Key::TVAudioDescriptionMixDown,
            TvZoomMode => Key::ZoomToggle,
            TvContentsMenu => Key::TVContentsMenu,
            TvMediaContextMenu => Key::TVMediaContext,
            TvTimerProgramming => Key::TVTimer,
            Help => Key::Help,
            NavigatePrevious => Key::NavigatePrevious,
            NavigateNext => Key::NavigateNext,
            NavigateIn => Key::NavigateIn,
            NavigateOut => Key::NavigateOut,
            MediaSkipForward => Key::MediaSkipForward,
            MediaSkipBackward => Key::MediaSkipBackward,
            MediaStepForward => Key::MediaStepForward,
            MediaStepBackward => Key::MediaStepBackward,
            Cut => Key::Cut,
            Copy => Key::Copy,
            Paste => Key::Paste,
            Refresh => Key::BrowserRefresh,

            // -----------------------------------------------------------------
            // Keycodes that don't have a logical Key mapping
            // -----------------------------------------------------------------
            Unknown => Key::Unidentified(native),

            // Can be added on demand
            SoftLeft => Key::Unidentified(native),
            SoftRight => Key::Unidentified(native),

            Menu => Key::Unidentified(native),

            Pictsymbols => Key::Unidentified(native),
            SwitchCharset => Key::Unidentified(native),

            // -----------------------------------------------------------------
            // Gamepad events should be exposed through a separate API, not
            // keyboard events
            ButtonA => Key::Unidentified(native),
            ButtonB => Key::Unidentified(native),
            ButtonC => Key::Unidentified(native),
            ButtonX => Key::Unidentified(native),
            ButtonY => Key::Unidentified(native),
            ButtonZ => Key::Unidentified(native),
            ButtonL1 => Key::Unidentified(native),
            ButtonR1 => Key::Unidentified(native),
            ButtonL2 => Key::Unidentified(native),
            ButtonR2 => Key::Unidentified(native),
            ButtonThumbl => Key::Unidentified(native),
            ButtonThumbr => Key::Unidentified(native),
            ButtonStart => Key::Unidentified(native),
            ButtonSelect => Key::Unidentified(native),
            ButtonMode => Key::Unidentified(native),
            // -----------------------------------------------------------------
            Window => Key::Unidentified(native),

            Button1 => Key::Unidentified(native),
            Button2 => Key::Unidentified(native),
            Button3 => Key::Unidentified(native),
            Button4 => Key::Unidentified(native),
            Button5 => Key::Unidentified(native),
            Button6 => Key::Unidentified(native),
            Button7 => Key::Unidentified(native),
            Button8 => Key::Unidentified(native),
            Button9 => Key::Unidentified(native),
            Button10 => Key::Unidentified(native),
            Button11 => Key::Unidentified(native),
            Button12 => Key::Unidentified(native),
            Button13 => Key::Unidentified(native),
            Button14 => Key::Unidentified(native),
            Button15 => Key::Unidentified(native),
            Button16 => Key::Unidentified(native),

            Yen => Key::Unidentified(native),
            Ro => Key::Unidentified(native),

            Assist => Key::Unidentified(native),

            Keycode11 => Key::Unidentified(native),
            Keycode12 => Key::Unidentified(native),

            StemPrimary => Key::Unidentified(native),
            Stem1 => Key::Unidentified(native),
            Stem2 => Key::Unidentified(native),
            Stem3 => Key::Unidentified(native),

            DpadUpLeft => Key::Unidentified(native),
            DpadDownLeft => Key::Unidentified(native),
            DpadUpRight => Key::Unidentified(native),
            DpadDownRight => Key::Unidentified(native),

            SoftSleep => Key::Unidentified(native),

            SystemNavigationUp => Key::Unidentified(native),
            SystemNavigationDown => Key::Unidentified(native),
            SystemNavigationLeft => Key::Unidentified(native),
            SystemNavigationRight => Key::Unidentified(native),

            AllApps => Key::Unidentified(native),
            ThumbsUp => Key::Unidentified(native),
            ThumbsDown => Key::Unidentified(native),
            ProfileSwitch => Key::Unidentified(native),
        },
    }
}

pub fn to_location(keycode: Keycode) -> KeyLocation {
    use android_activity::input::Keycode::*;

    match keycode {
        AltLeft => KeyLocation::Left,
        AltRight => KeyLocation::Right,
        ShiftLeft => KeyLocation::Left,
        ShiftRight => KeyLocation::Right,

        // According to https://developer.android.com/reference/android/view/KeyEvent#KEYCODE_NUM
        Num => KeyLocation::Left,

        CtrlLeft => KeyLocation::Left,
        CtrlRight => KeyLocation::Right,
        MetaLeft => KeyLocation::Left,
        MetaRight => KeyLocation::Right,

        NumLock => KeyLocation::Numpad,
        Numpad0 => KeyLocation::Numpad,
        Numpad1 => KeyLocation::Numpad,
        Numpad2 => KeyLocation::Numpad,
        Numpad3 => KeyLocation::Numpad,
        Numpad4 => KeyLocation::Numpad,
        Numpad5 => KeyLocation::Numpad,
        Numpad6 => KeyLocation::Numpad,
        Numpad7 => KeyLocation::Numpad,
        Numpad8 => KeyLocation::Numpad,
        Numpad9 => KeyLocation::Numpad,
        NumpadDivide => KeyLocation::Numpad,
        NumpadMultiply => KeyLocation::Numpad,
        NumpadSubtract => KeyLocation::Numpad,
        NumpadAdd => KeyLocation::Numpad,
        NumpadDot => KeyLocation::Numpad,
        NumpadComma => KeyLocation::Numpad,
        NumpadEnter => KeyLocation::Numpad,
        NumpadEquals => KeyLocation::Numpad,
        NumpadLeftParen => KeyLocation::Numpad,
        NumpadRightParen => KeyLocation::Numpad,

        _ => KeyLocation::Standard,
    }
}
