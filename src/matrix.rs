use crate::error::BoundError;
use std::fmt::Debug;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Matrix<T>
where
    T: Clone + Debug + Default + serde::ser::Serialize,
{
    content: Vec<Vec<T>>,
    pub x_dim: usize,
    pub y_dim: usize,
}

impl<T: Debug + Default + Clone + serde::ser::Serialize> Matrix<T> {
    /// Constructor for the matrix ADT
    pub fn new(x_dim: usize, y_dim: usize) -> Self {
        Self {
            content: vec![vec![T::default(); x_dim]; y_dim],
            x_dim,
            y_dim,
        }
    }

    /// Insert an element into the matrix at (x, y)
    /// and send an error result if the conte
    pub fn add(&mut self, x: usize, y: usize, value: T) -> Result<(), BoundError> {
        if x > self.x_dim || y > self.y_dim {
            return Err(BoundError::Exceed(x, y));
        }

        // It is safe to just unwrap here, we checked the bounds above
        *self.content.get_mut(x).unwrap().get_mut(y).unwrap() = value;
        Ok(())
    }

    /// Override an element from the matrix at (x, y)
    /// and insert the default value to take it's place
    pub fn remove(&mut self, x: usize, y: usize) -> Result<(), BoundError> {
        Ok(self.add(x, y, T::default())?)
    }

    /// Get an immutable reference to an element at (x, y)
    pub fn get(&self, x: usize, y: usize) -> Result<&T, BoundError> {
        if x > self.x_dim || y > self.y_dim {
            return Err(BoundError::Exceed(x, y));
        }

        // It is safe to just unwrap here, we checked the bounds above
        Ok(self.content.get(x).unwrap().get(y).unwrap())
    }

    /// Obtain a Vector of immutable references to the underlying
    /// container when given the column number
    /// If the column number supplied is too large, exit and complain
    /// through the use of the `BoundError`
    pub fn col(&self, col: usize) -> Result<Vec<&T>, BoundError> {
        if col > self.y_dim {
            return Err(BoundError::Exceed(0, col));
        }

        let mut container = vec![];
        for pos in 0..self.y_dim {
            container.push(self.get(pos, col).unwrap());
        }

        Ok(container)
    }
    /// Obtain a Vector of immutable references to the underlying
    /// container when given the row number.
    /// If the row number supplied is too large, exit and complain
    /// through the use of the `BoundError`
    pub fn row(&self, row: usize) -> Result<Vec<&T>, BoundError> {
        if row > self.x_dim {
            return Err(BoundError::Exceed(self.x_dim, self.y_dim));
        }

        let mut container = vec![];
        for pos in 0..self.x_dim {
            container.push(self.get(row, pos).unwrap());
        }

        Ok(container)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    /// Ensure the dimensions of the container are correct
    fn test_contstructor() {
        let (x_dim, y_dim) = (8, 10);
        let matrix = Matrix::<usize>::new(x_dim, y_dim);

        assert_eq!(x_dim, matrix.x_dim);
        assert_eq!(x_dim, matrix.content.get(0).unwrap().len());
        assert_eq!(y_dim, matrix.y_dim);
        assert_eq!(y_dim, matrix.content.len());
    }

    #[test]
    fn test_insert() {
        let (x_dim, y_dim) = (8, 10);
        let mut matrix = Matrix::<usize>::new(x_dim, y_dim);

        for x in 0..x_dim {
            for y in 0..3 {
                matrix.add(x, y, x + y).unwrap();
            }
        }

        for x in 0..matrix.x_dim {
            for y in 0..3 {
                assert_eq!(*matrix.get(x, y).unwrap(), x + y);
            }
        }
    }

    #[test]
    fn test_remove() {
        let (x_dim, y_dim) = (8, 10);
        let mut matrix = Matrix::<usize>::new(x_dim, y_dim);
        matrix.add(0, 0, 10_usize).unwrap();
        matrix.remove(0, 0).unwrap();
        assert_eq!(*matrix.get(0, 0).unwrap(), usize::default());
    }

    #[test]
    fn test_serializer() {
        let matrix = Matrix::<usize>::new(8_usize, 10_usize);
        let content = serde_json::to_string(&matrix).unwrap();
        //println!("Serialized content: {content}");
        let _deserialized_matrix = serde_json::from_str::<Matrix<usize>>(&content).unwrap();
        assert_eq!(matrix.x_dim, _deserialized_matrix.x_dim);
        assert_eq!(matrix.y_dim, _deserialized_matrix.y_dim);
        assert_eq!(matrix.content, _deserialized_matrix.content);
    }

    #[test]
    #[should_panic(expected = "Exceed(100, 100)")]
    fn test_out_of_bounds() {
        let mut matrix = Matrix::<usize>::new(1, 1);
        matrix.add(100, 100, 0_usize).unwrap();
    }

    #[test]
    fn test_debug() {
        let mut matrix = Matrix::<usize>::new(3, 3);
        for y in 0..matrix.y_dim {
            matrix.add(y, 0, 1).unwrap();
        }
        println!("{:?}", matrix.content);

        for (lhs, rhs) in matrix.col(0).unwrap().iter().zip([1, 1, 1]) {
            assert_eq!(**lhs, rhs);
        }

        for (lhs, rhs) in matrix.row(0).unwrap().iter().zip([1, 0, 0]) {
            assert_eq!(**lhs, rhs);
        }

    }
}