#![deny(clippy::all)]
#![deny(unsafe_code)]
use engine::ray_tracer;
use pixels::{Pixels, SurfaceTexture};
use rayon::{
    prelude::{IndexedParallelIterator, ParallelIterator},
    slice::ParallelSliceMut,
};
use settings::*;
use std::thread;
use std::{
    sync::mpsc::{self},
    sync::{Arc, Mutex},
    time::Duration,
};
use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
};

use crate::primitives::color::Color;

mod engine;
mod primitives;
mod settings;

fn main() {
    println!("===[[ Starlight Ray Tracer by Nicholas Stienz! ]]===");

    let event_loop = winit::event_loop::EventLoop::new();

    let window = winit::window::WindowBuilder::new()
        .with_title("Starlight Ray Tracer")
        .build(&event_loop)
        .expect("Failed to create window");
    let physical_size = PhysicalSize::new(WIDTH as f64, HEIGHT as f64);
    window.set_inner_size(physical_size);

    println!("Window Size: {}x{}", WIDTH, HEIGHT);

    let texture = SurfaceTexture::new(WIDTH, HEIGHT, &window);
    let mut pixels = Pixels::new(WIDTH, HEIGHT, texture).expect("Failed to create pixels context");

    let (sender, receiver) = mpsc::channel();

    let color_data: Arc<Mutex<Vec<Color>>> = Arc::new(Mutex::new(Vec::with_capacity((WIDTH * HEIGHT) as usize)));
    let thread_color_data = color_data.clone();

    thread::spawn(move || loop {
        let mut new_color_data = Vec::with_capacity((WIDTH * HEIGHT) as usize);

        new_color_data
            .par_chunks_mut(WIDTH as usize)
            .enumerate()
            .for_each(|(y, row)| {
                for x in 0..WIDTH {
                    row[x as usize] = ray_tracer::ray_trace_pixel(x, y as u32);
                }
            });

        let mut color_data = thread_color_data.lock().unwrap();
        *color_data = new_color_data;
        let send_data = thread_color_data.clone();

        sender.send(send_data).expect("Failed to send color data!");
        thread::sleep(Duration::from_millis(100));
    });

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_wait();

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => control_flow.set_exit(),
            _ => (),
        }

        if let Ok(color_data) = receiver.try_recv() {
            let color_data = color_data.lock().unwrap();
            for (pixel, &ref color) in pixels
                .frame_mut()
                .chunks_exact_mut(4)
                .zip(color_data.iter())
            {
                let formated_color = color.to_u8();
                pixel.copy_from_slice(&formated_color);
            }
            pixels
                .render()
                .expect("Failed to render pixels... this can't be good.");
        }
    });
}
