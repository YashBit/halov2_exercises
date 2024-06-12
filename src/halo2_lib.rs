// Initial Description Online

pub enum QuantumCell<F: ScalarField> {
    Existing(AssignedValue<F>),
    Witness(F),
    WitnessFraction(Assigned<F>),
    Constant(F),
}


fn mul(
    &self,
    ctx: &mut Context<F>,
    a: impl Into<QuantumCell<F>>,
    b: impl Into<QuantumCell<F>>,
) -> AssignedValue<F>


fn some_algorithm_in_zk<F: ScalarField>(builder: &mut BaseCircuitBuilder<F>, input:CircuitInput, make_public: &mut Vec<AssignedValue<F>>){
    let x = F::from_str_vartime(&input.x).expect("deserialize field element should not fail");
    // Built the circuit
    let ctx = builder.main(0);
    let x = ctx.load_witness(x);
    make_public.push(x);
    let gate = GateChip::<F>::default();
    let x_sq = gate.mul(ctx, x, x);
    let c = F::from(72);
    let out = gate.add(ctx, x_sq, Constant(c));
    make_public.push(out);
}


// Main
fn main(){
    env.logger::init();
    let args = Cli::parse();
    run(some_algorithm_in_zk, args);
}


