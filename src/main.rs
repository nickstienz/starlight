#![deny(clippy::all)]
#![deny(unsafe_code)]
#![allow(unused)]
use pixels::{Pixels, SurfaceTexture};
use rayon::prelude::*;
use std::time::Instant;
use std::{
    sync::mpsc::{channel, Sender},
    thread, time,
};
use winit::event::{Event, WindowEvent};

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;
const BUCKET_SIZE: u32 = 16;

fn main() {
    // Window stuff I don't understand nor care about
    let mut pixels = {
        let event_loop = winit::event_loop::EventLoop::new();
        let window = winit::window::WindowBuilder::new()
            .with_title("Starlight Ray Tracer")
            .with_inner_size(winit::dpi::PhysicalSize::new(WIDTH as f64, HEIGHT as f64))
            .build(&event_loop)
            .expect("Failed to create window");
        let texture = SurfaceTexture::new(WIDTH, HEIGHT, &window);
        Pixels::new(WIDTH, HEIGHT, texture).expect("Failed to create pixels context")
    };

    // MSCP bullshit
    let (tx, rx) = channel();

    // Worker threads

    // Number of times I have said "fuck" because of this: 29
    let buckets = (0..WIDTH)
        .step_by(BUCKET_SIZE as usize)
        .flat_map(|x| {
            (0..HEIGHT).step_by(BUCKET_SIZE as usize).map(move |y| {
                let bucket_width = BUCKET_SIZE.min(WIDTH - x);
                let bucket_height = BUCKET_SIZE.min(HEIGHT - y);
                (x, y, bucket_width, bucket_height)
            })
        })
        .collect::<Vec<_>>();

    // println!("Buckets: {:?}", buckets);

    // Multithreading bullshit
    let render_pixels_mutex = pixels_mutex.clone();
    let renderer_thread = thread::spawn(move || {
        buckets
            .par_iter()
            .for_each(|(bucket_x, bucket_y, width, height)| {
                (0..*width)
                    .flat_map(|bpixel_x| (0..*height).map(move |bpixel_y| (bpixel_x, bpixel_y)))
                    .for_each(|(bpixel_x, bpixel_y)| {
                        // Frame and index setup
                        let mut pixels = render_pixels_mutex
                            .lock()
                            .expect("Pixel failed to lock (Oh no)");
                        let frame = pixels.frame_mut();
                        let index =
                            (4 * ((bucket_x + bpixel_x) + WIDTH * (bucket_y + bpixel_y))) as usize;

                        // Variables
                        let absolute_x = bucket_x + bpixel_x;
                        let absolute_y = bucket_y + bpixel_y;
                        let u = absolute_x as f64 / WIDTH as f64;
                        let v = absolute_y as f64 / HEIGHT as f64;

                        // Ray Trace
                        let color = ray_trace(u, v);

                        // Set pixel
                        frame[index] = (color.0 * 255.0) as u8;
                        frame[index + 1] = (color.1 * 255.0) as u8;
                        frame[index + 2] = (color.2 * 255.0) as u8;
                        frame[index + 3] = 255;
                    });
            });
    });

    // Winit loop shit
    event_loop.run(move |event, _, control_flow| {
        control_flow.set_wait();

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => control_flow.set_exit(),
            _ => (),
        }

        let mut pixels = pixels_mutex.lock().expect("Failed to lock pixels mutex");
        pixels.render().unwrap();
    });
}

fn ray_trace(u: f64, v: f64) -> (f64, f64, f64) {
    (u, v, 0.5)
}
