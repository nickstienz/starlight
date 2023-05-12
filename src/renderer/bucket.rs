use super::settings::BUCKET_SIZE;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bucket {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub window_width: u32,
    pub window_height: u32,
    pub buffer: Vec<u8>,
}

impl Bucket {
    pub fn new(
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
            buffer: vec![0; 4 * (width * height) as usize],
        }
    }

    pub fn set_buffer(&mut self, buffer: Vec<u8>) {
        self.buffer = buffer;
    }
}

pub fn create_buckets(window_width: u32, window_height: u32) -> Vec<Bucket> {
    (0..window_width)
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
        .collect::<Vec<_>>()
}
