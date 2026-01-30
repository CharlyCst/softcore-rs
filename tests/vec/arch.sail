default Order dec
$include <prelude.sail>

/* TODO: This is not in the model, but they do use the result type */
$include <result.sail>

/* From model/prelude ------------------------------------------------------- */

$include <smt.sail>
$include <option.sail>
$include <arith.sail>
$include <string.sail>
$include <mapping.sail>
$include <vector_dec.sail>
$include <generic_equality.sail>
$include <hex_bits.sail>
$include <hex_bits_signed.sail>
$include <dec_bits.sail>

// not_bool alias.
val not : forall ('p : Bool). bool('p) -> bool(not('p))
function not(b) = not_bool(b)

infix 7 >>
infix 7 <<

val "shift_bits_right" : forall 'n 'm. (bits('n), bits('m)) -> bits('n)
val "shift_bits_left"  : forall 'n 'm. (bits('n), bits('m)) -> bits('n)

val "shiftl" : forall 'm 'n, 'n >= 0. (bits('m), int('n)) -> bits('m)
val "shiftr" : forall 'm 'n, 'n >= 0. (bits('m), int('n)) -> bits('m)

overload operator >> = {shift_bits_right, shiftr}
overload operator << = {shift_bits_left, shiftl}

/* The following defines % as euclidean modulus */
overload operator % = {emod_int}

val quot_positive_round_zero = pure {
    interpreter: "quot_round_zero",
    lem: "hardware_quot",
    lean: "Int.tdiv",
    c: "tdiv_int",
    coq: "Z.quot"
} : forall 'n 'm, 'n >= 0 & 'm > 0. (int('n), int('m)) -> int(div('n, 'm))

val quot_round_zero = pure {
    interpreter: "quot_round_zero",
    lem: "hardware_quot",
    c: "tdiv_int",
    coq: "Z.quot",
    lean: "Int.tdiv"
} : (int, int) -> int

overload operator / = {quot_positive_round_zero, quot_round_zero}
overload operator * = {mult_atom, mult_int}

val to_bits : forall 'l, 'l >= 0.(int('l), int) -> bits('l)
function to_bits (l, n) = get_slice_int(l, n, 0)

val zeros : forall 'n, 'n >= 0 . implicit('n) -> bits('n)
function zeros (n) = sail_zeros(n)

val ones : forall 'n, 'n >= 0 . implicit('n) -> bits('n)
function ones (n) = sail_ones(n)

val zero_extend : forall 'n 'm, 'm >= 'n. (implicit('m), bits('n)) -> bits('m)
function zero_extend(m, v) = sail_zero_extend(v, m)

overload operator | = {or_vec}
overload operator ^ = {xor_vec}

val sub_vec = pure {
    c: "sub_bits",
    lean: "_lean_sub",
    _: "sub_vec"
} : forall 'n. (bits('n), bits('n)) -> bits('n)

val sub_vec_int = pure {c: "sub_bits_int", lean: "BitVec.subInt", _: "sub_vec_int"} : forall 'n. (bits('n), int) -> bits('n)

overload operator - = {sub_vec, sub_vec_int}

/* From model/riscv_xlen64 -------------------------------------------------- */

type log2_xlen_bytes : Int = 3

/* From model/riscv_xlen ---------------------------------------------------- */

type log2_xlen  : Int = log2_xlen_bytes + 3
type xlen_bytes : Int = 2 ^ log2_xlen_bytes
type xlen       : Int = xlen_bytes * 8
type xlenbits         = bits(xlen)

/* From model/riscv_sys_regs ------------------------------------------------ */

bitfield Vtype : xlenbits = {
  vill      : xlen - 1,
  reserved  : xlen - 2 .. 8,
  vma       : 7,
  vta       : 6,
  vsew      : 5 .. 3,
  vlmul     : 2 .. 0
}
register vtype : Vtype

val get_sew_pow : unit -> {3, 4, 5, 6}
function get_sew_pow() = {
  let SEW_pow : {3, 4, 5, 6} = match vtype[vsew] {
    0b000 => 3,
    0b001 => 4,
    0b010 => 5,
    0b011 => 6,
    _     => {assert(false, "invalid vsew field in vtype"); 0}
  };
  SEW_pow
}

val get_sew : unit -> {8, 16, 32, 64}
function get_sew() = {
  match get_sew_pow() {
    3 => 8,
    4 => 16,
    5 => 32,
    6 => 64,
    _ => {assert(false, "invalid SEW"); 8}
  }
}

/* the vector register group multiplier (LMUL) */
/* this returns the power of 2 for LMUL */
val get_lmul_pow : unit -> {-3, -2, -1, 0, 1, 2, 3}
function get_lmul_pow() = {
  match vtype[vlmul] {
    0b101 => -3,
    0b110 => -2,
    0b111 => -1,
    0b000 => 0,
    0b001 => 1,
    0b010 => 2,
    0b011 => 3,
    _     => {assert(false, "invalid vlmul field in vtype"); 0}
  }
}

/* vector csrs */
register vstart : bits(16) /* use the largest possible length of vstart */
register vl     : xlenbits

enum agtype = { UNDISTURBED, AGNOSTIC }

val decode_agtype : bits(1) -> agtype
function decode_agtype(ag) = {
  match ag {
    0b0 => UNDISTURBED,
    0b1 => AGNOSTIC
  }
}

val get_vtype_vma : unit -> agtype
function get_vtype_vma() = decode_agtype(vtype[vma])

val get_vtype_vta : unit -> agtype
function get_vtype_vta() = decode_agtype(vtype[vta])

/* From model/riscv_vlen ---------------------------------------------------- */

/* function get_vlen_pow() -> range(3, 16) = config extensions.V.vlen_exp */
function get_vlen_pow() -> range(3, 16) = 9

type vlenmax : Int = 65536
let VLEN = 2 ^ get_vlen_pow()

/* From model/riscv_vreg_type ----------------------------------------------- */

type vreglenbits = bits(vlenmax) /* use the largest possible register length */
type vregtype = vreglenbits

enum vvfunct6 = {
    VV_VADD,
    VV_VSUB,
    VV_VAND,
    VV_VOR,
    VV_VXOR
}

/* From model/riscv_vext_regs ----------------------------------------------- */

newtype vregidx = Vregidx : bits(5)
newtype vregno = Vregno : range(0, 31)
function vregidx_to_vregno (Vregidx(b) : vregidx) -> vregno = Vregno(unsigned(b))
function vregno_to_vregidx (Vregno(b) : vregno) -> vregidx = Vregidx(to_bits(5, b))
function vregidx_offset(Vregidx(r) : vregidx, o : bits(5)) -> vregidx = Vregidx(r + o)
function vregidx_bits (Vregidx(b) : vregidx) -> bits(5) = b

let zvreg : vregidx = Vregidx(0b00000) /* v0, zero register  */

/* vector registers */
register vr0 : vregtype
register vr1 : vregtype
register vr2 : vregtype
register vr3 : vregtype
register vr4 : vregtype
register vr5 : vregtype
register vr6 : vregtype
register vr7 : vregtype
register vr8 : vregtype
register vr9 : vregtype
register vr10 : vregtype
register vr11 : vregtype
register vr12 : vregtype
register vr13 : vregtype
register vr14 : vregtype
register vr15 : vregtype
register vr16 : vregtype
register vr17 : vregtype
register vr18 : vregtype
register vr19 : vregtype
register vr20 : vregtype
register vr21 : vregtype
register vr22 : vregtype
register vr23 : vregtype
register vr24 : vregtype
register vr25 : vregtype
register vr26 : vregtype
register vr27 : vregtype
register vr28 : vregtype
register vr29 : vregtype
register vr30 : vregtype
register vr31 : vregtype

function rV (Vregno(r) : vregno) -> vregtype = {
  match r {
    0 => vr0,
    1 => vr1,
    2 => vr2,
    3 => vr3,
    4 => vr4,
    5 => vr5,
    6 => vr6,
    7 => vr7,
    8 => vr8,
    9 => vr9,
    10 => vr10,
    11 => vr11,
    12 => vr12,
    13 => vr13,
    14 => vr14,
    15 => vr15,
    16 => vr16,
    17 => vr17,
    18 => vr18,
    19 => vr19,
    20 => vr20,
    21 => vr21,
    22 => vr22,
    23 => vr23,
    24 => vr24,
    25 => vr25,
    26 => vr26,
    27 => vr27,
    28 => vr28,
    29 => vr29,
    30 => vr30,
    31 => vr31,
  }
}

function wV (Vregno(r) : vregno, v : vregtype) -> unit = {
  match r {
    0 => vr0 = v,
    1 => vr1 = v,
    2 => vr2 = v,
    3 => vr3 = v,
    4 => vr4 = v,
    5 => vr5 = v,
    6 => vr6 = v,
    7 => vr7 = v,
    8 => vr8 = v,
    9 => vr9 = v,
    10 => vr10 = v,
    11 => vr11 = v,
    12 => vr12 = v,
    13 => vr13 = v,
    14 => vr14 = v,
    15 => vr15 = v,
    16 => vr16 = v,
    17 => vr17 = v,
    18 => vr18 = v,
    19 => vr19 = v,
    20 => vr20 = v,
    21 => vr21 = v,
    22 => vr22 = v,
    23 => vr23 = v,
    24 => vr24 = v,
    25 => vr25 = v,
    26 => vr26 = v,
    27 => vr27 = v,
    28 => vr28 = v,
    29 => vr29 = v,
    30 => vr30 = v,
    31 => vr31 = v,
  };

  /* FIXME: Is the next function really important */
  /* dirty_v_context(); */

  assert(0 < VLEN & VLEN <= sizeof(vlenmax));

  /* FIXME: Is the next function really important */
  /* vreg_write_callback(vregno_to_vregidx(Vregno(r)), v); */
}

function rV_bits(i: vregidx) -> vregtype = rV(vregidx_to_vregno(i))

function wV_bits(i: vregidx, data: vregtype) -> unit = {
  wV(vregidx_to_vregno(i)) = data
}

overload V = {rV_bits, wV_bits, rV, wV}

/* Reads a single vreg into multiple elements */
val read_single_vreg : forall 'n 'm, 'n >= 0 & 'm >= 0. (int('n), int('m), vregidx) -> vector('n, bits('m))
function read_single_vreg(num_elem, SEW, vrid) = {
  let bv     : vregtype             = V(vrid);
  var result : vector('n, bits('m)) = vector_init(zeros());

  assert(8 <= SEW & SEW <= 64);
  foreach (i from 0 to (num_elem - 1)) {
    let start_index = i * SEW;
    result[i] = slice(bv, start_index, SEW);
  };

  result
}

/* Writes multiple elements into a single vreg */
val write_single_vreg : forall 'n 'm, 'n >= 0. (int('n), int('m), vregidx, vector('n, bits('m))) -> unit
function write_single_vreg(num_elem, SEW, vrid, v) = {
  var r : vregtype = zeros();

  assert(8 <= SEW & SEW <= 64);
  foreach (i from (num_elem - 1) downto 0) {
    r = r << SEW;
    r = r | zero_extend(v[i]);
  };

  V(vrid) = r
}

/* The general vreg reading operation with num_elem as max(VLMAX,VLEN/SEW)) */
val read_vreg : forall 'n 'm 'p, 'n >= 0 & 'm >= 0. (int('n), int('m), int('p), vregidx) -> vector('n, bits('m))
function read_vreg(num_elem, SEW, LMUL_pow, vrid) = {
  let vrid_val = unsigned(vregidx_bits(vrid));
  var result : vector('n, bits('m)) = vector_init(zeros());
  let LMUL_pow_reg = if LMUL_pow < 0 then 0 else LMUL_pow;

  /* Check for valid vrid */
  if vrid_val + 2 ^ LMUL_pow_reg > 32 then {
    /* vrid would read past largest vreg (v31) */
    assert(false, "invalid register group: vrid overflow the largest number")
  } else if vrid_val % (2 ^ LMUL_pow_reg) != 0 then {
    /* vrid must be a multiple of emul */
    assert(false, "invalid register group: vrid is not a multiple of EMUL")
  } else {
    if LMUL_pow < 0 then {
      result = read_single_vreg('n, SEW, vrid);
    } else {
      let 'num_elem_single : int = VLEN / SEW;
      assert('num_elem_single >= 0);
      foreach (i_lmul from 0 to (2 ^ LMUL_pow_reg - 1)) {
        let r_start_i : int = i_lmul * 'num_elem_single;
        let r_end_i   : int = r_start_i + 'num_elem_single - 1;
        let vrid_lmul     : vregidx = vregidx_offset(vrid, to_bits(5, i_lmul));
        let single_result : vector('num_elem_single, bits('m)) = read_single_vreg('num_elem_single, SEW, vrid_lmul);
        foreach (r_i from r_start_i to r_end_i) {
          let s_i : int = r_i - r_start_i;
          assert(0 <= r_i & r_i < num_elem);
          assert(0 <= s_i & s_i < 'num_elem_single);
          result[r_i] = single_result[s_i];
        }
      }
    }
  };

  result
}

/* The general vreg writing operation with num_elem as max(VLMAX,VLEN/SEW)) */
val write_vreg : forall 'n 'm 'p, 'n >= 0 & 'm >= 0. (int('n), int('m), int('p), vregidx, vector('n, bits('m))) -> unit
function write_vreg(num_elem, SEW, LMUL_pow, vrid, vec) = {
  let LMUL_pow_reg = if LMUL_pow < 0 then 0 else LMUL_pow;

  let 'num_elem_single  : int = VLEN / SEW;
  assert('num_elem_single >= 0);
  foreach (i_lmul from 0 to (2 ^ LMUL_pow_reg - 1)) {
    var single_vec : vector('num_elem_single, bits('m)) = vector_init(zeros());
    let vrid_lmul  : vregidx = vregidx_offset(vrid, to_bits(5, i_lmul));
    let r_start_i  : int = i_lmul * 'num_elem_single;
    let r_end_i    : int = r_start_i + 'num_elem_single - 1;
    foreach (r_i from r_start_i to r_end_i) {
      let s_i : int = r_i - r_start_i;
      assert(0 <= r_i & r_i < num_elem);
      assert(0 <= s_i & s_i < 'num_elem_single);
      single_vec[s_i] = vec[r_i]
    };
    write_single_vreg('num_elem_single, SEW, vrid_lmul, single_vec)
  }
}

/* num_elem means max(VLMAX,VLEN/SEW)) according to Section 5.4 of RVV spec */
val get_num_elem : (int, int) -> {'n, 'n > 0. int('n)}
function get_num_elem(LMUL_pow, SEW) = {
  let LMUL_pow_reg = if LMUL_pow < 0 then 0 else LMUL_pow;
  /* Ignore lmul < 1 so that the entire vreg is read, allowing all masking to
   * be handled in init_masked_result */
  let num_elem = 2 ^ LMUL_pow_reg * VLEN / SEW;
  assert(num_elem > 0);
  num_elem
}

/* Mask register reading operation with num_elem as max(VLMAX,VLEN/SEW)) */
val read_vmask : forall 'n, 'n > 0. (int('n), bits(1), vregidx) -> bits('n)
function read_vmask(num_elem, vm, vrid) = {
  assert(num_elem <= sizeof(vlenmax));
  let vreg_val : vregtype = V(vrid);
  var result   : bits('n) = ones();

  if vm == 0b1 then {
    return result
  };

  foreach (i from 0 to (num_elem - 1)) {
    result[i] = vreg_val[i]
  };

  result
}

/* From model/riscv_inst_retire --------------------------------------------- */

union ExecutionResult = {
  // Successfully retired.
  Retire_Success                 : unit,

  // WFI executed, which may or may not retire
  // depending on the configuration.
  // Wait_For_Interrupt             : unit,

  // Did not retire for a standard reason.
  Illegal_Instruction            : unit,
  // Trap                           : (Privilege, ctl_result, xlenbits),
  // Memory_Exception               : (virtaddr, ExceptionType),

  // Did not retire for custom reason.
  // Ext_CSR_Check_Failure          : unit,
  // Ext_ControlAddr_Check_Failure  : ext_control_addr_error,
  // Ext_DataAddr_Check_Failure     : ext_data_addr_error,
  // Ext_XRET_Priv_Failure          : unit,
}

/* From model/riscv_insts_begin --------------------------------------------- */

scattered union ast

val execute : ast -> ExecutionResult
scattered function execute

/* From model/riscv_insts_vext_utils ---------------------------------------- */

/* Check for valid vtype setting
 *  1. If the vill bit is set, then any attempt to execute a vector instruction that depends upon vtype will raise an illegal instruction exception.
 *  2. vset{i}vl{i} and whole-register loads, stores, and moves do not depend upon vtype.
 */
val valid_vtype : unit -> bool
function valid_vtype() = {
  vtype[vill] == 0b0
}

/* Check for valid destination register when vector masking is enabled:
 *  The destination vector register group for a masked vector instruction
 *  cannot overlap the source mask register (v0),
 *  unless the destination vector register is being written with a mask value (e.g., compares)
 *  or the scalar result of a reduction.
 */
val valid_rd_mask : (vregidx, bits(1)) -> bool
function valid_rd_mask(rd, vm) = {
  vm != 0b0 | rd != zvreg
}

/* a. Normal check including vtype.vill field and vd/v0 overlap if vm = 0 */
val illegal_normal : (vregidx, bits(1)) -> bool
function illegal_normal(vd, vm) = {
  not(valid_vtype()) | not(valid_rd_mask(vd, vm))
}

/* Get the starting element index from csr vtype */
val get_start_element : unit -> result(nat, unit)
function get_start_element() = {
  let start_element = unsigned(vstart);
  let VLEN_pow = get_vlen_pow();
  let SEW_pow = get_sew_pow();
  /* The use of vstart values greater than the largest element
   * index for the current SEW setting is reserved.
   *
   * TODO: the bound here might be incorrect.
   * See https://github.com/riscv/sail-riscv/pull/755#discussion_r2035095825
   */
  if start_element > (2 ^ (3 + VLEN_pow - SEW_pow) - 1)
  then Err(())
  else Ok(start_element)
}

/* Get the ending element index from csr vl */
val get_end_element : unit -> int
function get_end_element() = unsigned(vl) - 1

/* Mask handling; creates a pre-masked result vector for vstart, vl, vta/vma, and vm */
/* vm should be baked into vm_val from doing read_vmask */
/* tail masking when lmul < 1 is handled in write_vreg */
/* Returns two vectors:
 *   vector1 is the result vector with values applied to masked elements
 *   vector2 is a "mask" vector that is true for an element if the corresponding element
 *     in the result vector should be updated by the calling instruction
 */
val init_masked_result : forall 'n 'm 'p, 'n >= 0 & 'm >= 0. (int('n), int('m), int('p), vector('n, bits('m)), bits('n)) -> result((vector('n, bits('m)), bits('n)), unit)
function init_masked_result(num_elem, SEW, LMUL_pow, vd_val, vm_val) = {
  let start_element : nat = match get_start_element() {
    Ok(v)   => v,
    Err(()) => return Err(())
  };
  let end_element   = get_end_element();
  let tail_ag : agtype = get_vtype_vta();
  let mask_ag : agtype = get_vtype_vma();
  var mask : bits('n) = undefined;
  var result : vector('n, bits('m)) = undefined;

  /* Determine the actual number of elements when lmul < 1 */
  let real_num_elem = if LMUL_pow >= 0 then num_elem else num_elem / (2 ^ (0 - LMUL_pow));
  assert(num_elem >= real_num_elem);

  foreach (i from 0 to (num_elem - 1)) {
    if i < start_element then {
      /* Prestart elements defined by vstart */
      result[i] = vd_val[i];
      mask[i] = bitzero
    } else if i > end_element then {
      /* Tail elements defined by vl */
      result[i] = match tail_ag {
        UNDISTURBED => vd_val[i],
        AGNOSTIC    => vd_val[i] /* TODO: configuration support */
      };
      mask[i] = bitzero
    } else if i >= real_num_elem then {
      /* Tail elements defined by lmul < 1 */
      result[i] = match tail_ag {
        UNDISTURBED => vd_val[i],
        AGNOSTIC    => vd_val[i] /* TODO: configuration support */
      };
      mask[i] = bitzero
    } else if vm_val[i] == bitzero then {
      /* Inactive body elements defined by vm */
      result[i] = match mask_ag {
        UNDISTURBED => vd_val[i],
        AGNOSTIC    => vd_val[i] /* TODO: configuration support */
      };
      mask[i] = bitzero
    } else {
      /* Active body elements */
      mask[i] = bitone;
    }
  };

  Ok((result, mask))
}

/* From model/riscv_vext_control -------------------------------------------- */

function set_vstart(value : bits(16)) -> unit = {
  /* FIXME: Is the next function really important */
  /* dirty_v_context(); */
  let vstart_length = get_vlen_pow();
  vstart = zero_extend(value[(vstart_length - 1) .. 0]);
  /* csr_write_callback("vstart", zero_extend(vstart)); */
}

/* From model/riscv_insts_vext_arith ---------------------------------------- */

union clause ast = VVTYPE : (vvfunct6, bits(1), vregidx, vregidx, vregidx)

function clause execute(VVTYPE(funct6, vm, vs2, vs1, vd)) = {
  let SEW_pow  = get_sew_pow();
  let SEW      = get_sew();
  let LMUL_pow = get_lmul_pow();
  let VLEN_pow = get_vlen_pow();
  let num_elem = get_num_elem(LMUL_pow, SEW);

  if illegal_normal(vd, vm) then return Illegal_Instruction();

  let 'n = num_elem;
  let 'm = SEW;

  let vm_val  : bits('n) = read_vmask(num_elem, vm, zvreg);
  let vs1_val : vector('n, bits('m)) = read_vreg(num_elem, SEW, LMUL_pow, vs1);
  let vs2_val : vector('n, bits('m)) = read_vreg(num_elem, SEW, LMUL_pow, vs2);
  let vd_val  : vector('n, bits('m)) = read_vreg(num_elem, SEW, LMUL_pow, vd);

  let (initial_result, mask) : (vector('n, bits('m)), bits('n)) = match init_masked_result(num_elem, SEW, LMUL_pow, vd_val, vm_val) {
    Ok(v)   => v,
    Err(()) => return Illegal_Instruction()
  };
  var result = initial_result;

  foreach (i from 0 to (num_elem - 1)) {
    if mask[i] == bitone then {
      result[i] = match funct6 {
        VV_VADD          => vs2_val[i] + vs1_val[i],
        VV_VSUB          => vs2_val[i] - vs1_val[i],
        VV_VAND          => vs2_val[i] & vs1_val[i],
        VV_VOR           => vs2_val[i] | vs1_val[i],
        VV_VXOR          => vs2_val[i] ^ vs1_val[i],
      }
    }
  };

  write_vreg(num_elem, SEW, LMUL_pow, vd, result);
  set_vstart(zeros());
  Retire_Success()
}
