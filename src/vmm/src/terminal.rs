use std::os::fd::RawFd;
use libc::{STDERR_FILENO, STDIN_FILENO, STDOUT_FILENO};
use nix::sys::termios::{LocalFlags, SetArg, tcgetattr, tcsetattr};
use nix::unistd::isatty;

pub fn term_set_raw_mode() -> Result<(), nix::Error> {
    if let Some(fd) = get_connected_term_fd() {
        term_fd_set_raw_mode(fd)
    } else {
        Ok(())
    }
}

pub fn term_set_canonical_mode() -> Result<(), nix::Error> {
    if let Some(fd) = get_connected_term_fd() {
        term_fd_set_canonical_mode(fd)
    } else {
        Ok(())
    }
}

pub fn term_fd_set_raw_mode(term: RawFd) -> Result<(), nix::Error> {
    let mut termios = tcgetattr(term)?;
    termios.local_flags &= !(LocalFlags::ECHO | LocalFlags::ICANON | LocalFlags::ISIG);
    tcsetattr(term, SetArg::TCSANOW, &termios)?;
    Ok(())
}

pub fn term_fd_set_canonical_mode(term: RawFd) -> Result<(), nix::Error> {
    let mut termios = tcgetattr(term)?;
    termios.local_flags |= LocalFlags::ECHO | LocalFlags::ICANON | LocalFlags::ISIG;
    tcsetattr(term, SetArg::TCSANOW, &termios)?;
    Ok(())
}

pub fn get_connected_term_fd() -> Option<RawFd> {
    if isatty(STDIN_FILENO).unwrap_or(false) {
        Some(STDIN_FILENO)
    } else if isatty(STDOUT_FILENO).unwrap_or(false) {
        Some(STDOUT_FILENO)
    } else if isatty(STDERR_FILENO).unwrap_or(false) {
        Some(STDERR_FILENO)
    } else {
        None
    }
}


