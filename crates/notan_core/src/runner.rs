use crate::state::AppState;
use crate::sys::System;

pub(crate) fn default_runner<S: AppState>(mut app: System<S>) -> Result<(), String> {
    // Execute initialize callback
    app.init();

    // Frame starts here
    app.frame_start();

    // Execute event's callback
    app.update();

    // Frame ends here
    app.frame_end();

    // Execute close callback
    app.close();

    Ok(())
}
