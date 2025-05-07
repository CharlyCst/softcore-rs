(lang dune 3.0)

(using dune_site 0.1)

(wrapped_executables false)

(package
  (name sail_rust_backend)
  (synopsis "Sail to Rust translation")
  (depends
    (libsail (= 0.19))))
