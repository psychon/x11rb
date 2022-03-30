//! Provides the `ConnectInstruction` structure, which allows for a `ParsedDisplay`
//! to be transformed into a server connection.

use super::ParsedDisplay;
use std::path::PathBuf;

/// Either a hostname and port to connect to, or a socket to connect to.
// Do we want to make this #[non_exhaustive], in case we add more connection
// instructions in the future? i.e. what if we want to do X11 over QUIC?
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConnectAddress<'a> {
    /// Connect to this hostname and port over TCP.
    Hostname(&'a str, u16),
    /// Connect to this Unix socket.
    Socket(PathBuf),
}

/// Get an iterator over all of the addresses we should target with a
/// `ParsedDisplay`.
pub(super) fn connect_addresses(p: &ParsedDisplay) -> impl Iterator<Item = ConnectAddress<'_>> {
    const TCP_PORT_BASE: u16 = 6000;
    let ParsedDisplay {
        host,
        protocol,
        display,
        ..
    } = p;

    let mut targets = Vec::new();

    if (protocol.is_none() || protocol.as_deref() != Some("unix"))
        && !host.is_empty()
        && host != "unix"
    {
        targets.push(ConnectAddress::Hostname(host, TCP_PORT_BASE + display));
    } else {
        if protocol.is_none() || protocol.as_deref() == Some("unix") {
            let file_name = format!("/tmp/.X11-unix/X{}", display);
            targets.push(ConnectAddress::Socket(file_name.into()));

            // TODO: Try abstract socket (file name with prepended '\0')
            // Not supported on Rust right now: https://github.com/rust-lang/rust/issues/42048
        }

        if protocol.is_none() && host.is_empty() {
            targets.push(ConnectAddress::Hostname(
                "localhost",
                TCP_PORT_BASE + display,
            ));
        }
    }

    targets.into_iter()
}

#[cfg(test)]
mod tests {
    // make sure iterator properties are clean
    use super::{super::parse_display, ConnectAddress};
    use std::path::PathBuf;

    #[test]
    fn basic_test() {
        let pd = parse_display(Some(":0")).unwrap();
        let ci = pd.connect_instruction();
        let ci = ci.collect::<Vec<_>>();

        assert_eq!(
            ci,
            vec![
                ConnectAddress::Socket(PathBuf::from("/tmp/.X11-unix/X0")),
                ConnectAddress::Hostname("localhost", 6000),
            ]
        );
    }

    #[test]
    fn try_over_hostname() {
        let pd = parse_display(Some("192.168.1.111:0")).unwrap();
        let ci = pd.connect_instruction();

        let ci = ci.collect::<Vec<_>>();

        assert_eq!(ci, vec![ConnectAddress::Hostname("192.168.1.111", 6000),]);
    }

    #[test]
    fn try_over_unix_hostname() {
        let pd = parse_display(Some("unix/host:0")).unwrap();
        let ci = pd.connect_instruction();

        let ci = ci.collect::<Vec<_>>();

        assert_eq!(
            ci,
            vec![ConnectAddress::Socket(PathBuf::from("/tmp/.X11-unix/X0")),]
        );
    }
}
