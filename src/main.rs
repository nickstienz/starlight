#![deny(clippy::all)]
#![deny(unsafe_code)]
use crossbeam_channel::bounded;
use num_cpus;
use pixels::{Pixels, SurfaceTexture};
use std::thread;
use winit::{
    dpi::{LogicalSize, PhysicalSize},
    event::{Event, WindowEvent},
};

const WIDTH: u32 = 100;
const HEIGHT: u32 = 100;
const BUCKET_SIZE: u32 = 16;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Bucket {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub window_width: u32,
    pub window_height: u32,
    pub buffer: Vec<u8>,
}

impl Bucket {
    fn new(
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        window_width: u32,
        window_height: u32,
    ) -> Bucket {
        Bucket {
            x,
            y,
            width,
            height,
            window_width,
            window_height,
            buffer: vec![0; (width * height * 4) as usize],
        }
    }

    fn set_buffer(&mut self, buffer: Vec<u8>) {
        self.buffer = buffer;
    }
}

fn main() {
    // Window stuff I don't understand nor care about
    let event_loop = winit::event_loop::EventLoop::new();

    let window = winit::window::WindowBuilder::new()
        .with_title("Starlight Ray Tracer")
        .build(&event_loop)
        .expect("Failed to create window");
    let logical_size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
    let scale_factor = window.scale_factor();
    let physical_size: PhysicalSize<u32> = logical_size.to_physical(scale_factor);
    window.set_inner_size(physical_size);

    let (window_width, window_height) = {
        let size = window.inner_size();
        (size.width, size.height)
    };

    println!("Window Size: {}x{}", window_width, window_height);

    let texture = SurfaceTexture::new(window_width, window_height, &window);
    let mut pixels =
        Pixels::new(window_width, window_height, texture).expect("Failed to create pixels context");

    // Number of times I have said "fuck" because of this: 29
    let buckets = (0..window_width)
        .step_by(BUCKET_SIZE as usize)
        .flat_map(|x| {
            (0..window_height)
                .step_by(BUCKET_SIZE as usize)
                .map(move |y| {
                    let bucket_width = BUCKET_SIZE.min(window_width - x);
                    let bucket_height = BUCKET_SIZE.min(window_height - y);
                    Bucket::new(
                        x,
                        y,
                        bucket_width,
                        bucket_height,
                        window_width,
                        window_height,
                    )
                })
        })
        .collect::<Vec<_>>();

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
                tx.send(ray_trace(&bucket))
                    .expect("Oh no, something went wrong sending a bucket!");
            }
        }));
    }

    // Send buckets to render
    buckets.into_iter().for_each(|bucket| {
        // println!("Sending Bucket!");
        tx.send(bucket).expect("Failed to send bucket to workers!");
    });

    println!("Done sending buckets!");

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
            println!("{:?}", pixels.frame().len());
            println!("{:?}", bucket.buffer.len());

            for x in 0..bucket.width / 4 {
                for y in 0..bucket.height / 4 {
                    let frame = pixels.frame_mut();
                    let i = coords_to_index(bucket.x + x, bucket.y + y, window_width);
                    let j = coords_to_index(x, y, window_width);
                    frame[i] = bucket.buffer[j];
                    frame[i + 1] = bucket.buffer[j + 1];
                    frame[i + 2] = bucket.buffer[j + 2];
                    frame[i + 3] = bucket.buffer[j + 3];
                }
            }

            pixels.render().unwrap();
        }
    });
}

fn ray_trace(bucket: &Bucket) -> Bucket {
    // println!("Ray Tracing Buceket!");
    let mut new_bucket = bucket.clone();
    new_bucket.set_buffer(vec![255; (bucket.width * bucket.height * 4) as usize]);
    new_bucket
}

fn coords_to_index(x: u32, y: u32, width: u32) -> usize {
    (x + y * width) as usize
}
