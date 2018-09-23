(def inc
  (let (add (fn (a b)
              (+ a b)))
    (fn (n)
      (add n 1))))

(def rebuild-inc
  (macro
    (fn (name)
      (list 'def name
        (list 'proc (routine-params inc)
          (routine-body inc))))))

(eval '(rebuild-inc inc2) (routine-environment inc))

(list
  (inc 3)
  (routine-params inc)
  (routine-body inc)
  (routine-environment inc)
  (eval '(inc2 5) (routine-environment inc)))
