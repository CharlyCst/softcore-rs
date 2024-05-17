#![allow(non_snake_case)]

// ———————————————————————————— Manually written ———————————————————————————— //

#[derive(Default, Clone, PartialEq, Eq, Debug)]
pub struct Ctx {
    Xs: [usize; 32],
}

pub fn mirage_itype(imm: usize, rs1: usize, rd: usize, ctx: &mut Ctx) {
    if rd == 0 {
        return;
    }
    ctx.Xs[rd] = ctx.Xs[rs1].wrapping_add(imm);
}

// ——————————————————————————————— Generated ———————————————————————————————— //

fn rX(r: usize, ctx: &mut Ctx) -> usize {
    match r {
        b__0 if b__0 == 0b00000 => 0b0000 as usize,
        _ => ctx.Xs[r],
    }
}

fn wX(r: usize, v: usize, ctx: &mut Ctx) {
    if r != 0b00000 {
        ctx.Xs[r] = v
    } else {
        ()
    }
}

pub fn ITYPE(imm: usize, rs1: usize, rd: usize, ctx: &mut Ctx) {
    let rs1_val = rX(rs1, ctx);
    let imm_ext: usize = imm as usize;
    let result = rs1_val.wrapping_add(imm_ext);
    wX(rd, result, ctx)
}

// —————————————————————————————————— Kani —————————————————————————————————— //

mod kani {
    use super::{mirage_itype, Ctx, ITYPE};

    fn any_ctx() -> Ctx {
        let mut ctx: Ctx = Default::default();
        for i in 1..32 {
            ctx.Xs[i] = kani::any();
        }
        ctx
    }

    #[kani::proof]
    fn itype() {
        let mut ctx = any_ctx();
        let imm = kani::any();
        kani::assume(imm < (1 << 12));
        let rs1 = kani::any();
        kani::assume(rs1 < 32);
        let rd = kani::any();
        kani::assume(rd < 32);

        let mut mctx = ctx.clone();
        ITYPE(imm, rs1, rd, &mut ctx);
        mirage_itype(imm, rs1, rd, &mut mctx);

        assert_eq!(ctx, mctx);
    }
}
