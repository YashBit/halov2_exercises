use std::marker::PhantomData;

use halo2_proofs::{
    circuit::{Layouter, SimpleFloorPlanner, Value},
    halo2curves::FieldExt,
    plonk::{Advice, Circuit, Column, ConstraintSystem, Error, Fixed},
    poly::Rotation,
};

#[derive(Clone, Copy)]
pub struct StandardPlonkConfig<F: FieldExt> {
    a: Column<Advice>,
    b: Column<Advice>,
    c: Column<Advice>,
    #[allow(dead_code)]
    q_a: Column<Fixed>,
    #[allow(dead_code)]
    q_b: Column<Fixed>,
    q_c: Column<Fixed>,
    q_ab: Column<Fixed>,
    constant: Column<Fixed>,
    _marker: PhantomData<F>,
}

impl<F: FieldExt> StandardPlonkConfig<F> {
    pub fn configure(meta: &mut ConstraintSystem<F>) -> Self {
        let [a, b, c] = [(); 3].map(|_| meta.advice_column());
        let [q_a, q_b, q_c, q_ab, constant] = [(); 5].map(|_| meta.fixed_column());
        [a, b, c].map(|column| meta.enable_equality(column));
        meta.create_gate("q_a·a + q_b·b + q_c·c + q_ab·a·b + constant = 0", |meta| {
            let [a, b, c] = [a, b, c].map(|column| meta.query_advice(column, Rotation::cur()));
            let [q_a, q_b, q_c, q_ab, constant] = [q_a, q_b, q_c, q_ab, constant]
                .map(|column| meta.query_fixed(column, Rotation::cur()));
            vec![q_a * a.clone() + q_b * b.clone() + q_c * c + q_ab * a * b + constant]
        });

        StandardPlonkConfig { a, b, c, q_a, q_b, q_c, q_ab, constant, _marker: PhantomData }
    }
}

#[derive(Clone, Default)]
pub struct StandardPlonk<F: FieldExt> {
    pub x: Value<F>,
}

impl<F: FieldExt> Circuit<F> for StandardPlonk<F> {
    type Config = StandardPlonkConfig<F>;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self::default()
    }

    fn configure(meta: &mut ConstraintSystem<F>) -> Self::Config {
        StandardPlonkConfig::configure(meta)
    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<F>,
    ) -> Result<(), Error> {
        layouter.assign_region(
            || "",
            |mut region| {
                let x = region.assign_advice(
                    || "add annotation if you want",
                    config.a,
                    0,
                    || self.x,
                )?;

                // square x
                // row 1: | x | x | x^2 | 0 | 0 | -1 | 1 | 0 |
                x.copy_advice(|| "", &mut region, config.a, 1)?;
                x.copy_advice(|| "", &mut region, config.b, 1)?;
                let val = x.value().map(|x| *x * x);
                region.assign_advice(|| "", config.c, 1, || val)?;
                region.assign_fixed(|| "", config.q_c, 1, || Value::known(-F::one()))?;
                region.assign_fixed(|| "", config.q_ab, 1, || Value::known(F::one()))?;
                let c = F::from(72);
                let val = x.value().map(|x| *x * x + c);
                x.copy_advice(|| "", &mut region, config.a, 2)?;
                x.copy_advice(|| "", &mut region, config.b, 2)?;
                region.assign_advice(|| "", config.c, 2, || val)?;
                region.assign_fixed(|| "", config.q_c, 2, || Value::known(-F::one()))?;
                region.assign_fixed(|| "", config.q_ab, 2, || Value::known(F::one()))?;
                region.assign_fixed(|| "", config.constant, 2, || Value::known(c))?;
                Ok(())
            },
        )
    }
}

#[cfg(test)]
mod test {
    use halo2_proofs::{
        arithmetic::Field, circuit::Value, dev::MockProver, halo2curves::bn256::Fr,
    };
    use rand::rngs::OsRng;
    use super::StandardPlonk;
    #[test]
    fn test_standard_plonk() {
        let k = 5;
        let circuit = StandardPlonk { x: Value::known(Fr::random(OsRng)) };
        MockProver::run(k, &circuit, vec![]).unwrap().assert_satisfied();
    }
}