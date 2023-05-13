#![deny(clippy::all)]
#![deny(unsafe_code)]
use crossbeam_channel::bounded;
use engine::ray_tracer;
use num_cpus;
use pixels::{Pixels, SurfaceTexture};
use renderer::bucket::*;
use renderer::settings::*;
use std::thread;
use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
};

mod engine;
mod primitives;
mod renderer;

fn main() {
    println!("===[[ Starlight Ray Tracer by Nicholas Stienz! ]]===");
    // Window stuff I don't understand nor care about
    let event_loop = winit::event_loop::EventLoop::new();

    let window = winit::window::WindowBuilder::new()
        .with_title("Starlight Ray Tracer")
        .build(&event_loop)
        .expect("Failed to create window");
    let physical_size = PhysicalSize::new(WIDTH as f64, HEIGHT as f64);
    window.set_inner_size(physical_size);

    window.set_max_inner_size(Some(PhysicalSize::new(WIDTH, HEIGHT)));

    println!("Window Size: {}x{}", WIDTH, HEIGHT);

    let texture = SurfaceTexture::new(WIDTH, HEIGHT, &window);
    let mut pixels = Pixels::new(WIDTH, HEIGHT, texture).expect("Failed to create pixels context");

    // Number of times I have said "fuck" because of this: 31
    let buckets = create_buckets(WIDTH, HEIGHT);
    let mut num_buckets = buckets.len();

    // Crossbeam fun
    let (tx, rx) = bounded::<Bucket>(buckets.len());

    let num_threads = buckets.len().min(num_cpus::get());
    let mut handles = Vec::with_capacity(num_threads);

    // Create render threads
    for _ in 0..num_threads {
        let tx = tx.clone();
        let rx = rx.clone();
        handles.push(thread::spawn(move || {
            while let Ok(bucket) = rx.recv() {
                tx.send(ray_trace_bucket(&bucket))
                    .expect("Oh no, something went wrong sending a bucket!");
            }
        }));
    }

    // Send buckets to render
    buckets.into_iter().for_each(|bucket| {
        tx.send(bucket).expect("Failed to send bucket to workers!");
    });

    println!("Done sending buckets: {}!", num_buckets);
    println!("Starting event loop!");

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_wait();

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => control_flow.set_exit(),
            _ => (),
        }

        if let Some(bucket) = rx.try_recv().ok() {
            for bp_x in 0..bucket.width {
                for bp_y in 0..bucket.height {
                    let (abs_x, abs_y) = (bucket.x + bp_x, bucket.y + bp_y);
                    let abs_idx = cti(abs_x, abs_y, WIDTH);
                    pixels.frame_mut()[abs_idx] = bucket.buffer[cti(bp_x, bp_y, bucket.width)];
                    pixels.frame_mut()[abs_idx + 1] =
                        bucket.buffer[cti(bp_x, bp_y, bucket.width) + 1];
                    pixels.frame_mut()[abs_idx + 2] =
                        bucket.buffer[cti(bp_x, bp_y, bucket.width) + 2];
                    pixels.frame_mut()[abs_idx + 3] =
                        bucket.buffer[cti(bp_x, bp_y, bucket.width) + 3];
                }
            }
            pixels
                .render()
                .expect("Failed to render pixels... this can't be good.");
            num_buckets -= 1;
            print!("\r{} buckets left!", num_buckets);
            if num_buckets == 0 {
                println!("\nDone rendering!");
            }
        }
    });
}

fn ray_trace_bucket(bucket: &Bucket) -> Bucket {
    let mut buffer = vec![0; 4 * (bucket.width * bucket.height) as usize];
    for x in 0..bucket.width {
        for y in 0..bucket.height {
            let abs_x = bucket.x + x;
            let abs_y = bucket.y + y;
            let color = ray_tracer::ray_trace_pixel(
                abs_x,
                abs_y,
                bucket.window_width,
                bucket.window_height,
            );
            let idx = cti(x, y, bucket.width);
            buffer[idx] = color[0];
            buffer[idx + 1] = color[1];
            buffer[idx + 2] = color[2];
            buffer[idx + 3] = color[3];
        }
    }
    let mut new_bucket = bucket.clone();
    new_bucket.set_buffer(buffer);
    new_bucket
}

fn cti(x: u32, y: u32, width: u32) -> usize {
    (4 * (y * width + x)) as usize
}
