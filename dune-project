(lang dune 3.0)

(using dune_site 0.1)

(wrapped_executables false)

(package
  (name sail_virt_backend)
  (synopsis "Sail virtualizability checker")
  (depends
    (libsail (= :version))
    (base64 (>= 3.1.0))))
