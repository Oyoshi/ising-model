use rand::RngExt;

pub trait Ising {
    fn run_sweep<R: rand::Rng>(&mut self, rng: &mut R, temp: f64);
    fn calculate_magnetization(&self) -> f64;
}

pub struct MetropolisTable {
    pub prob4: f64,
    pub prob8: f64,
}

pub struct IsingClassic {
    pub grid: Vec<Vec<i8>>,
    pub n: usize,
    pub size: usize,
}

impl IsingClassic {
    pub fn new(n: usize) -> Self {
        let grid = vec![vec![1; n]; n]; // cold start - all spins are up
        let size = n * n;
        IsingClassic { grid, n, size }
    }
}

impl Ising for IsingClassic {
    fn run_sweep<R>(&mut self, rng: &mut R, temp: f64)
    where
        R: rand::Rng,
    {
        let n = self.n;
        for _ in 0..self.size {
            let x = rng.random_range(0..n);
            let y = rng.random_range(0..n);

            let s_i = self.grid[y][x] as i32;
            let sum = self.grid[y][(x + 1) % n] as i32
                + self.grid[y][(x + n - 1) % n] as i32
                + self.grid[(y + 1) % n][x] as i32
                + self.grid[(y + n - 1) % n][x] as i32;

            let de = 2 * s_i * sum;

            if de <= 0 || rng.random::<f64>() < (-(de as f64) / temp).exp() {
                self.grid[y][x] *= -1;
            }
        }
    }

    fn calculate_magnetization(&self) -> f64 {
        let sum: i64 = self
            .grid
            .iter()
            .flat_map(|row: &Vec<i8>| row.iter())
            .map(|&s| s as i64)
            .sum();
        (sum as f64) / (self.size as f64)
    }
}

// use lookup tables to avoid repeated exp() callculations
impl MetropolisTable {
    pub fn new(temp: f64) -> Self {
        MetropolisTable {
            prob4: (-4.0 / temp).exp(),
            prob8: (-8.0 / temp).exp(),
        }
    }
}

pub struct IsingOptimized {
    pub grid: Vec<i8>,
    pub n: usize,
    pub size: usize,
    pub mask: usize,
}

impl IsingOptimized {
    pub fn new(n: usize) -> Self {
        assert!(n.is_power_of_two(), "n must be power of two");
        let size = n * n;
        // stored in row-major order for better cache performance
        let grid = vec![1; size];
        IsingOptimized {
            grid,
            n,
            size,
            mask: n - 1,
        }
    }

    #[inline(always)]
    fn get_index(&self, x: usize, y: usize) -> usize {
        y * self.n + x
    }
}

impl Ising for IsingOptimized {
    fn run_sweep<R>(&mut self, rng: &mut R, temp: f64)
    where
        R: rand::Rng,
    {
        let temp_lookup_table = MetropolisTable::new(temp);
        let n = self.n;
        let mask = self.mask;
        for _ in 0..self.size {
            let x = rng.random_range(0..n);
            let y = rng.random_range(0..n);

            let s_i = self.grid[self.get_index(x, y)] as i32;
            // using masking instead of modulo for periodic boundaries
            let sum = self.grid[self.get_index((x + 1) & mask, y)] as i32
                + self.grid[self.get_index((x + n - 1) & mask, y)] as i32
                + self.grid[self.get_index(x, (y + 1) & mask)] as i32
                + self.grid[self.get_index(x, (y + n - 1) & mask)] as i32;

            let de = 2 * s_i * sum;

            if de <= 0
                || (de == 4 && rng.random::<f64>() < temp_lookup_table.prob4)
                || (de == 8 && rng.random::<f64>() < temp_lookup_table.prob8)
            {
                let idx = self.get_index(x, y);
                self.grid[idx] *= -1;
            }
        }
    }

    fn calculate_magnetization(&self) -> f64 {
        let sum: i64 = self.grid.iter().map(|&s| s as i64).sum();
        (sum as f64) / (self.size as f64)
    }
}
