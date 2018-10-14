(load "examples/prelude.q")
(load "examples/qtest.q")

;;;;;;;;;; Tests

(defqtest "Truthful values in 'if'"
  (qt= (list
       (if (quote true) 1 2)
       (if true 1 2))
     '(1 1)))

(defqtest "Recursion"
  (qt=
    '(nil nil 5)
    (list
      (defn bar (exps)
        exps)

      (defn foo (exps)
        (if (list? exps)
          (if (not (empty? (rest exps)))
            (begin
              (bar (first exps))
              (foo (rest exps)))
            (bar (first exps)))
          (list)))

      (foo '(4 5))
      )
    ))

(qt-start)