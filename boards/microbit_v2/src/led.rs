use capsules::led_matrix::LedMatrixDriver;
use kernel::hil::gpio::Pin;
use kernel::hil::led::Led;
use kernel::hil::time::Alarm;

// one Led from the matrix
pub struct LedFromMatrix<'a, L: Pin, A: Alarm<'a>> {
    matrix: &'a LedMatrixDriver<'a, L, A>,
    row: usize,
    col: usize,
}

impl<'a, L: Pin, A: Alarm<'a>> LedFromMatrix<'a, L, A> {
    pub fn new(matrix: &'a LedMatrixDriver<'a, L, A>, row: usize, col: usize) -> Self {
        LedFromMatrix { matrix, row, col }
    }
}

impl<'a, L: Pin, A: Alarm<'a>> Led for LedFromMatrix<'a, L, A> {
    fn init(&self) {}

    fn on(&self) {
        self.matrix.on(self.row, self.col);
    }

    fn off(&self) {
        self.matrix.off(self.row, self.col);
    }

    fn toggle(&self) {
        self.matrix.toggle(self.row, self.col);
    }

    fn read(&self) -> bool {
        self.matrix.read(self.row, self.col)
    }
}
