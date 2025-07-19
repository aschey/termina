use std::{
    io::{self, Write as _},
    time::Duration,
};
use termina::escape::csi::DecPrivateModeCode as Dec;
use termina::{PlatformTerminal, Terminal};
// helpers for these maze of types
const fn set(f: termina::escape::csi::DecPrivateModeCode) -> termina::escape::csi::Csi {
    termina::escape::csi::Csi::Mode(termina::escape::csi::Mode::SetDecPrivateMode(
        termina::escape::csi::DecPrivateMode::Code(f),
    ))
}
const fn reset(f: termina::escape::csi::DecPrivateModeCode) -> termina::escape::csi::Csi {
    termina::escape::csi::Csi::Mode(termina::escape::csi::Mode::ResetDecPrivateMode(
        termina::escape::csi::DecPrivateMode::Code(f),
    ))
}

// struct Context {
//     terminal: PlatformTerminal,
// }

// impl Context {
//     fn new() -> std::io::Result<Self> {
//         let mut terminal = PlatformTerminal::new()?;
//         terminal.enter_raw_mode()?;
//         terminal.set_panic_hook(Self::reset);
//         Self::initialize(&mut terminal)?;
//         Ok(Self { terminal })
//     }

//     fn change_mode(
//         terminal: &mut impl std::io::Write,
//         // just a way to reduce how many times these modes have to be spelled out
//         mode: fn(termina::escape::csi::DecPrivateModeCode) -> termina::escape::csi::Csi,
//     ) -> std::io::Result<()> {
//         use termina::escape::csi::DecPrivateModeCode as Dec;

//         for request in [
//             Dec::ClearAndEnableAlternateScreen, //
//             Dec::AnyEventMouse,
//             Dec::SGRMouse,
//         ] {
//             write!(terminal, "{}", mode(request))?;
//             // flush after each, just to be cautious (performance overhead here is irrelevant)
//             terminal.flush()?;
//         }
//         Ok(())
//     }

//     fn initialize(terminal: &mut impl std::io::Write) -> std::io::Result<()> {
//         Self::change_mode(terminal, set)
//     }

//     fn reset(terminal: &mut impl std::io::Write) {
//         Self::change_mode(terminal, reset).unwrap();
//     }
// }

// impl Drop for Context {
//     fn drop(&mut self) {
//         Self::reset(&mut self.terminal);
//         self.terminal.enter_cooked_mode().unwrap();
//     }
// }

fn main() -> io::Result<()> {
    let mut terminal = PlatformTerminal::new()?;
    terminal.enter_raw_mode()?;
    let mut out = io::stdout();
    for request in [
        Dec::ClearAndEnableAlternateScreen, //
        Dec::AnyEventMouse,
        Dec::SGRMouse,
    ] {
        write!(out, "{}", set(request))?;
    }
    writeln!(
        &mut out,
        "\x1b[1;1;Hwaiting for a second. move the mouse to see the problem"
    )?;
    out.flush()?;

    std::thread::sleep(std::time::Duration::from_secs(1));

    for request in [
        Dec::ClearAndEnableAlternateScreen, //
        Dec::AnyEventMouse,
        Dec::SGRMouse,
    ] {
        write!(out, "{}", reset(request))?;
    }
    out.flush()?;

    terminal.event_reader().drain()?;
    terminal.enter_cooked_mode()?;

    Ok(())
}
