pub(crate) enum BackendMessages {
    FullscreenMode(bool),
    Size { width: i32, height: i32 },
    Exit,
}
