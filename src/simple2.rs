use clap::Parser;
use halo2_base::gates::circuit::builder::BaseCircuitBuilder;
use halo2_base::gates::{GateChip, GateInstructions};
use halo2_base::utils::ScalarField;
use halo2_base::AssignedValue;
#[allow(unused_imports)]
use halo2_base::{
    Context,
    QuantumCell::{Constant, Existing, Witness},
};
use halo2_scaffold::scaffold::cmd::Cli;
use halo2_scaffold::scaffold::run;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CircuitInput{
    pub x: String, 
}

fn some_algorithm_in_zk<F: ScalarField>(
    builder: &mut BaseCircuitBuilder<F>,
    input: CircuitInput,
    make_public: &mut Vec<AssignedValue<F>>, 
){
    let x = F::from_str_vartime(&input.x).expect("deserialize field element should not fail");
    let ctx = builder.main(0);
    let x = ctx.load_witness(x);
    make_public.push(x);
    // Creating a Gate Chip
    let gate = GateChip::<F>::default();
    // HALO2-LIB API FUNCTIONS
    let x_sq = gate.mul(ctx, x, x);
    let constant = F::from(72);
    let out = gate.add(ctx, x_sq, Constant(c));
    make_public.push(out);

}

fn main() {
    env_logger::init();
    let args = Cli::parse();
    run(some_algorithm_in_zk, args);
}



