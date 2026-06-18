default Order dec
$include <prelude.sail>

// A simple architecture with vector register of fixed size, length and count of
// elements

// Utils -----------------------------------------------------------------------

val sub_vec = pure {
    c: "sub_bits",
    lean: "_lean_sub",
    _: "sub_vec"
} : forall 'n. (bits('n), bits('n)) -> bits('n)

val sub_vec_int = pure {c: "sub_bits_int", lean: "BitVec.subInt", _: "sub_vec_int"} : forall 'n. (bits('n), int) -> bits('n)

overload operator | = {or_vec}
overload operator ^ = {xor_vec}
overload operator - = {sub_vec, sub_vec_int}

val to_bits : forall 'l, 'l >= 0.(int('l), int) -> bits('l)
function to_bits (l, n) = get_slice_int(l, n, 0)

val zeros : forall 'n, 'n >= 0 . implicit('n) -> bits('n)
function zeros (n) = sail_zeros(n)

// TODO: Add some basic vector arithmetic and instructions to see if we can
// prove thing on the extracted model. We want things like just addition and
// stuff
// For now maybe we don't even need dependent types

// Registers -------------------------------------------------------------------


type V_LEN_ELEM = 2 ^ 16 // Size of each element in each vector register
type V_LEN_BITS = 16 // Number of element in each vector register
type vreglenbits = bits(V_LEN_BITS)
type vregtype = vector(V_LEN_ELEM, vreglenbits)

// register vr0 : vregtype
register vr1 : vregtype
register vr2 : vregtype
register vr3 : vregtype

newtype vregidx = Vregidx : bits(2)
newtype vregno = Vregno : range(0, 3)
function vregidx_to_vregno (Vregidx(b) : vregidx) -> vregno = Vregno(unsigned(b))
function vregno_to_vregidx (Vregno(b) : vregno) -> vregidx = Vregidx(to_bits(2, b))

function rV (Vregno(r) : vregno) -> vregtype = {
  match r {
    0 => vector_init(zeros()),
    1 => vr1,
    2 => vr2,
    3 => vr3,

  }
}

function wV (Vregno(r) : vregno, v : vregtype) -> unit = {
  match r {
    0 => (),
    1 => vr1 = v,
    2 => vr2 = v,
    3 => vr3 = v,
  };
}

enum vvop = {
    VV_VADD,
    VV_VSUB,
    VV_VAND,
    VV_VOR,
    VV_VXOR
}

/* ast */

scattered union ast
union clause ast = VVTYPE : (vvop, vregidx, vregidx, vregidx)

val execute : ast -> unit
function execute(VVTYPE(op, vs2, vs1, vd)) = {
    let vs1_val = rV(vregidx_to_vregno(vs1));
    let vs2_val = rV(vregidx_to_vregno(vs2));
    var vd_val: vregtype = vector_init(zeros());
    let len_elem = sizeof(V_LEN_ELEM);
    assert(0 <= len_elem & len_elem - 1 < sizeof(V_LEN_ELEM));
    foreach (i from 0 to (len_elem - 1)) {
        vd_val[i] = match op {
            VV_VADD => vs2_val[i] + vs1_val[i],
            VV_VSUB => vs2_val[i] - vs1_val[i],
            VV_VAND => vs2_val[i] & vs1_val[i],
            VV_VOR  => vs2_val[i] | vs1_val[i],
            VV_VXOR => vs2_val[i] ^ vs1_val[i],
        }
    };
    wV(vregidx_to_vregno(vd), vd_val);
}
