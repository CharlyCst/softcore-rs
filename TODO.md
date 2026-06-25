# TODO

- `rs_transform.ml`:
  + We should be updating the context definitions with each transformations
    instead of re-construction the function type context in `use_dynamic_bitvec_args`
- Do we really need to have `RsTypGeneric` and `RsTypGenericParam`?
- Is `RsTypOption` not the same thing as simply a `RsTypGenericParam`?
- Use references for most `BitDynamic` functions in `bitvector.rs`
- We should have type for methods application (A list of type * name ->
  `rs_fn_type`) and use methods for `bitvector_length`
- We should convert some externals built-ins to method application probably,
  such as `bitvector_length`
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
- Is the problem the fact that some types used to be in the context with empty
  body but now no longer are because when we update the context they are removed
- We should be able to specify which functions we want to export
- Updating the context continuously is highly inefficient
