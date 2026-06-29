# TODO

- `rs_transform.ml`:
  + We should be updating the context definitions with each transformations
    instead of re-construction the function type context in `use_dynamic_bitvec_args`
- Do we really need to have `RsTypGeneric` and `RsTypGenericParam`?
- Is `RsTypOption` not the same thing as simply a `RsTypGenericParam`?
- Use references for most `BitDynamic` functions in `bitvector.rs`
- We should have type for methods application (A list of type * name ->
  `rs_fn_type`) and use methods for `bitvector_length`, `bitvector_concat`
- We need a `concat` and a `concat_static`
- When we removed signature from the context, we introduce a lot of RsTypId
  "TodoNoSignature". We should instead add them to the context with an empty
  body I guess?
- Transformation should always be bottom->up to avoid the problem we run into
  with `.into()` which just run indefinitely?
  Or we should have two translations, `bottom_up` and `top_bottom` and then we
  can translate them once at a time to know which one require being `top_bottom`
- Why is it `arch_t` instead of `arch`
- Add a `RsTypMutBorrow` type instead of doing `RsTypId "&mut Core"`
- Add a `RsPatPathSeparator` for the `add_namespace` function
- We should be able to specify which functions we want to export and make public
- Updating the context continuously is highly inefficient, we should update it
  in the `rust_transform_func` probably
- Check in the compiler if we have constants bigger than `64`, in which case we
  still want to use `BitDynamic`
- I think some functions from `rv64/arch_prelude.rs` are unused
- We would later want to have type information not be optional
- Add a `bitand_static` for `BitDynamic` and `BitStatic`
- No cast in the `return` expression
- For now we are still being very dumb with cast everywhere. First make it work
  and then improve
- Instead of using the overloaded operators for our `BitVectors` in
  `rs_transform.ml`, we should simply try to see if the right side is a
  `BitStatic`. If yes, then we should use a `op_static`, and otherwise just use
  the `op` function. By having corresponding name in the prelude, it should be
  enough.
  A problem we have: We currently don't know the current type for everything, we
  only know the expected type (sometime).
- A way to fix our problem is just to say that `wrapped_add` the arguments
  should just be something which can be converted into a `BitDynamic`
