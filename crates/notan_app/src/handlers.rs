use crate::app::{App, AppState};
use crate::assets::AssetManager;
use crate::events::Event;
use crate::graphics::{GfxExtension, GfxRenderer, Graphics};
use crate::plugins::{Plugin, Plugins};

// Order of params App, AssetManager, Graphics, GlyphManager, Plugins, S, Event
notan_macro::handler!(Setup<&mut App, &mut AssetManager, &mut Graphics, &mut Plugins> -> S);
notan_macro::handler!(App<&mut App, &mut AssetManager, &mut Plugins, &mut S>);
notan_macro::handler!(Event<&mut App, &mut AssetManager, &mut Plugins, &mut S, Event>);
notan_macro::handler!(Draw<&mut App, &mut AssetManager, &mut Graphics, &mut Plugins, &mut S>);
notan_macro::handler!(Plugin<&mut App, &mut AssetManager, &mut Graphics, &mut Plugins> -> !S); // !S stands for Plugin + 'static
notan_macro::handler!(Extension<&mut App, &mut AssetManager, &mut Graphics, &mut Plugins> -> $S); // $S stands for GfxExtension + 'static
