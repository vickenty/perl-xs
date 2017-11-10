
use perl_xs::context::Context;

xs! {
    package XSTest::Codegen;

    sub take_kv_struct(ctx, class: String) {
        let teststruct = TestStruct::from_kv_stack(&mut ctx, 1);
        format!("{:?}",teststruct)
    }
}

#[derive(FromKeyValueStack,Debug)]
struct TestStruct {
    alpha:          bool,
    beta:           Option<String>,
    #[perlxs(key = "-charlie")]
    charlie:        Option<String>,
    delta:          Option<bool>,
    #[perlxs(key = "_echo")]
    echo:          Option<String>,
}