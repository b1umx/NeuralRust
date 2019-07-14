use std::io;
use std::string;
use std::vec;


pub struct Matrix {
    height: usize,
    width: usize,
    array: Vec<Vec<f32>>
}

impl Matrix {
    pub fn new(height: usize, width: usize) -> Matrix {
        let mut array = Vec::new();
        for _i in 0..height {
            let mut row = Vec::new();
            for _j in 0..width {
                row.push(0.0);
            }
            array.push(row);
        }

        Matrix {
            height: height,
            width: width,
            array: array
        }
    }

    pub fn from_array(array: Vec<Vec<f32>>) -> Matrix {
        Matrix {
            height: array.len(),
            width: array[0].len(),
            array: array
        }
    }

    pub fn multiply(&self, factor: f32) -> Matrix {
        let mut result = Matrix::new(self.height, self.width);

        for i in 0..self.height {
            for j in 0..self.width {
                result.array[i][j] = factor * self.array[i][j];
            }
        }

        result
    }

    pub fn add(&self, other: &Matrix) -> Matrix {
        assert!(self.height == other.height && self.width == other.width);

        let mut result = Matrix::new(self.height, self.width);
        for i in 0..self.height {
            for j in 0..self.width {
                result.array[i][j] = self.array[i][j] + other.array[i][j];
            }
        }

        result
    }

    pub fn subtract(&self, other: &Matrix) -> Matrix {
        assert!(self.height == other.height && self.width == other.width);
        
        let mut result = Matrix::new(self.height, self.width);
        for i in 0..self.height {
            for j in 0..self.width {
                result.array[i][j] = self.array[i][j] - other.array[i][j];
            }
        }

        result
    }
    
    pub fn matrix_multiply(&self, other: &Matrix) -> Matrix {
        assert!(self.height == other.height && self.width == other.width);
        
        let mut result = Matrix::new(self.height, self.width);
        for i in 0..self.height {
            for j in 0..self.width {
                result.array[i][j] = self.array[i][j] * other.array[i][j];
            }
        }

        result
    }

    pub fn dot(&self, other: &Matrix) -> Matrix {
        assert_eq!(self.width, other.height);

        let mut result = Matrix::new(self.height, other.width);
        let mut elem: f32 = 0.0;
        for i in 0..self.height {
            for j in 0..other.width {
                for h in 0..self.width {
                    elem += self.array[i][h] * other.array[h][j];
                }
                result.array[i][j] = elem;
                elem = 0.0;
            }
        }

        result
    }

    pub fn transpose(&self) -> Matrix {
        let mut result = Matrix::new(self.width, self.height); for i in 0..self.width {
            for j in 0..self.height {
                result.array[i][j] = self.array[j][i];
            }
        }

        result
    }

    pub fn apply_function(&self, function: fn(f32) -> f32) -> Matrix {
        let mut result = Matrix::new(self.height, self.width);
        for i in 0..self.height {
            for j in 0..self.width {
                result.array[i][j] = function(self.array[i][j]);
            }
        }

        result
    }

    pub fn print(&self, stream: &mut io::Write) {
        let mut max_length: vec::Vec<i32> = vec::Vec::new();
        for n in 0..self.width {
            max_length.push(0);
        }

        for i in 0..self.height {
            for j in 0..self.width {
                let elem = self.array[i][j].to_string();
                let elem_size = elem.len() as i32;
                if max_length[j] < elem_size {
                    max_length[j] = elem_size;
                }
            }
        }

        for i in 0..self.height {
            for j in 0..self.width {
                let elem = &self.array[i][j].to_string();
                let mut buf = string::String::new();
                if j > 0 {
                    buf.push(' ');
                }
                let delta = max_length[j] - (elem.len() as i32);
                for k in 0..delta {
                    buf.push(' ');
                }
                buf.push_str(elem);
                stream.write(buf.as_bytes());
            }
            stream.write(b"\n");
        }
    }
}

#[test]
fn test_new() {
    let mut m = Matrix::new(2, 3);
    m.array[0][0] = 0.0;
    m.array[0][1] = 1.0;
    m.array[0][2] = 2.0;
    m.array[1][0] = 0.5;
    m.array[1][1] = 1.5;
    m.array[1][2] = 2.5;

    assert_eq!(m.height, 2, "Высота установлена некорректно");
    assert_eq!(m.width, 3, "Ширина установлена некорректно");
    assert_eq!(m.array[0][0], 0.0);
    assert_eq!(m.array[0][1], 1.0);
    assert_eq!(m.array[0][2], 2.0);
    assert_eq!(m.array[1][0], 0.5);
    assert_eq!(m.array[1][1], 1.5);
    assert_eq!(m.array[1][2], 2.5);
}

#[test]
fn test_from_array() {
    let v = vec![
        vec![1.0, 2.0, 3.0], 
        vec![4.0, 5.0, 6.0]
    ];
    let m = Matrix::from_array(v);

    assert_eq!(m.height, 2, "Высота установлена некорректно");
    assert_eq!(m.width, 3, "Ширина установлена некорректно");
    assert_eq!(m.array[0][0], 1.0);
    assert_eq!(m.array[0][1], 2.0);
    assert_eq!(m.array[0][2], 3.0);
    assert_eq!(m.array[1][0], 4.0);
    assert_eq!(m.array[1][1], 5.0);
    assert_eq!(m.array[1][2], 6.0);
}

#[test]
fn test_multiply() {
    let m = Matrix::from_array(vec![
        vec![1.0, 2.0, 3.0], 
        vec![4.0, 5.0, 6.0]
    ]);

    let fm = m.multiply(2.0);

    assert_eq!(fm.height, 2, "Высота установлена некорректно");
    assert_eq!(fm.width, 3, "Ширина установлена некорректно");
    assert_eq!(fm.array[0][0], 2.0);
    assert_eq!(fm.array[0][1], 4.0);
    assert_eq!(fm.array[0][2], 6.0);
    assert_eq!(fm.array[1][0], 8.0);
    assert_eq!(fm.array[1][1], 10.0);
    assert_eq!(fm.array[1][2], 12.0);
}

#[test]
fn test_add() {
    let m1 = Matrix::from_array(vec![
        vec![1.0, 2.0, 3.0], 
        vec![4.0, 5.0, 6.0]
    ]);
    let m2 = Matrix::from_array(vec![
        vec![2.0, 3.0, 4.0], 
        vec![5.0, 6.0, 7.0]
    ]);

    let fm = m1.add(&m2);

    assert_eq!(fm.height, 2, "Высота установлена некорректно");
    assert_eq!(fm.width, 3, "Ширина установлена некорректно");
    assert_eq!(fm.array[0][0], 3.0);
    assert_eq!(fm.array[0][1], 5.0);
    assert_eq!(fm.array[0][2], 7.0);
    assert_eq!(fm.array[1][0], 9.0);
    assert_eq!(fm.array[1][1], 11.0);
    assert_eq!(fm.array[1][2], 13.0);
}

#[test]
fn test_subtract() {
    let m1 = Matrix::from_array(vec![
        vec![3.0, 5.0, 7.0], 
        vec![9.0, 11.0, 13.0]
    ]);
    let m2 = Matrix::from_array(vec![
        vec![2.0, 3.0, 4.0], 
        vec![5.0, 6.0, 7.0]
    ]);

    let fm = m1.subtract(&m2);

    assert_eq!(fm.height, 2, "Высота установлена некорректно");
    assert_eq!(fm.width, 3, "Ширина установлена некорректно");
    assert_eq!(fm.array[0][0], 1.0);
    assert_eq!(fm.array[0][1], 2.0);
    assert_eq!(fm.array[0][2], 3.0);
    assert_eq!(fm.array[1][0], 4.0);
    assert_eq!(fm.array[1][1], 5.0);
    assert_eq!(fm.array[1][2], 6.0);
}

#[test]
fn test_matrix_multiply() {
    let m1 = Matrix::from_array(vec![
        vec![3.0, 5.0, 7.0], 
        vec![9.0, 11.0, 13.0]
    ]);
    let m2 = Matrix::from_array(vec![
        vec![-2.0, 1.0, -1.0], 
        vec![2.0, 0.0, 3.0]
    ]);

    let fm = m1.matrix_multiply(&m2);

    assert_eq!(fm.height, 2, "Высота установлена некорректно");
    assert_eq!(fm.width, 3, "Ширина установлена некорректно");
    assert_eq!(fm.array[0][0], -6.0);
    assert_eq!(fm.array[0][1], 5.0);
    assert_eq!(fm.array[0][2], -7.0);
    assert_eq!(fm.array[1][0], 18.0);
    assert_eq!(fm.array[1][1], 0.0);
    assert_eq!(fm.array[1][2], 39.0);
}

#[test]
fn test_dot() {
    let m1 = Matrix::from_array(vec![
        vec![1.0, 2.0, 3.0], 
        vec![3.0, 2.0, 1.0]
    ]);
    let m2 = Matrix::from_array(vec![
        vec![2.0, 3.0], 
        vec![4.0, 5.0],
        vec![4.0, 3.0]
    ]);

    let fm = m1.dot(&m2);

    assert_eq!(fm.height, 2, "Высота установлена некорректно");
    assert_eq!(fm.width, 2, "Ширина установлена некорректно");
    assert_eq!(fm.array[0][0], 22.0);
    assert_eq!(fm.array[0][1], 22.0);
    assert_eq!(fm.array[1][0], 18.0);
    assert_eq!(fm.array[1][1], 22.0);
}

#[test]
fn test_transpose() {
    let m = Matrix::from_array(vec![
        vec![1.0, 2.0, 3.0], 
        vec![4.0, 5.0, 6.0]
    ]);

    let fm = m.transpose();

    assert_eq!(fm.height, 3, "Высота установлена некорректно");
    assert_eq!(fm.width, 2, "Ширина установлена некорректно");
    assert_eq!(fm.array[0][0], 1.0);
    assert_eq!(fm.array[0][1], 4.0);
    assert_eq!(fm.array[1][0], 2.0);
    assert_eq!(fm.array[1][1], 5.0);
    assert_eq!(fm.array[2][0], 3.0);
    assert_eq!(fm.array[2][1], 6.0);
}

#[test]
fn test_apply_function() {
    let m = Matrix::from_array(vec![
        vec![1.0, 2.0, 3.0], 
        vec![4.0, 5.0, 6.0]
    ]);

    fn add_two(val: f32) -> f32 {
        val + 2.0
    }

    let fm = m.apply_function(add_two);

    assert_eq!(fm.height, 2, "Высота установлена некорректно");
    assert_eq!(fm.width, 3, "Ширина установлена некорректно");
    assert_eq!(fm.array[0][0], 3.0);
    assert_eq!(fm.array[0][1], 4.0);
    assert_eq!(fm.array[0][2], 5.0);
    assert_eq!(fm.array[1][0], 6.0);
    assert_eq!(fm.array[1][1], 7.0);
    assert_eq!(fm.array[1][2], 8.0);
}

#[test]
fn test_print() {
    let m = Matrix::from_array(vec![
        vec![2.0, 2345.0],
        vec![23.0, 1.0]
    ]);
    let mut stream = io::Cursor::new(Vec::new());

    m.print(&mut stream);
    let check_val = b" 2 2345\n23    1\n";
    assert_eq!(&stream.get_ref(), &check_val);
}

