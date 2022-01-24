mod conway;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
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

    let mut pausa = false;

    'game: loop {
        let ahora = Instant::now();

        match procesar_entrada(&mut event_pump) {
            Some(Acción::Salir) => {
                break 'game
            }
            Some(Acción::Pausa) => {
                pausa = !pausa;
            }
            None => {}
        }

        match iterar_juego(&mut mapa, pausa) {
            Ok(_) => {},
            Err(_) => { break 'game;}
        }

        match pintar_mapa(&mut canvas, &mapa) {
            Ok(_) => {},
            Err(_) => { break 'game;}
        }

        let dormir = Duration::new(0, 1_000_000_000u32 / 3).saturating_sub(ahora.elapsed());
        ::std::thread::sleep(dormir);
    }
}

fn procesar_entrada(event_pump: &mut sdl2::EventPump) -> Option<Acción> {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. } |
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                return Some(Acción::Salir)
            }
            Event::KeyDown { keycode: Some(Keycode::P), .. } => {
                return Some(Acción::Pausa)
            }
            _ => {}
        }
    }
    None
}

fn iterar_juego(mapa: &mut conway::Conway, pausa: bool) -> Result<(), anyhow::Error> {
    if pausa { return Ok(()); }

    let mut nuevo = conway::Conway::new(mapa.ancho(), mapa.alto(), false);

    for i in 0..mapa.ancho() {
        for j in 0..mapa.alto() {
            let vecinas = mapa.recorrer_vecinas(i, j).filter(|c| *c).count();

            if mapa.ver_célula(i,j).unwrap() == false && vecinas == 3 {
                nuevo.nacer_célula(i, j)?;
            } else if mapa.ver_célula(i, j).unwrap() && (vecinas == 2 || vecinas == 3) {
                nuevo.nacer_célula(i, j)?;
            } else {
                nuevo.matar_célula(i, j)?;
            }
        }
    }
    
    *mapa = nuevo;

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

//    dbg!(ancho_ventana);
//    dbg!(alto_ventana);
//    dbg!(filas);
//    dbg!(columnas);
//    dbg!(ancho);
//    dbg!(alto);

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
    Pausa
}
