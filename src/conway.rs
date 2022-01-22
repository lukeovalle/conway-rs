pub struct Conway {
    mapa: Vec<Vec<bool>>; // arreglo (columna) de filas de bool
}

struct CélulaIter<'a, It> {
    vecinos: &'a [bool]
}

impl Conway {
    pub fn new(ancho: usize, alto: usize, aleatorio: bool) -> Conway {
        let mapa: Vec<Vec<bool>> = Vec::with_capacity(alto);

        for i in 0..alto {
            mapa.push(Vec::with_capacity(ancho));

            for j in 0..ancho {
                mapa.last().unwrap().push(if aleatorio { rand::random() } else { false });
            }
        }

        Conway { mapa }
    }

    pub fn alto() -> usize {
        mapa.len()
    }

    pub fn ancho() -> usize {
        mapa.first().unwrap().len()
    }

    pub fn matar_célula(x, y) {
        mapa[x][y] = false;
    }

    pub fn nacer_célula(x, y) {
        mapa[x][y] = true;
    }

    pub fn recorrer_vecinas(x, y) -> std::iter::Iterator<Item=bool> {
    }
}


