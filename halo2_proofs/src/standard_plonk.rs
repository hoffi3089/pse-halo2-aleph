//! Simple standard Plonk circuit configuration. Helpful for deserializing verification keys.

use core::marker::PhantomData;
use ff::Field;

use crate::{
    circuit::{Layouter, SimpleFloorPlanner},
    plonk::{
        Advice, Circuit, Column, ConstraintSystem,
        Error,
        Fixed, Instance,
    },
    poly::Rotation,
};

/// Standard Plonk circuit configuration.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[allow(missing_docs)]
pub struct StandardPlonkConfig<Fr> {
    pub a: Column<Advice>,
    pub b: Column<Advice>,
    pub c: Column<Advice>,
    pub q_a: Column<Fixed>,
    pub q_b: Column<Fixed>,
    pub q_c: Column<Fixed>,
    pub q_ab: Column<Fixed>,
    pub constant: Column<Fixed>,
    pub instance: Column<Instance>,
    _phantom: PhantomData<Fr>,
}

impl<Fr: Field> StandardPlonkConfig<Fr> {
    fn configure(meta: &mut ConstraintSystem<Fr>) -> Self {
        let [a, b, c] = [(); 3].map(|_| meta.advice_column());
        let [q_a, q_b, q_c, q_ab, constant] = [(); 5].map(|_| {
            let col = meta.fixed_column();
            meta.enable_equality(col);
            col
        });
        let instance = meta.instance_column();

        [a, b, c].map(|column| meta.enable_equality(column));
        meta.enable_equality(instance);

        meta.create_gate(
            "q_a·a + q_b·b + q_c·c + q_ab·a·b + constant + instance = 0",
            |meta| {
                let [a, b, c] = [a, b, c].map(|column| meta.query_advice(column, Rotation::cur()));
                let [q_a, q_b, q_c, q_ab, constant] = [q_a, q_b, q_c, q_ab, constant]
                    .map(|column| meta.query_fixed(column, Rotation::cur()));
                let instance = meta.query_instance(instance, Rotation::cur());
                Some(
                    q_a * a.clone()
                        + q_b * b.clone()
                        + q_c * c
                        + q_ab * a * b
                        + constant
                        + instance,
                )
            },
        );

        StandardPlonkConfig {
            a,
            b,
            c,
            q_a,
            q_b,
            q_c,
            q_ab,
            constant,
            instance,
            _phantom: PhantomData,
        }
    }
}

/// Standard Plonk circuit. Warning: usable only for a configuration phase!
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct StandardPlonk;

impl<Fr: Field> Circuit<Fr> for StandardPlonk {
    type Config = StandardPlonkConfig<Fr>;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self::default()
    }

    fn configure(meta: &mut ConstraintSystem<Fr>) -> Self::Config {
        StandardPlonkConfig::configure(meta)
    }

    fn synthesize(
        &self,
        _: Self::Config,
        _: impl Layouter<Fr>,
    ) -> Result<(), Error> {
        unreachable!("`StandardPlonk` is intended only for a configuration purposes")
    }
}
