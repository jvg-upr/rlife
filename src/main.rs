use life::{Life, LifeGrid};
use log::{debug, info};
use minifb::{Key, KeyRepeat, MouseButton, MouseMode, Window, WindowOptions};
use std::sync::{mpsc, Arc, RwLock};
use std::time::{Duration, Instant};

// width of map grid
const WIDTH: usize = 64;

// height of map grid
const HEIGHT: usize = 36;

// time per step of simulation
const SIM_STEP_TIME: Duration = Duration::from_millis(300);

// simulate life
fn sim_task<T: Life>(
    grid: Arc<RwLock<Box<T>>>,
    mut scratch_grid: Box<T>,
    pause_chan: mpsc::Receiver<()>,
) {
    // time last update was made
    let mut last_update = Instant::now();
    loop {
        match pause_chan.try_recv() {
            // wait till resume signal
            Ok(_) => {
                info!("received pause signal");
                let _ = pause_chan.recv();
                info!("received resume signal");
            }
            // simulate next step
            Err(mpsc::TryRecvError::Empty) if SIM_STEP_TIME <= last_update.elapsed() => {
                debug!("generating next generation");
                // generate next generation
                grid.read()
                    .expect("Poisoned")
                    .next_generation(scratch_grid.as_mut());
                // swap next generation with current one
                debug!("updating map");
                std::mem::swap(grid.write().expect("Poisoned").as_mut(), &mut scratch_grid);
                last_update = Instant::now();
            }
            // return on channel disconnection, when program ends
            Err(mpsc::TryRecvError::Disconnected) => {
                info!("received disconnect signal");
                return;
            }
            // do nothing if it's not time for the next step
            _ => (),
        }
    }
}

fn main() {
    env_logger::init();

    info!("starting up");

    // setup window
    info!("setting up window");
    let mut window = Window::new(
        "Conway's Game of Life - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions {
            scale: minifb::Scale::X16,
            ..Default::default()
        },
    )
    .unwrap();
    window.limit_update_rate(Some(Duration::from_millis(30)));

    // setup shared state
    info!("setting up thread shared state");
    let curr = Arc::new(RwLock::new(Box::new(LifeGrid::<WIDTH, HEIGHT>::default())));
    let (pause_tx, pause_rx) = mpsc::channel();

    // setup simulation thread
    info!("setting up simulation thread");
    let life_sim_thread = {
        // clone Arc to share state
        let curr = curr.clone();

        // create new thread
        std::thread::spawn(move || {
            info!("simulation thread started");
            let task = sim_task(curr, Box::new(LifeGrid::<WIDTH, HEIGHT>::default()), pause_rx);
            info!("simulation thread finished");
            task
        })
    };

    // frame buffer
    info!("initializing frame buffer");
    let mut buffer = [0x0; WIDTH * HEIGHT];

    // I/O thread
    info!("starting I/O handling");

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // manage user input

        // keyboard input
        // pause/resume simulation
        if window.is_key_pressed(Key::Space, KeyRepeat::No) {
            debug!("sending simulation toggle signal");
            pause_tx.send(()).unwrap();
        }

        // mouse input
        // set selected cell alive/dead
        if let Some((x, y)) = window
            .get_mouse_pos(MouseMode::Discard)
            .map(|(x, y)| (x as usize, y as usize))
        {
            if window.get_mouse_down(MouseButton::Left) {
                debug!("setting Cell ({}, {}): alive", x, y);
                curr.write().expect("Poisoned").set_cell(x, y, true);
            } else if window.get_mouse_down(MouseButton::Right) {
                debug!("setting Cell ({}, {}): dead", x, y);
                curr.write().expect("Poisoned").set_cell(x, y, false);
            }
        }

        // update screen
        let curr = curr.read().expect("poisoned");
        let life_it = curr.iter().flat_map(|row| row.iter());

        // update buffer
        buffer
            .iter_mut()
            .zip(life_it)
            .for_each(|(cell, is_alive)| *cell = u32::MAX * *is_alive as u32);

        // update screen with buffer
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }

    // drop channel as signal that program has ended
    info!("sending shutdown signal to simulation thread");
    drop(pause_tx);

    // wait for simulation thread
    info!("waiting for simulation thread to finish");
    life_sim_thread.join().unwrap();
}
