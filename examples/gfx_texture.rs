use futures::{Async, Future};
use nae::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use futures::future::err;

struct State {
    loader: AppLoader,
    ta: Option<TA>,
    tt: Option<TT>,
}

struct ResSignal {
    inner: Rc<RefCell<ResParser>>,
    fut: Box<dyn Future<Item = Vec<u8>, Error = String>>,
    file: String,
}

impl ResSignal {
    fn new(
        file: &str,
        inner: Rc<RefCell<ResParser>>,
    ) -> ResSignal {
        ResSignal {
            inner,
            fut: Box::new(load_file(file)),
            file: file.to_string()
        }
    }
}

trait Res {
    fn from_file(app: &mut AppLoader, file: &str) -> Self;
}

trait ResInner: ResParser
where
    Self: Sized + 'static,
{
    fn new(file: &str) -> Self;
    fn from_file(app: &mut AppLoader, file: &str) -> Rc<RefCell<Self>> {
        app.load_inner_resource(file)
    }
}

trait ResParser {
    fn parse(&mut self, data: Vec<u8>) -> Result<(), String>;
}

struct AppLoader {
    queue: Vec<ResSignal>,
}

impl AppLoader {
    fn new() -> Self {
        Self { queue: vec![] }
    }

    fn load_inner_resource<T: ResInner + 'static>(&mut self, file: &str) -> Rc<RefCell<T>>{
        let inner = Rc::new(RefCell::new(T::new(file)));
        let signal = ResSignal::new(file, inner.clone());
        self.queue.push(signal);
        inner
    }

    fn try_load(&mut self) -> Result<(), Vec<String>> {
        if self.queue.len() == 0 {
            return Ok(());
        }

        let mut queue = vec![];
        let mut errors = vec![];

        while let Some(mut res) = self.queue.pop() {
            match res.fut.poll() {
                Ok(state) => match state {
                    Async::Ready(buff) => {
                        println!("Loaded file: {}", res.file);
                        res.inner.borrow_mut().parse(buff);
                    }
                    _ => {
                        queue.push(res);
                    }
                }
                Err(e) => {
                    println!("Error loading file: {} -> {}", res.file, e);
                    errors.push(e);
                }
            }
        }

        self.queue = queue;

        if errors.len() != 0 {
            return Err(errors);
        }

        Ok(())
    }
}

struct InnerTT;
impl ResParser for InnerTT {
    fn parse(&mut self, data: Vec<u8>) -> Result<(), String> {
        println!("here... {:?}", data.len());
        Ok(())
    }
}

impl ResInner for InnerTT {
    fn new(file: &str) -> Self {
        InnerTT
    }
}

impl Drop for InnerTT {
    fn drop(&mut self) {
        println!("ok tt dropped...");
    }
}

impl Drop for InnerTA {
    fn drop(&mut self) {
        println!("ok ta dropped...");
    }
}

struct TT {
    inner: Rc<RefCell<InnerTT>>
}

impl Res for TT {
    fn from_file(app: &mut AppLoader, file: &str) -> Self {
        Self {
            inner: InnerTT::from_file(app, file)
        }
    }
}

struct InnerTA;
impl ResParser for InnerTA {
    fn parse(&mut self, data: Vec<u8>) -> Result<(), String> {
        println!("here... {:?}", data.len());
        Ok(())
    }
}

impl ResInner for InnerTA {
    fn new(file: &str) -> Self {
        InnerTA
    }
}

struct TA {
    inner: Rc<RefCell<InnerTA>>
}

impl Res for TA {
    fn from_file(app: &mut AppLoader, file: &str) -> Self {
        Self {
            inner: InnerTA::from_file(app, file)
        }
    }
}

#[nae::main]
fn main() {
    nae::init_with(init).draw(draw).build().unwrap();
}

fn init(app: &mut App) -> State {
    let mut loader = AppLoader::new();
    let tt = TT::from_file(&mut loader, "./examples/assets/rust.png");
    let ta = TA::from_file(&mut loader, "./examples/assets/rust.png");

    State {
        ta: Some(ta),
        tt: None,
        // ta, tt,
        loader
    }
}

fn draw(app: &mut App, state: &mut State) {
    state.loader.try_load();

    // let _ = state.ta.take();
    // if let Some(ta) = &state.ta {
    //
    // }

    let draw = app.draw2();
    draw.begin(Color::BLUE);
    draw.end();
}
