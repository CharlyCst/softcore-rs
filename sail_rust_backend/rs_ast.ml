module SSet = Set.Make (String)
module SMap = Map.Make (String)
module Big_int = Libsail.Ast_util.Big_int

type rs_type =
  | RsTypId of string
  | RsTypTuple of rs_type list
  | RsTypUnit
  | RsTypGeneric of string
  | RsTypGenericParam of string * rs_type_param list
  | RsTypArray of rs_type * rs_type_param
  | RsTypOption of
      rs_type_param (* TODO: This should just be a regular TypGenericParam ? *)
  | RsTypTodo of string
  | RsTypBorrow of rs_type

and rs_type_param =
  | RsTypParamTyp of rs_type
  | RsTypParamNum of rs_exp

and rs_generic =
  | RsGenTyp of string
  | RsGenConst of string * string (* variable name, generic type *)

and rs_lit =
  | RsLitUnit
  | RsLitTrue
  | RsLitFalse
  | RsLitNum of Big_int.num
  | RsLitBin of string
  | RsLitHex of string
  | RsLitStr of string
  | RsLitTodo

and rs_pat =
  | RsPatLit of rs_lit
  | RsPatId of string
  | RsPatType of rs_type * rs_pat
  | RsPatWildcard
  | RsPatTuple of rs_pat list
  | RsPatApp of rs_pat * rs_pat list
  | RsPatTodo of string
  | RsPatSome of rs_pat
  | RsPatNone

and rs_binop =
  | RsBinopEq
  | RsBinopNeq
  | RsBinopGt
  | RsBinopGe
  | RsBinopLt
  | RsBinopLe
  | RsBinopAnd
  | RsBinopOr
  | RsBinopXor
  | RsBinopLAnd
  | RsBinopLOr
  | RsBinopAdd
  | RsBinopSub
  | RsBinopMult
  | RsBinopDiv
  | RsBinopShiftLeft
  | RsBinopShiftRight
  | RsBinopMod

and rs_unop =
  | RsUnopNeg
  | RsUnopNot

and rs_method_app =
  { exp : rs_exp
  ; name : string
  ; generics : string list
  ; args : rs_exp list
  }

and rs_exp_aux =
  | RsLet of bool * rs_pat * rs_exp * rs_exp (* first bool is true if mutable *)
  | RsApp of rs_exp * string list * rs_exp list (* the strings are the generics *)
  | RsMethodApp of rs_method_app
  | RsStaticApp of rs_type * string * rs_exp list
  | RsId of string
  | RsLit of rs_lit
  | RsField of rs_exp * string
  | RsBlock of rs_exp list
  | RsConstBlock of rs_exp list
  | RsInstrList of rs_exp list
  | RsIf of rs_exp * rs_exp * rs_exp
  | RsMatch of rs_exp * rs_pexp list
  | RsTuple of rs_exp list
  | RsArray of rs_exp list
  | RsArraySize of rs_exp * rs_exp
  | RsVec of rs_exp list
  | RsVecSize of rs_exp * rs_exp
  | RsAssign of rs_lexp * rs_exp
  | RsIndex of rs_exp * rs_exp
  | RsBinop of rs_exp * rs_binop * rs_exp
  | RsUnop of rs_unop * rs_exp
  | RsAs of rs_exp * rs_type
  | RsSome of rs_exp
  | RsBorrow of rs_exp
  | RsNone
  | RsPathSeparator of rs_type * rs_type
  | RsFor of rs_type * rs_exp * rs_exp * rs_exp
  | RsForRev of rs_type * rs_exp * rs_exp * rs_exp
  | RsStruct of rs_type * (string * rs_exp) list
  | RsStructAssign of rs_exp * string * rs_exp
  | RsReturn of rs_exp
  | RsTodo of string

and rs_exp =
  { e_annot : rs_type option
  ; e_exp : rs_exp_aux
  }

and rs_lexp =
  | RsLexpId of string
  | RsLexpTyp of string * rs_type
  | RsLexpField of rs_exp * string
  | RsLexpIndex of rs_lexp * rs_exp
  | RsLexpIndexRange of rs_lexp * rs_exp * rs_exp
  | RsLexpBitVectorAccess of rs_lexp * rs_exp
  | RsLexpTodo

and rs_pexp =
  | RsPexp of rs_pat * rs_exp
  | RsPexpWhen of rs_pat * rs_exp * rs_exp

type rs_block = rs_exp list

type rs_fn_type =
  { generics : rs_generic list
  ; args : rs_type list
  ; ret : rs_type
  ; (* Sail passes type variables as standard arguments in some circumstances.
       In those cases, we need to relate the resulting generic variable with
       the corresponding argument.*)
    linked_gen_args : (int * int) list (* (generic_idx, arg_idx) *)
  }

type rs_fn =
  { name : string
  ; signature : rs_fn_type
  ; args : rs_pat list
  ; body : rs_exp
  ; const : bool
  ; doc : string list
  ; mutable use_sail_ctx : bool
  }

type rs_enum =
  { name : string
  ; generics : rs_generic list
  ; fields : (string * rs_type option) list
  ; derive : string list
  ; doc : string list
  }

type rs_struct =
  { name : string
  ; generics : rs_generic list
  ; fields : (string * rs_type) list
  ; derive : string list
  ; doc : string list
  }

type rs_alias =
  { new_typ : string
  ; generics : rs_generic list
  ; old_type : rs_type
  }

type rs_const =
  { name : string
  ; typ : rs_type
  ; value : rs_exp
  ; doc : string list
  }

type rs_obj =
  | RsFn of rs_fn
  | RsEnum of rs_enum
  | RsStruct of rs_struct
  | RsAlias of rs_alias
  | RsConst of rs_const
  | RsImport of string
  | RsAttribute of string
  | RsObjTodo of string

type rs_program = RsProg of rs_obj list

(* Built-in types ----------------------------------------------------------- *)

let rs_type_bool : rs_type = RsTypId "bool"
let rs_type_usize : rs_type = RsTypId "usize"
let rs_type_i128 : rs_type = RsTypId "i128"
let rs_type_i64 : rs_type = RsTypId "i64"
let rs_type_i32 : rs_type = RsTypId "i32"
let rs_type_u128 : rs_type = RsTypId "u128"
let rs_type_u64 : rs_type = RsTypId "u64"
let rs_type_u32 : rs_type = RsTypId "u32"
let rs_type_string : rs_type = RsTypId "String"

let rs_type_builtins : rs_type list =
  [ rs_type_bool
  ; rs_type_usize
  ; rs_type_i128
  ; rs_type_i64
  ; rs_type_i32
  ; rs_type_u128
  ; rs_type_u64
  ; rs_type_u32
  ; rs_type_string
  ]
;;

(* Prelude types ------------------------------------------------------------ *)

let rs_type_nat : rs_type = RsTypId "nat"
let rs_type_int : rs_type = RsTypId "i128"
let rs_type_bitdynamic : rs_type = RsTypId "BitDynamic"
let rs_type_core : rs_type = RsTypId "Core"

let rs_type_bitstatic (len : rs_type_param) : rs_type =
  RsTypGenericParam ("BitStatic", [ len ])
;;

let rs_type_boundedvec (typ : rs_type) (len : rs_type_param) : rs_type =
  RsTypGenericParam ("BoundedVec", [ RsTypParamTyp typ; len ])
;;
