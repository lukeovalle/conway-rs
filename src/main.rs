mod conway;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::rect::Rect;
use std::time::{Duration, Instant};
use anyhow::anyhow;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Conway", 800, 600)
        .position_centered()
        .resizable()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut mapa = conway::Conway::new(40, 40, true);

    let mut event_pump = sdl_context.event_pump().unwrap();

    // Quiero que vaya a 60fps pero haga 3 iteraciones por segundo
    let tiempo_por_frame = Duration::from_nanos(1_000_000_000u64 / 60);
    let tiempo_por_tick = Duration::from_nanos(1_000_000_000u64 / 3);

    let mut tiempo_que_pasó = Duration::ZERO;
    let mut pausa = false;

    'game: loop {
        let ahora = Instant::now();

        match procesar_entrada(&mut event_pump, &canvas, &mapa) {
            Some(Acción::Salir) => {
                break 'game
            }
            Some(Acción::Pausa) => {
                pausa = !pausa;
            }
            Some(Acción::CrearCélula { x, y }) => {
                if let Err(e) = mapa.crear_célula(x, y) {
                    println!("{}", e);
                }
            }
            Some(Acción::MatarCélula { x, y }) => {
                if let Err(e) = mapa.matar_célula(x, y) {
                    println!("{}", e);
                }
            }
            Some(Acción::LimpiarMapa) => {
                mapa = conway::Conway::new(mapa.ancho(), mapa.alto(), false)
            }
            Some(Acción::AleatorizarMapa) => {
                mapa = conway::Conway::new(mapa.ancho(), mapa.alto(), true)
            }
            None => {}
        }

        while tiempo_que_pasó >= tiempo_por_tick {
            if let Err(e) = iterar_juego(&mut mapa, pausa) {
                dbg!(e);
                break 'game;
            }
            tiempo_que_pasó -= tiempo_por_tick;
        }

        if let Err(_) = pintar_mapa(&mut canvas, &mapa) {
            break 'game;
        }

        ::std::thread::sleep(tiempo_por_frame.saturating_sub(ahora.elapsed()));
        tiempo_que_pasó += ahora.elapsed();
    }
}

fn procesar_entrada(
    event_pump: &mut sdl2::EventPump,
    canvas: &sdl2::render::Canvas<sdl2::video::Window>,
    mapa: &conway::Conway
) -> Option<Acción> {
    let ancho_ventana = canvas.window().drawable_size().0 as usize;
    let alto_ventana = canvas.window().drawable_size().1 as usize;
    let filas = mapa.alto();
    let columnas = mapa.ancho();
    let ancho = ancho_ventana / columnas;
    let alto = alto_ventana / filas;
 
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. } |
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                return Some(Acción::Salir)
            }
            Event::KeyDown { keycode: Some(Keycode::P), .. } => {
                return Some(Acción::Pausa)
            }
            Event::KeyDown { keycode: Some(Keycode::C), .. } => {
                return Some(Acción::LimpiarMapa)
            }
            Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                return Some(Acción::AleatorizarMapa)
            }
            Event::MouseButtonDown { mouse_btn: MouseButton::Left, x, y, .. } => {
                return Some(Acción::CrearCélula {
                    x: x as usize / ancho,
                    y: y as usize / alto
                })
            }
            Event::MouseButtonDown { mouse_btn: MouseButton::Right, x, y, .. } => {
                return Some(Acción::MatarCélula {
                    x: x as usize / ancho,
                    y: y as usize / alto
                })
            }
            _ => {}
        }
    }
    None
}

fn iterar_juego(mapa: &mut conway::Conway, pausa: bool) -> Result<(), anyhow::Error> {


    if !pausa {
        return mapa.iterar_mapa();
    }

    Ok(())
}

fn pintar_mapa(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    mapa: &conway::Conway
) -> Result<(), anyhow::Error> {

    canvas.set_draw_color(Color::RGB(10, 70, 10));
    canvas.clear();

    let ancho_ventana = canvas.window().drawable_size().0 as i32;
    let alto_ventana = canvas.window().drawable_size().1 as i32;
    let filas = mapa.alto() as i32;
    let columnas = mapa.ancho() as i32;
    let ancho = ancho_ventana / columnas;
    let alto = alto_ventana / filas;

    // rejilla
    canvas.set_draw_color(Color::RGB(0, 0, 0));

    for i in 1..columnas {
        canvas.draw_line(
            (ancho * i as i32, 0),
            (ancho * i as i32, alto_ventana)
        ).map_err(|e| anyhow!(e))?;
    }
    for i in 1..filas {
        canvas.draw_line(
            (0, alto * i as i32),
            (ancho_ventana, alto * i as i32)
        ).map_err(|e| anyhow!(e))?;
    }

    // dibujo rectángulos
    canvas.set_draw_color(Color::RGB(10, 180, 10));
    for i in 0..columnas {
        for j in 0..filas {
            if mapa.ver_célula(i as usize, j as usize).unwrap() == false { continue; };

            canvas.fill_rect(
                Rect::new(
                    ancho_ventana / columnas * i as i32,
                    alto_ventana / filas * j as i32,
                    ancho as u32,
                    alto as u32
                )
            ).map_err(|e| anyhow!(e))?;
        }
    }


    canvas.present();

    Ok(())
}

enum Acción {
    Salir,
    Pausa,
    CrearCélula { x: usize, y: usize },
    MatarCélula { x: usize, y: usize },
    LimpiarMapa,
    AleatorizarMapa
}
