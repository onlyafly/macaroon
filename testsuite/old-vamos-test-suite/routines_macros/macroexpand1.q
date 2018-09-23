(def defproc
  (macro
    (fn (name args body)
      (list 'def name
        (list 'proc args
          body)))))

(macroexpand1 '(defproc addem (a b) (+ a b)))
