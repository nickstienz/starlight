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
        .with_title("Starlight Ray Tracer - FPS: ...")
        .build(&event_loop)
        .expect("Failed to create window");
    let physical_size = PhysicalSize::new(WIDTH as f64, HEIGHT as f64);
    window.set_inner_size(physical_size);

    window.set_max_inner_size(Some(PhysicalSize::new(WIDTH, HEIGHT)));

    println!("Window Size: {}x{}", WIDTH, HEIGHT);

    let texture = SurfaceTexture::new(WIDTH, HEIGHT, &window);
    let mut pixels = Pixels::new(WIDTH, HEIGHT, texture).expect("Failed to create pixels context");

    let (sender, receiver) = mpsc::channel();

    thread::spawn(move || {
        let mut color_data: Vec<Color> =
            vec![Color::new(0.0, 0.0, 0.0, 1.0); (WIDTH * HEIGHT) as usize];

        // I somehow cause a memory leak in rust so fuck me I guess... I'm reworking this later
        loop {
            thread::sleep(Duration::from_millis(1000));
            color_data
                .par_chunks_mut(WIDTH as usize)
                .enumerate()
                .for_each(|(y, row)| {
                    for x in 0..WIDTH {
                        row[x as usize] = ray_tracer::ray_trace_pixel(x, y as u32);
                    }
                });

            sender
                .send(color_data.clone())
                .expect("Failed to send color data!");
        }
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
            for (pixel, &color) in pixels
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
