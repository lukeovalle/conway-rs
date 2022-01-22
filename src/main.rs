use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use std::time::Duration;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Conway", 800, 600)
        .position_centered()
        .resizable()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut mapa = conway::Conway::new(5, 10, true);


    let mut event_pump = sdl_context.event_pump().unwrap();

    'game: loop {
        match procesar_entrada(&mut event_pump, &mut mapa) {
            Some(_) => {
                break 'game
            }
            None => {}
        }

        iterar_juego(&mut mapa);

        pintar_mapa(&mut canvas, &mapa);

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn procesar_entrada(event_pump: &mut sdl2::EventPump, mapa: &mut conway::Conway) -> Option<String> {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. } |
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                return Some("Saliendo".to_string())
            }
            _ => {}
        }
    }
    None
}

fn iterar_juego(mapa: &mut conway::Conway) {

}

fn pintar_mapa(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, mapa: &conway::Conway) {
    canvas.set_draw_color(Color::RGB(10, 90, 10));
    canvas.clear();
    canvas.set_draw_color(Color::RGB(255, 255, 255));

    let ancho_ventana = canvas.window().drawable_size().0 as i32;
    let alto_ventana = canvas.window().drawable_size().1 as i32;
    let filas = mapa.ancho() as i32;
    let columnas = mapa.alto() as i32;
    let ancho = ancho_ventana / columnas;
    let alto = alto_ventana / filas;
    

    // rejilla
    for i in 1..(mapa.len()) {
        let st = canvas.draw_line(
            (0 as i32, alto_ventana / filas * i as i32),
            (ancho_ventana as i32, alto_ventana / filas * i as i32)
            );

        match st {
            Err(txt) => { println!("{}", txt); },
            _ => {}
        }
    }
    for i in 1..(mapa.first().unwrap().len()) {
        let st = canvas.draw_line(
            (ancho_ventana / columnas * i as i32, 0 as i32),
            (ancho_ventana / columnas * i as i32, alto_ventana as i32)
            );

        match st {
            Err(txt) => { println!("{}", txt); },
            _ => {}
        }
    }

    // dibujo rectángulos
    for i in 0..(mapa.len()) {
        for j in 0..(mapa.first().unwrap().len()) {
            if mapa[i][j] == false { continue; };

            let st = canvas.fill_rect(
                Rect::new(
                    ancho_ventana / columnas * i as i32,
                    alto_ventana / filas * j as i32,
                    ancho as u32,
                    alto as u32
                    )
                );

            match st {
                Err(txt) => { println!("{}", txt); },
                _ => {}
            }
        }
    }


    canvas.present()
}

