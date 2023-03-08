#[derive(Debug)]
pub enum SDLError {
    LoadSDLContext,
    LoadVideoSubsystem,
    LoadTtfContext,
    BuildWindow,
    BuildCanvas
}

impl std::fmt::Display for SDLError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SDLError::LoadSDLContext => write!(f, "Failt at loading SDL context."),
            SDLError::LoadVideoSubsystem => write!(f, "Fail at loading SDL video subsystem."),
            SDLError::LoadTtfContext => write!(f, "Failt at loading ttf context."),
            SDLError::BuildWindow => write!(f, "Fail at building window."),
            SDLError::BuildCanvas => write!(f, "Fail at building canvas.")
        }
    }
}

impl std::error::Error for SDLError {}
