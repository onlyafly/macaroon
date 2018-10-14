(def defn
  (macro (name args &rest exps)
    (list 'def name
      (list 'fn args
        (cons 'begin exps)))))

(defn list? (n)
  (= (typeof n) 'list))

(defn atom? (n)
  (not (list? n)))

(atom? '(1 2 3))