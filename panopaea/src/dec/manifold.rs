
use math::LinearView;
use sparse::{DiagonalMatrix, SparseMatrix};
use std::marker::PhantomData;

pub trait Manifold2d<T> : Hodge0<T> + Hodge1<T> + Hodge2<T> {
    /// Storage type for 0-simplices (vertex).
    /// A differential primal 0-form is stored for each 0-simplex.
    // type Simplex0: LinearView<Elem = T>;
    /// Storage type for 1-simplices (edge).
    /// A differential primal 1-form is stored for each 1-simplex.
    // type Simplex1: LinearView<Elem = T>;
    /// Storage type for 2-simplices (face).
    /// A differential primal 2-form is stored for each 2-simplex.
    // type Simplex2: LinearView<Elem = T>;

    ///
    fn num_elem_0(&self) -> usize;
    ///
    fn num_elem_1(&self) -> usize;
    ///
    fn num_elem_2(&self) -> usize;

    ///
    fn new_simplex_0(&self) -> Self::Simplex0;
    ///
    fn new_simplex_1(&self) -> Self::Simplex1;
    ///
    fn new_simplex_2(&self) -> Self::Simplex2;

    /// Discrete exterior derivative operator for primal 0-forms.
    ///
    /// The operator maps primal 0-forms to primal 1-forms.
    fn derivative_0_primal(&self, &mut Self::Simplex1, &Self::Simplex0);
    fn derivative_0_dual(&self, &mut Self::Simplex1, &Self::Simplex2);

    /// Discrete exterior derivative operator for primal 1-forms.
    ///
    /// The operator maps primal 1-forms to primal 2-forms.
    fn derivative_1_primal(&self, &mut Self::Simplex2, &Self::Simplex1);
    fn derivative_1_dual(&self, &mut Self::Simplex0, &Self::Simplex1);

    fn hodge_0_primal(&self, dual: &mut Self::Simplex0, primal: &Self::Simplex0) {
        Hodge0::apply(self, dual, primal)
    }
    fn hodge_2_dual(&self, primal: &mut Self::Simplex0, dual: &Self::Simplex0) {
        Hodge0::apply_inv(self, primal, dual)
    }

    fn hodge_1_primal(&self, dual: &mut Self::Simplex1, primal: &Self::Simplex1) {
        Hodge1::apply(self, dual, primal)
    }
    fn hodge_1_dual(&self, primal: &mut Self::Simplex1, dual: &Self::Simplex1) {
        Hodge1::apply_inv(self, primal, dual)
    }

    fn hodge_2_primal(&self, dual: &mut Self::Simplex2, primal: &Self::Simplex2) {
        Hodge2::apply(self, dual, primal)
    }
    fn hodge_0_dual(&self, primal: &mut Self::Simplex2, dual: &Self::Simplex2) {
        Hodge2::apply_inv(self, primal, dual)
    }

    //
    fn derivative_0_primal_matrix(&self) -> SparseMatrix<T>;
    fn derivative_0_dual_matrix(&self) -> SparseMatrix<T>;

    fn derivative_1_primal_matrix(&self) -> SparseMatrix<T>;
    fn derivative_1_dual_matrix(&self) -> SparseMatrix<T>;

    fn hodge_0_primal_matrix(&self) -> DiagonalMatrix<T>;
    fn hodge_1_primal_matrix(&self) -> DiagonalMatrix<T>;
    fn hodge_2_primal_matrix(&self) -> DiagonalMatrix<T>;

    fn hodge_0_dual_matrix(&self) -> DiagonalMatrix<T>;
    fn hodge_1_dual_matrix(&self) -> DiagonalMatrix<T>;
    fn hodge_2_dual_matrix(&self) -> DiagonalMatrix<T>;
}

pub trait Hodge0<T> {
    /// Storage type for 0-simplices (vertex).
    /// A differential primal 0-form is stored for each 0-simplex.
    type Simplex0;
    fn apply(&self, dual: &mut Self::Simplex0, primal: &Self::Simplex0);
    fn apply_inv(&self, primal: &mut Self::Simplex0, dual: &Self::Simplex0);
}

pub trait Hodge1<T> {
    /// Storage type for 1-simplices (edge).
    /// A differential primal 1-form is stored for each 1-simplex.
    type Simplex1;
    fn apply(&self, dual: &mut Self::Simplex1, primal: &Self::Simplex1);
    fn apply_inv(&self, primal: &mut Self::Simplex1, dual: &Self::Simplex1);
}

pub trait Hodge2<T> {
    /// Storage type for 2-simplices (face).
    /// A differential primal 2-form is stored for each 2-simplex.
    type Simplex2;
    fn apply(&self, dual: &mut Self::Simplex2, primal: &Self::Simplex2);
    fn apply_inv(&self, primal: &mut Self::Simplex2, dual: &Self::Simplex2);
}

pub struct Laplacian<'a, T, M: Manifold2d<T> + 'a> {
    pub manifold: &'a M,
    _marker: PhantomData<*const T>
}
