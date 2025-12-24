use bitvector::BitVector;

pub struct BitMap {
    data: BitVector,
    columns: usize,
    rows: usize,
}

pub struct BitMapIter {
    index: usize,
    rows: usize,
    columns: usize,
}

impl BitMap {
    pub fn new(columns: usize, rows: usize) -> BitMap {
        BitMap {
            data: BitVector::new(columns * rows),
            columns,
            rows,
        }
    }

    fn index_from_pos(&self, x: usize, y: usize) -> Option<usize> {
        if x >= self.columns || y >= self.rows {
            None
        } else {
            Some(x + y * self.columns)
        }
    }

    pub fn exists(&self, x: usize, y: usize) -> bool {
        self.index_from_pos(x, y)
            .map_or(false, |i| self.data.contains(i))
    }

    pub fn add(&mut self, x: usize, y: usize) -> bool {
        self.index_from_pos(x, y)
            .map_or(false, |i| self.data.insert(i))
    }

    pub fn remove(&mut self, x: usize, y: usize) -> bool {
        self.index_from_pos(x, y)
            .map_or(false, |i| self.data.remove(i))
    }

    pub fn coordinates(&self) -> BitMapIter {
        BitMapIter {
            index: 0,
            rows: self.rows,
            columns: self.columns,
        }
    }

    pub fn num_entities(&self) -> usize {
        self.data.len()
    }

    pub fn size(&self) -> (usize, usize) {
        (self.columns, self.rows)
    }
}

impl Iterator for BitMapIter {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let count = self.rows * self.columns;
        if self.index >= count {
            None
        } else {
            let x = self.index % self.columns;
            let y = self.index / self.columns;

            self.index += 1;
            Some((x, y))
        }
    }
}
