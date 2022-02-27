use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("desbordó de la matriz")]
    Desbordado
}

pub struct CélulaIter {
    vecinos: Vec<bool>,
    actual: usize
}

impl CélulaIter {
    fn new(vecinos: Vec<bool>) -> CélulaIter {
        CélulaIter { vecinos, actual: 0 }
    }
}

impl Iterator for CélulaIter {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        let valor = if self.actual < self.vecinos.len() {
            Some(self.vecinos[self.actual])
        } else {
            None
        };

        self.actual += 1;

        valor
    }
}

pub struct Conway {
    mapa: Vec<Vec<bool>>, // arreglo (fila) de columnas de bool
    siguiente: Vec<Vec<bool>> // siguiente iteración del mapa
}


impl Conway {
    pub fn new(ancho: usize, alto: usize) -> Conway {
        let mut mapa: Vec<Vec<bool>> = Vec::with_capacity(ancho);

        for i in 0..ancho {
            mapa.push(Vec::with_capacity(alto));

            for _j in 0..alto {
                mapa[i].push(false);
            }
        }
        let siguiente = mapa.clone();

        Conway { mapa, siguiente }
    }

    pub fn ancho(&self) -> usize {
        self.mapa.len()
    }

    pub fn alto(&self) -> usize {
        self.mapa.first().unwrap().len()
    }

    pub fn limpiar(&mut self) {
        for i in 0..self.ancho() {
            for j in 0..self.alto() {
                self.mapa[i][j] = false;
            }
        }
    }

    pub fn aleatorizar(&mut self) {
        for i in 0..self.ancho() {
            for j in 0..self.alto() {
                self.mapa[i][j] = rand::random();
            }
        }
    }

    pub fn ver_célula(&self, x: usize, y: usize) -> Result<bool, Error> {
        if x >= self.ancho() || y >= self.alto() {
            Err(Error::Desbordado)
        } else {
            Ok(self.mapa[x][y])
        }
    }

    pub fn matar_célula(&mut self, x: usize, y: usize) -> Result<(), Error> {
        if x >= self.ancho() || y >= self.alto() {
            return Err(Error::Desbordado);
        }

        self.mapa[x][y] = false;

        Ok(())
    }

    pub fn crear_célula(&mut self, x: usize, y: usize) -> Result<(), Error> {
        if x >= self.ancho() || y >= self.alto() {
            return Err(Error::Desbordado);
        }

        self.mapa[x][y] = true;

        Ok(())
    }

    pub fn recorrer_vecinas(&self, x: usize, y: usize) -> CélulaIter {
        let mut vecinos = Vec::with_capacity(8);

        for i in -1..=1 {
            for j in -1..=1 {
                if x as i32 + i < 0
                || x as i32 + i >= self.ancho() as i32
                || y as i32 + j < 0
                || y as i32 + j >= self.alto() as i32
                || (i == 0 && j == 0) {
                    continue;
                }

                vecinos.push(self.mapa[(x as i32 + i) as usize][(y as i32 + j) as usize]);
            }
        }

        CélulaIter::new(vecinos)
    }

    pub fn iterar_mapa(&mut self) -> Result<(), anyhow::Error> {
        for i in 0..self.ancho() {
            for j in 0..self.alto() {
                let vecinas = self.recorrer_vecinas(i, j).filter(|c| *c).count();

                if self.ver_célula(i, j).unwrap() && (vecinas == 2 || vecinas == 3) {
                    self.siguiente[i][j] = true;
                } else {
                    self.siguiente[i][j] = false;
                }
            }
        }
        std::mem::swap(&mut self.mapa, &mut self.siguiente);
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ancho() {
        let mapa = Conway::new(5, 10);

        assert_eq!(mapa.ancho(), 5);
    }

    #[test]
    fn test_alto() {
        let mapa = Conway::new(5, 10);

        assert_eq!(mapa.alto(), 10);
    }

    #[test]
    fn test_limpiar() -> Result<(), anyhow::Error> {
        let mut mapa = Conway::new(5, 10);
        mapa.aleatorizar();

        mapa.limpiar();

        for i in 0..mapa.ancho() {
            for j in 0..mapa.alto() {
                assert_eq!(mapa.ver_célula(i, j)?, false);
            }
        }

        Ok(())
    }

}
