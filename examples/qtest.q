;;;;;;;;;;;;;;;;;;;;;;;;
; qtest test framework ;
;;;;;;;;;;;;;;;;;;;;;;;;

(def _qtest_tests '())

(defn _qtest_runtests (tests)
  (cond
    (= tests '()) nil
    else (let (test (first tests)
               othertests (rest tests)
               testname (first test)
               testfn (first (rest test))
               result (testfn))
           (begin
             (cond
               (= result true) (println ".")
               else (println "TEST FAILED: " testname))
             (_qtest_runtests othertests)))))

;;;;;;;;;; External API

;; (defqtest "Sample Test"
;;   pred1 pred2 predn...)
;; =>
;; (update! _qtest_tests
;;          (cons (list "Sample Test" (fn () (begin pred1 pred2 predn...)))
;;                _qtest_tests))
;;
(defmacro defqtest (name &rest preds)
  (list 'update! '_qtest_tests
    (list 'cons
      (list 'list name
        (list 'fn '()
          (cons 'begin preds)))
      '_qtest_tests)))

(defn qt= (actual expected)
  (if (= actual expected)
    true
    (begin
      (println "TEST FAILED. EXPECTED <" expected "> BUT GOT <" actual ">")
      false)))

(defn qtest-start ()
  (println "Running qtest tests...")
  (_qtest_runtests _qtest_tests)
  (println "Tests complete..."))