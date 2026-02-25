default Order dec
$include <prelude.sail>

val zeros : forall 'n, 'n >= 0 . implicit('n) -> bits('n)
function zeros (n) = sail_zeros(n)

register rsize  : {8, 16, 32, 64} = 8
register rcount : {'n, 'n > 0. int('n)} = 1
register rv     : bits(256)

/* Reads a single vreg into multiple elements */
val read : forall 'n 'm, 'n >= 0 & 'm >= 0. (int('n), int('m)) -> vector('n, bits('m))
function read(elem_count, elem_size) = {
  var result : vector('n, bits('m)) = vector_init(zeros());

  foreach (i from 0 to (elem_count - 1)) {
    let start_index = i * elem_size;
    result[i] = slice(rv, start_index, elem_size);
  };

  result
}

val execute : unit -> unit
function execute() = {
  let elem_size  = rsize;
  let elem_count = rcount;

  let 'n = elem_count;
  let 'm = elem_size;

  let vs : vector('n, bits('m)) = read(elem_count, elem_size);
}
