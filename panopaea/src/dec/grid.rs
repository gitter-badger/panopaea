
use math::LinearView;
use ndarray::{Array, ArrayView, ArrayViewMut, Ix1, Ix2, LinalgScalar, Zip};
use sparse::{DiagonalMatrix, SparseMatrix};
use std::ops::Neg;
use domain::Grid2d;
use super::manifold::{Hodge0, Hodge1, Hodge2, Manifold2d};

#[derive(Debug)]
pub struct Staggered2d<T> {
    data: Array<T, Ix1>,
    dim: (usize, usize), // (y, x)
}

impl<T> Staggered2d<T> {
    pub fn dim(&self) -> (usize, usize) {
        self.dim
    }

    /// (vertical, horizontal)
    pub fn split(&self) -> (ArrayView<T, Ix2>, ArrayView<T, Ix2>) {
        let size_0 = self.dim.1 * (self.dim.0 + 1);

        (unsafe { ArrayView::<T, Ix2>::from_shape_ptr((self.dim.0 + 1, self.dim.1), self.data.as_ptr()) },
         unsafe { ArrayView::<T, Ix2>::from_shape_ptr((self.dim.0, self.dim.1 + 1), self.data.as_ptr().offset(size_0 as isize)) })
    }

    /// (vertical, horizontal)
    pub fn split_mut(&mut self) -> (ArrayViewMut<T, Ix2>, ArrayViewMut<T, Ix2>) {
        let size_0 = self.dim.1 * (self.dim.0 + 1);

        (unsafe { ArrayViewMut::<T, Ix2>::from_shape_ptr((self.dim.0 + 1, self.dim.1), self.data.as_mut_ptr()) },
         unsafe { ArrayViewMut::<T, Ix2>::from_shape_ptr((self.dim.0, self.dim.1 + 1), self.data.as_mut_ptr().offset(size_0 as isize)) })
    }
}

impl<T> LinearView for Staggered2d<T> {
    type Elem = T;
    fn view_linear(&self) -> ArrayView<T, Ix1> {
        self.data.view()
    }

    fn view_linear_mut(&mut self) -> ArrayViewMut<T, Ix1> {
        self.data.view_mut()
    }
}

impl<T> Hodge0<T> for Grid2d
where T: LinalgScalar + Neg<Output = T> + Send + Sync
{
    type Simplex0 = Array<T, Ix2>;
    fn apply(&self, dual: &mut Self::Simplex0, primal: &Self::Simplex0) {
        let two = T::one() + T::one();
        let four = two + two;
        let (h, w) = self.dim();

        // corners
        dual[(0, 0)]     = primal[(0, 0)] / four;
        dual[(0, w-1)]   = primal[(0, w-1)] / four;
        dual[(h-1, 0)]   = primal[(h-1, 0)] / four;
        dual[(h-1, w-1)] = primal[(h-1, w-1)] / four;

        // sides
        Zip::from(dual.slice_mut(s![..1, 1..-1]))
            .and(primal.slice(s![..1, 1..-1]))
            .apply(|dual, &primal| {
                *dual = primal / two;
            });

        Zip::from(dual.slice_mut(s![-1.., 1..-1]))
            .and(primal.slice(s![-1.., 1..-1]))
            .apply(|dual, &primal| {
                *dual = primal / two;
            });

        Zip::from(dual.slice_mut(s![1..-1, ..1]))
            .and(primal.slice(s![1..-1, ..1]))
            .apply(|dual, &primal| {
                *dual = primal / two;
            });

        Zip::from(dual.slice_mut(s![1..-1, -1..]))
            .and(primal.slice(s![1..-1, -1..]))
            .apply(|dual, &primal| {
                *dual = primal / two;
            });

        // inner
        Zip::from(dual.slice_mut(s![1..-1, 1..-1]))
            .and(primal.slice(s![1..-1, 1..-1]))
            .apply(|dual, &primal| {
                *dual = primal;
            });
    }
    fn apply_inv(&self, primal: &mut Self::Simplex0, dual: &Self::Simplex0) {
        let two = T::one() + T::one();
        let four = two + two;
        let (h, w) = self.dim();

        // corners
        primal[(0, 0)]     = dual[(0, 0)] * four;
        primal[(0, w-1)]   = dual[(0, w-1)] * four;
        primal[(h-1, 0)]   = dual[(h-1, 0)] * four;
        primal[(h-1, w-1)] = dual[(h-1, w-1)] * four;

        // sides
        Zip::from(primal.slice_mut(s![..1, 1..-1]))
            .and(dual.slice(s![..1, 1..-1]))
            .apply(|primal, &dual| {
                *primal = dual * two;
            });

        Zip::from(primal.slice_mut(s![-1.., 1..-1]))
            .and(dual.slice(s![-1.., 1..-1]))
            .apply(|primal, &dual| {
                *primal = dual * two;
            });

        Zip::from(primal.slice_mut(s![1..-1, ..1]))
            .and(dual.slice(s![1..-1, ..1]))
            .apply(|primal, &dual| {
                *primal = dual * two;
            });

        Zip::from(primal.slice_mut(s![1..-1, -1..]))
            .and(dual.slice(s![1..-1, -1..]))
            .apply(|primal, &dual| {
                *primal = dual * two;
            });

        // inner
        Zip::from(primal.slice_mut(s![1..-1, 1..-1]))
            .and(dual.slice(s![1..-1, 1..-1]))
            .apply(|primal, &dual| {
                *primal = dual;
            });
    }
}

impl<T> Hodge1<T> for Grid2d
where T: LinalgScalar + Neg<Output = T> + Send + Sync
{
    type Simplex1 = Staggered2d<T>;
    fn apply(&self, dual: &mut Self::Simplex1, primal: &Self::Simplex1) {
        let primal = primal.split();
        let mut dual = dual.split_mut();

        Zip::from(&mut dual.0)
            .and(&primal.0)
            .apply(|dual, &primal| {
                *dual = primal;
            });

        Zip::from(&mut dual.1)
            .and(&primal.1)
            .apply(|dual, &primal| {
                *dual = -primal;
            });
    }
    fn apply_inv(&self, primal: &mut Self::Simplex1, dual: &Self::Simplex1) {
        let dual = dual.split();
        let mut primal = primal.split_mut();

        Zip::from(&mut primal.0)
            .and(&dual.0)
            .apply(|primal, &dual| {
                *primal = -dual;
            });

        Zip::from(&mut primal.1)
            .and(&dual.1)
            .apply(|primal, &dual| {
                *primal = dual;
            });
    }
}

impl<T> Hodge2<T> for Grid2d
where T: LinalgScalar
{
    type Simplex2 = Array<T, Ix2>;
    fn apply(&self, dual: &mut Self::Simplex2, primal: &Self::Simplex2) {
        dual.assign(primal);
    }
    fn apply_inv(&self, primal: &mut Self::Simplex2, dual: &Self::Simplex2) {
        primal.assign(dual);
    }
}

impl<T> Manifold2d<T> for Grid2d
    where T: LinalgScalar + Neg<Output = T> + Send + Sync
{
    fn num_elem_0(&self) -> usize {
        (self.dim().0 + 1) * (self.dim().1 + 1)
    }

    fn num_elem_1(&self) -> usize {
        (self.dim().0 + 1) * self.dim().1 + self.dim().0 * (self.dim().1 + 1)
    }

    fn num_elem_2(&self) -> usize {
        self.dim().0 * self.dim().1
    }

    fn new_simplex_0(&self) -> Self::Simplex0 {
        Array::from_elem((self.dim().0 + 1, self.dim().1 + 1), T::zero()) // vertices
    }

    fn new_simplex_1(&self) -> Self::Simplex1 {
        Staggered2d {
            data: Array::from_elem((self.dim().0 + 1) * self.dim().1 + self.dim().0 * (self.dim().1 + 1), T::zero()),
            dim: self.dim(),
        }
    }

    fn new_simplex_2(&self) -> Self::Simplex2 {
        Array::from_elem((self.dim().0, self.dim().1), T::zero()) // faces
    }

    fn derivative_0_primal(&self, edges: &mut Self::Simplex1, vertices: &Self::Simplex0) {
        let mut edges = edges.split_mut();

        par_azip!(
            mut edge (&mut edges.0),
            v0 (vertices.slice(s![.., ..-1])),
            v1 (vertices.slice(s![.., 1..]))
         in { *edge = v1 - v0; });

        par_azip!(
            mut edge (&mut edges.1),
            v0 (vertices.slice(s![..-1, ..])),
            v1 (vertices.slice(s![1.., ..]))
         in { *edge = v1 - v0; });
    }

    fn derivative_0_dual(&self, edges: &mut Self::Simplex1, faces: &Self::Simplex2) {
        let mut edges = edges.split_mut();

        // vertical
        par_azip!(
            mut edge (edges.0.slice_mut(s![1..-1, ..])),
            f0 (faces.slice(s![..-1, ..])),
            f1 (faces.slice(s![1.., ..]))
         in { *edge = -(f1 - f0); });

        // horizontal
        par_azip!(
            mut edge (edges.1.slice_mut(s![.., 1..-1])),
            f0 (faces.slice(s![.., ..-1])),
            f1 (faces.slice(s![.., 1..]))
         in { *edge = f0 - f1; });

    }

    fn derivative_1_primal(&self, faces: &mut Self::Simplex2, edges: &Self::Simplex1) {
        let edges = edges.split();

        par_azip!(
            mut face (faces),
            top    (edges.0.slice(s![..-1,   ..])),
            bottom (edges.0.slice(s![ 1..,   ..])),
            left   (edges.1.slice(s![  .., ..-1])),
            right  (edges.1.slice(s![  .., 1..]))
         in { *face = -bottom + top - left + right; });
    }

    fn derivative_1_dual(&self, faces: &mut Self::Simplex0, edges: &Self::Simplex1) {
        unimplemented!()
    }

    fn derivative_0_primal_matrix(&self) -> SparseMatrix<T> {
        unimplemented!()
        /*
        let dim = (self.num_elem_1(), self.num_elem_0());
        let mut matrix = SparseMatrix::<T>::new(dim);

        let (h, w) = self.dim();
        let one = T::one();
        let mut idx = 0;

        // horizontal edges
        for y in 0..(h+1) {
            for x in 0..w {
                let v_idx = y*(w+1) + x;
                matrix.insert((idx, v_idx), -one);
                matrix.insert((idx, v_idx + 1), one);
                idx += 1;
            }
        }

        // vertical edges
        for y in 0..h {
            for x in 0..(w+1) {
                let v_idx = y*(w+1) + x;
                matrix.insert((idx, v_idx), -one);
                matrix.insert((idx, v_idx + w + 1), one);
                idx += 1;
            }
        }

        matrix
        */
    }

    fn derivative_0_dual_matrix(&self) -> SparseMatrix<T> {
        unimplemented!()
    }

    fn derivative_1_primal_matrix(&self) -> SparseMatrix<T> {
        unimplemented!()
    }
    fn derivative_1_dual_matrix(&self) -> SparseMatrix<T> {
        unimplemented!()
    }

    fn hodge_0_primal_matrix(&self) -> DiagonalMatrix<T> {
        unimplemented!()
    }
    fn hodge_1_primal_matrix(&self) -> DiagonalMatrix<T> {
        unimplemented!()
    }
    fn hodge_2_primal_matrix(&self) -> DiagonalMatrix<T> {
        unimplemented!()
    }

    fn hodge_0_dual_matrix(&self) -> DiagonalMatrix<T> {
        unimplemented!()
    }
    fn hodge_1_dual_matrix(&self) -> DiagonalMatrix<T> {
        unimplemented!()
    }
    fn hodge_2_dual_matrix(&self) -> DiagonalMatrix<T> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use ndarray::*;
    use super::*;

    #[test]
    fn grid_2d_divergence() {
        let grid = Grid2d::new((5, 5));
        let mut vel = <Grid2d as Manifold2d<f32>>::new_simplex_1(&grid);
        let velocities_y = &[
            [0.0, 0.0, 0.0, 0.0, 0.0],
            [0.0, 1.8, 1.8, 0.0, 0.0],
            [0.0, 2.0, 2.0, 0.0, 0.0],
            [0.0, 0.8, 0.8, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0, 0.0],
        ];

        let velocities_x = &[
            [0.0, 0.7, 0.0, -0.7, 0.0, 0.0],
            [0.0, 0.1, 0.0, -0.1, 0.0, 0.0],
            [0.0, -0.7, 0.0, 0.7, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        ];

        {
            let (mut vy, mut vx) = vel.split_mut();
            for ((y, x), v) in vy.indexed_iter_mut() {
                *v = velocities_y[y][x];
            }
            for ((y, x), v) in vx.indexed_iter_mut() {
                *v = velocities_x[y][x];
            }
        }

        let div_ref = [
            0.7, 1.1, 1.1, 0.7, 0.0,
            0.1, 0.1, 0.1, 0.1, 0.0,
            -0.7, -0.5, -0.5, -0.7, 0.0,
            0.0, -0.8, -0.8, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0,
        ];

        let mut vel_primal = <Grid2d as Manifold2d<f32>>::new_simplex_1(&grid);
        let mut divergence = <Grid2d as Manifold2d<f32>>::new_simplex_2(&grid);
        grid.hodge_1_dual(&mut vel_primal, &mut vel);
        grid.derivative_1_primal(&mut divergence, &vel_primal);

        let div = ArrayView::from_shape((5, 5), &div_ref).unwrap();
        let eps = 1.0e-3;

        let mut equal = true;
        for (&div, &reference) in divergence.iter().zip(div.iter()) {
            if !equal { break }
            equal = (div - reference).abs() < eps;
        }

        assert!(equal, "{:#?} approx eq {:#?} (eps = {:#?})", &divergence, &div, eps);
    }

    #[test]
    fn grid_2d_laplacian() {
        let grid = Grid2d::new((3, 3));

        let faces_primal = arr2(&[
            [-0.0, -3.0, -0.0],
            [-0.0, 2.0, 6.0],
            [1.0, -0.0, -0.0],
        ]);

        let mut faces_dual = <Grid2d as Manifold2d<f64>>::new_simplex_2(&grid);
        let mut edges_dual = <Grid2d as Manifold2d<f64>>::new_simplex_1(&grid);
        let mut edges_primal = <Grid2d as Manifold2d<f64>>::new_simplex_1(&grid);
        let mut laplacian = <Grid2d as Manifold2d<f64>>::new_simplex_2(&grid);

        grid.hodge_2_primal(&mut faces_dual, &faces_primal);
        grid.derivative_0_dual(&mut edges_dual, &faces_dual);
        grid.hodge_1_dual(&mut edges_primal, &edges_dual);
        grid.derivative_1_primal(&mut laplacian, &edges_primal);

        let laplacian_ref = [3.0, -11.0, -3.0, -3.0, 5.0, 16.0, 2.0, -3.0, -6.0];
        let laplac = ArrayView::from_shape((3, 3), &laplacian_ref).unwrap();
        let eps = 1.0e-3;

        let mut equal = true;
        for (&val, &reference) in laplacian.iter().zip(laplac.iter()) {
            if !equal { break }
            equal = (val - reference).abs() < eps;
        }

        assert!(equal, "{:#?} approx eq {:#?} (eps = {:#?})", &laplacian, &laplac, eps);
    }

    #[test]
    fn grid_2d_gradient() {
        let grid = Grid2d::new((3, 3));

        let faces_dual = arr2(&[
            [-0.0, -3.0, -0.0],
            [-0.0, 2.0, 6.0],
            [1.0, -0.0, -0.0],
        ]);

        let mut gradient = <Grid2d as Manifold2d<f64>>::new_simplex_1(&grid);

        grid.derivative_0_dual(&mut gradient, &faces_dual);

        let gradient_ref = [3.0, -11.0, -3.0, -3.0, 5.0, 16.0, 2.0, -3.0, -6.0];
        let grad = ArrayView::from_shape((3, 3), &gradient_ref).unwrap();
        let eps = 1.0e-3;
    }
}
