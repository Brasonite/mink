use std::{ffi::CString, str::FromStr, sync::Arc, time::Instant};

use pyo3::{
    prelude::*,
    types::{PyDict, PyFunction},
};
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowAttributes},
};

use crate::{
    assets::Assets,
    audio::Audio,
    graphics::{builtin::VideoBuiltins, draw::Draw, stack::VideoStack},
    input::Input,
    stats::Stats,
    time::Time,
    windowing::Window as GlobalWindow,
};
struct Runtime<'a> {
    py: Python<'a>,

    init_fn: &'a Bound<'a, PyFunction>,
    load_fn: &'a Bound<'a, PyFunction>,
    update_fn: &'a Bound<'a, PyFunction>,
    draw_fn: &'a Bound<'a, PyFunction>,
    exit_fn: &'a Bound<'a, PyFunction>,

    last_frame: Instant,

    window: Option<Arc<Window>>,
    video: Option<VideoStack<'a>>,
    builtins: Option<Arc<VideoBuiltins>>,

    global_assets: Option<Bound<'a, Assets>>,
    global_audio: Option<Bound<'a, Audio>>,
    global_draw: Option<Bound<'a, Draw>>,
    global_input: Option<Bound<'a, Input>>,
    global_stats: Option<Bound<'a, Stats>>,
    global_time: Option<Bound<'a, Time>>,
    global_window: Option<Bound<'a, GlobalWindow>>,
}

impl<'a> Runtime<'a> {
    pub fn new(
        python: Python<'a>,
        init_fn: &'a Bound<PyFunction>,
        load_fn: &'a Bound<PyFunction>,
        update_fn: &'a Bound<PyFunction>,
        draw_fn: &'a Bound<PyFunction>,
        exit_fn: &'a Bound<PyFunction>,
    ) -> Self {
        Self {
            py: python,
            init_fn,
            load_fn,
            update_fn,
            draw_fn,
            exit_fn,
            last_frame: Instant::now(),
            window: None,
            video: None,
            builtins: None,
            global_assets: None,
            global_audio: None,
            global_draw: None,
            global_input: None,
            global_stats: None,
            global_time: None,
            global_window: None,
        }
    }
}

impl<'a> ApplicationHandler for Runtime<'a> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(
                    WindowAttributes::default()
                        .with_title("Mink")
                        .with_min_inner_size(PhysicalSize::new(1, 1)),
                )
                .unwrap(),
        );
        let video = VideoStack::new(Arc::clone(&window));
        let builtins = Arc::new(VideoBuiltins::new(&video));

        let global_assets = Assets::new(&video, Arc::clone(&builtins))
            .into_pyobject(self.py)
            .unwrap();
        let global_audio = Audio::new().into_pyobject(self.py).unwrap();
        let global_draw = Draw::new(&video, Arc::clone(&builtins))
            .into_pyobject(self.py)
            .unwrap();
        let global_input = Input::new().into_pyobject(self.py).unwrap();
        let global_stats = Stats::new().into_pyobject(self.py).unwrap();
        let global_time = Time::new().into_pyobject(self.py).unwrap();
        let global_window = GlobalWindow::new(Arc::clone(&window))
            .into_pyobject(self.py)
            .unwrap();

        {
            let locals = PyDict::new(self.py);
            locals.set_item("global_assets", &global_assets).unwrap();
            locals.set_item("global_audio", &global_audio).unwrap();
            locals.set_item("global_draw", &global_draw).unwrap();
            locals.set_item("global_input", &global_input).unwrap();
            locals.set_item("global_stats", &global_stats).unwrap();
            locals.set_item("global_time", &global_time).unwrap();
            locals.set_item("global_window", &global_window).unwrap();

            self.py
                .run(
                    &CString::from_str(mink_scripts::SET_GLOBALS).unwrap(),
                    None,
                    Some(&locals),
                )
                .unwrap();
        }

        self.last_frame = Instant::now();

        self.init_fn.call0().unwrap();
        self.load_fn.call0().unwrap();

        self.window = Some(window);
        self.video = Some(video);
        self.builtins = Some(builtins);

        self.global_assets = Some(global_assets);
        self.global_audio = Some(global_audio);
        self.global_draw = Some(global_draw);
        self.global_input = Some(global_input);
        self.global_stats = Some(global_stats);
        self.global_time = Some(global_time);
        self.global_window = Some(global_window);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::RedrawRequested => {
                match self.window.as_ref() {
                    Some(window) => window.request_redraw(),
                    None => return,
                };

                let delta = self.last_frame.elapsed().as_secs_f32();
                if let Some(time) = self.global_time.as_ref() {
                    time.borrow_mut().delta = delta;
                }
                self.last_frame = Instant::now();

                self.update_fn.call0().unwrap();

                let video = match self.video.as_mut() {
                    Some(video) => video,
                    None => return,
                };

                if let Some(draw) = self.global_draw.as_ref() {
                    draw.borrow_mut().begin_frame(video);
                }

                self.draw_fn.call0().unwrap();

                let mut draw = match self.global_draw.as_ref() {
                    Some(draw) => draw.borrow_mut(),
                    None => return,
                };

                if let Err(e) = video.submit(&mut draw) {
                    match e {
                        wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated => {
                            video.resize([video.config.width, video.config.height])
                        }
                        wgpu::SurfaceError::OutOfMemory | wgpu::SurfaceError::Other => {
                            println!("Out of memory!");
                            event_loop.exit();
                        }
                        wgpu::SurfaceError::Timeout => println!("Surface timeout"),
                    }
                }

                if let Some(mut input) = self.global_input.as_ref().map(|x| x.borrow_mut()) {
                    input.tick();
                }
            }
            WindowEvent::Resized(new_size) => {
                if let Some(video) = &mut self.video {
                    video.resize(new_size.into());
                }
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if let Some(mut input) = self.global_input.as_ref().map(|x| x.borrow_mut()) {
                    input.key_event(event);
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                if let Some(mut input) = self.global_input.as_ref().map(|x| x.borrow_mut()) {
                    input.mouse_motion_event(position);
                }
            }
            WindowEvent::MouseInput { state, button, .. } => {
                if let Some(mut input) = self.global_input.as_ref().map(|x| x.borrow_mut()) {
                    input.click_event(state, button);
                }
            }
            WindowEvent::MouseWheel { delta, .. } => {
                if let Some(mut input) = self.global_input.as_ref().map(|x| x.borrow_mut()) {
                    input.scroll_event(delta);
                }
            }
            _ => {}
        }
    }

    fn exiting(&mut self, _: &ActiveEventLoop) {
        self.exit_fn.call0().unwrap();

        self.py
            .run(
                &CString::from_str(mink_scripts::RESET_GLOBALS).unwrap(),
                None,
                None,
            )
            .unwrap();
    }
}

#[pyfunction]
#[pyo3(pass_module)]
pub fn run(
    module: &Bound<PyModule>,
    init: &Bound<PyFunction>,
    load: &Bound<PyFunction>,
    update: &Bound<PyFunction>,
    draw: &Bound<PyFunction>,
    exit: &Bound<PyFunction>,
) {
    let event_loop = EventLoop::new().unwrap();
    let mut runtime = Runtime::new(module.py(), init, load, update, draw, exit);
    event_loop.run_app(&mut runtime).unwrap()
}
