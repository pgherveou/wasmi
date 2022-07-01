;; Exports a function `count_until` that takes an input `n`.
;; The exported function counts an integer `n` times and then returns 0.
(module
  (func $count_until (export "count_until") (param $limit i32) (result i32)
    (local $counter i32)
    (block $exit
        (loop $loop
            (br_if ;; exit loop if $counter == $limit
                $exit
                (i32.eq
                    (local.tee $counter
                        (i32.add
                            (local.get $counter)
                            (i32.const 1)
                        )
                    )
                    (local.get $limit)
                )
            )
            (br $loop) ;; continue loop
        )
    )
    (return (local.get $counter))
  )
)
